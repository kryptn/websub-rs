use std::time::{Duration, SystemTime};

use aws_config::meta::region::RegionProviderChain;
use serde_derive::{Deserialize, Serialize};
use serde_dynamo::{from_item, from_items, to_item};
use uuid::Uuid;

use aws_sdk_dynamodb::{
    model::{AttributeValue, Select},
    Client, Region,
};

use eyre::Result;

#[cfg(feature = "ssm")]
pub mod ssm;


fn now() -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    now.as_secs()
}

fn offset(with: Duration) -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    (now + with).as_secs()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub topic_url: String,
    pub hub_url: String,
    pub subscribed_at: u64,
}

impl Subscription {
    pub fn new(topic_url: String, hub_url: String) -> Self {
        let id = uuid::Uuid::new_v4();
        Self::new_with_id(id, topic_url, hub_url)
    }

    fn new_with_id(id: Uuid, topic_url: String, hub_url: String) -> Self {
        let subscribed_at = now();
        Self {
            id,
            topic_url,
            hub_url,
            subscribed_at,
        }
    }

    pub fn renewed(&self) -> Self {
        Self::new_with_id(self.id, self.topic_url.clone(), self.hub_url.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddSubscription {
    pub id: Option<Uuid>,
    pub topic_url: String,
    pub hub_url: String,
}

impl From<AddSubscription> for Subscription {
    fn from(s: AddSubscription) -> Self {
        let mut sub = Subscription::new(s.topic_url, s.hub_url);
        sub.subscribed_at = now();
        if let Some(id) = s.id {
            sub.id = id;
        }

        sub
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionLease {
    pub subscription_id: Uuid,
    pub expiry: usize,
}

impl SubscriptionLease {
    pub fn new(subscription_id: Uuid, expiry: usize) -> Self {
        Self {
            subscription_id,
            expiry,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionHandler {
    pub subscription_id: Uuid,
    pub consumer_name: String,
    pub description: Option<String>,
}

impl SubscriptionHandler {
    pub fn new(subscription_id: Uuid, consumer_name: String, description: Option<String>) -> Self {
        Self {
            subscription_id,
            consumer_name,
            description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub consumer_name: String,
    pub body: String,
    pub subscription_id: Option<Uuid>,
    pub expiry: u64,
}

impl Message {
    pub fn new(
        id: String,
        consumer_name: String,
        subscription_id: Option<Uuid>,
        body: String,
    ) -> Self {
        Self {
            id,
            consumer_name,
            body,
            subscription_id,
            expiry: offset(Duration::from_secs(60 * 60)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlackIncomingWebhook {
    url: String,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Destination {
    SlackIncomingWebhook(SlackIncomingWebhook),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Consumer {
    pub name: String,
    pub destination: Destination,
}

impl Consumer {
    pub fn slack_url(&self) -> Option<String> {
        match &self.destination {
            Destination::SlackIncomingWebhook(d) => Some(d.url.clone()),
        }
    }
}

pub async fn dynamodb_client() -> Client {
    let region = "us-west-2";
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    Client::new(&shared_config)
}

#[derive(Clone)]
struct TableConfig {
    subscriptions: String,
    leases: String,
    handlers: String,
    messages: String,
    consumers: String,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            subscriptions: "subscriptions".to_string(),
            leases: "subscription_leases".to_string(),
            handlers: "subscription_handlers".to_string(),
            messages: "messages".to_string(),
            consumers: "consumers".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct WebsubClient {
    client: Client,
    tables: TableConfig,
}

impl WebsubClient {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            tables: TableConfig::default(),
        }
    }

    pub async fn default() -> Self {
        let client = dynamodb_client().await;
        Self::new(client)
    }

    pub async fn create_subscription(&self, subscription: &Subscription) -> Result<()> {
        let item = to_item(subscription)?;
        self.client
            .put_item()
            .table_name(&self.tables.subscriptions)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_subscriptions(&self) -> Result<Vec<Subscription>> {
        let result = self
            .client
            .scan()
            .table_name(&self.tables.subscriptions)
            .send()
            .await?;
        if let Some(items) = result.items {
            let subscriptions: Vec<Subscription> = from_items(items)?;
            Ok(subscriptions)
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_subscription_by_id(&self, id: Uuid) -> Result<Option<Subscription>> {
        let resp = self
            .client
            .query()
            .table_name(&self.tables.subscriptions)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", "id")
            .expression_attribute_values(":value", AttributeValue::S(id.to_string()))
            .select(Select::AllAttributes)
            .send()
            .await?;

        if let Some(items) = resp.items {
            if !items.is_empty() {
                let item = items[0].clone();
                let subscription = from_item(item)?;
                return Ok(Some(subscription));
            }
        }
        Ok(None)
    }

    pub async fn create_lease(&self, lease: &SubscriptionLease) -> Result<()> {
        let item = to_item(lease)?;
        self.client
            .put_item()
            .table_name(&self.tables.leases)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_handler(&self, handler: &SubscriptionHandler) -> Result<()> {
        // todo: verify that the subscription and handler exist
        let item = to_item(handler)?;
        self.client
            .put_item()
            .table_name(&self.tables.handlers)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_handlers_for_subscription(
        &self,
        subscription_id: Uuid,
    ) -> Result<Vec<SubscriptionHandler>> {
        let resp = self
            .client
            .query()
            .table_name(&self.tables.handlers)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", "subscription_id")
            .expression_attribute_values(":value", AttributeValue::S(subscription_id.to_string()))
            .select(Select::AllAttributes)
            .send()
            .await?;

        if let Some(items) = resp.items {
            let handlers = from_items(items)?;
            Ok(handlers)
        } else {
            Ok(vec![])
        }
    }

    pub async fn put_message(&self, message: &Message) -> Result<()> {
        let item = to_item(message)?;
        self.client
            .put_item()
            .table_name(&self.tables.messages)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_consumer(&self, consumer: &Consumer) -> Result<()> {
        let item = to_item(consumer)?;
        self.client
            .put_item()
            .table_name(&self.tables.consumers)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_consumer(&self, name: &str) -> Result<Option<Consumer>> {
        let resp = self
            .client
            .query()
            .table_name(&self.tables.consumers)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", "name")
            .expression_attribute_values(":value", AttributeValue::S(name.to_owned()))
            .select(Select::AllAttributes)
            .send()
            .await?;

        // i'm certain there's a cleaner way to do this with combinators.
        if let Some(items) = resp.items {
            let consumers: Vec<Consumer> = from_items(items)?;
            Ok(Some(consumers[0].clone()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
