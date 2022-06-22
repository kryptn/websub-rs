use aws_config::meta::region::RegionProviderChain;
use serde_derive::{Deserialize, Serialize};
use serde_dynamo::{from_items, to_item};
use uuid::Uuid;

use aws_sdk_dynamodb::{
    model::{AttributeValue, Select},
    Client, Region,
};

use eyre::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub topic_url: String,
    pub hub_url: String,
}

impl Subscription {
    pub fn new(topic_url: String, hub_url: String) -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            id,
            topic_url,
            hub_url,
        }
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
        if let Some(id) = s.id {
            sub.id = id;
        }

        sub
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionLease {
    pub subscription_id: Uuid,
    pub expiry: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionHandler {
    pub subscription_id: Uuid,
    pub handler: String,
    //pub expiry: isize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub body: String,
    pub expiry: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeCommand {
    pub subscription_id: Option<Uuid>,
    pub topic_url: String,
    pub hub_url: String,
    pub lease_seconds: isize,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HandlerKind {
    Slack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddHandlerCommand {
    pub subscription_id: Option<Uuid>,
    pub handler_kind: HandlerKind,
}

pub async fn dynamodb_client() -> Client {
    let region = "us-west-2";
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    Client::new(&shared_config)
}

#[allow(dead_code)]
struct TableConfig {
    subscriptions: String,
    leases: String,
    handlers: String,
    messages: String,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            subscriptions: "subscriptions".to_string(),
            leases: "subscription_leases".to_string(),
            handlers: "subscription_handlers".to_string(),
            messages: "messages".to_string(),
        }
    }
}

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

    pub async fn create_lease(&self, lease: SubscriptionLease) -> Result<()> {
        let item = to_item(lease)?;
        self.client
            .put_item()
            .table_name(&self.tables.leases)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_handler(&self, handler: SubscriptionHandler) -> Result<()> {
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

    pub async fn put_messages_for_callback(&self) -> Result<()> {
        Ok(())
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
