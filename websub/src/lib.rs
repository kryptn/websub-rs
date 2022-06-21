use std::collections::HashMap;

use aws_config::meta::region::RegionProviderChain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use aws_sdk_dynamodb::{
    client::fluent_builders::PutItem,
    model::{AttributeValue, Select},
    Client, Region,
};

use eyre::Result;

pub trait IntoPutItem {
    fn into_put_item(&self, client: &Client) -> PutItem;
}

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

impl IntoPutItem for Subscription {
    fn into_put_item(&self, client: &Client) -> PutItem {
        let id = AttributeValue::S(self.id.to_string());
        let topic_url = AttributeValue::S(self.topic_url.clone());
        let hub_url = AttributeValue::S(self.hub_url.clone());

        client
            .put_item()
            .item("Id", id)
            .item("TopicUrl", topic_url)
            .item("HubUrl", hub_url)
    }
}

struct RecordHelper(HashMap<String, AttributeValue>);

impl RecordHelper {
    fn get_as_uuid(&self, key: &str) -> Result<Uuid> {
        let value = self.0.get(key).expect("this key should be known");
        let value = value.as_s().expect("any uuid is stored as string");
        let value = Uuid::parse_str(value)?;

        Ok(value)
    }

    fn get_as_string(&self, key: &str) -> Result<String> {
        let value = self.0.get(key).expect("this key should be known");
        let value = value.as_s().expect("everythings a string");
        Ok(value.to_owned())
    }
}

impl From<HashMap<String, AttributeValue>> for Subscription {
    fn from(record: HashMap<String, AttributeValue>) -> Self {
        let record = RecordHelper(record);

        let id = record.get_as_uuid("Id").expect("this is defined");
        let topic_url = record.get_as_string("TopicUrl").expect("this is defined");
        let hub_url = record.get_as_string("HubUrl").expect("this is defined");

        Self {
            id,
            topic_url,
            hub_url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionLease {
    pub subscription_id: Uuid,
    pub expiry: isize,
}

impl IntoPutItem for SubscriptionLease {
    fn into_put_item(&self, client: &Client) -> PutItem {
        let subscription_id = AttributeValue::S(self.subscription_id.to_string());
        let expiry = AttributeValue::N(self.expiry.to_string());

        client
            .put_item()
            .item("SubscriptionId", subscription_id)
            .item("Expiry", expiry)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionHandler {
    pub subscription_id: Uuid,
    pub handler: String,
    //pub expiry: isize
}

impl IntoPutItem for SubscriptionHandler {
    fn into_put_item(&self, client: &Client) -> PutItem {
        let subscription_id = AttributeValue::S(self.subscription_id.to_string());
        let handler = AttributeValue::N(self.handler.clone());

        client
            .put_item()
            .item("SubscriptionId", subscription_id)
            .item("Handler", handler)
    }
}

impl From<HashMap<String, AttributeValue>> for SubscriptionHandler {
    fn from(record: HashMap<String, AttributeValue>) -> Self {
        let subscription_id = record
            .get("SubscriptionId")
            .map(|id| id.as_s().expect("known"))
            .map(|id| Uuid::parse_str(id).expect("known"))
            .expect("required key");

        let handler = record
            .get("Handler")
            .map(|h| h.as_s().expect("known"))
            .expect("required")
            .to_owned();

        Self {
            subscription_id,
            handler,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub body: String,
    pub expiry: isize,
}

impl IntoPutItem for Message {
    fn into_put_item(&self, client: &Client) -> PutItem {
        let id = AttributeValue::S(self.id.to_string());
        let subscription_id = AttributeValue::S(self.subscription_id.to_string());
        let body = AttributeValue::S(self.body.clone());
        let expiry = AttributeValue::S(self.expiry.to_string());

        client
            .put_item()
            .item("Id", id)
            .item("Subscription Id", subscription_id)
            .item("Body", body)
            .item("Expiry", expiry)
    }
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

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<()> {
        subscription
            .into_put_item(&self.client)
            .table_name(self.tables.subscriptions.clone())
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_subscriptions(&self) -> Result<Vec<Subscription>> {
        Ok(self
            .client
            .scan()
            .table_name(self.tables.subscriptions.clone())
            .select(Select::AllAttributes)
            .send()
            .await?
            .items()
            .unwrap()
            .iter()
            .map(|record| record.clone().into())
            .collect())
    }

    pub async fn create_lease(&self, lease: SubscriptionLease) -> Result<()> {
        lease
            .into_put_item(&self.client)
            .table_name(self.tables.leases.clone())
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_handler(&self, handler: SubscriptionHandler) -> Result<()> {
        handler
            .into_put_item(&self.client)
            .table_name(self.tables.handlers.clone())
            .send()
            .await?;

        Ok(())
    }

    // pub async fn get_handlers_for_subscription(
    //     &self,
    //     subscription_id: Uuid,
    // ) -> Result<Vec<SubscriptionHandler>> {
    //     let resp = self
    //         .client
    //         .query()
    //         .table_name(self.tables.handlers.clone())
    //         .key_condition_expression("#key = :value")
    //         .expression_attribute_names("#key", "SubscriptionId")
    //         .expression_attribute_values(":value", AttributeValue::S(subscription_id.to_string()))
    //         .select(Select::AllAttributes)
    //         .send()
    //         .await?;

    //     let handlers = Vec::new();

    //     Ok(handlers)
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
