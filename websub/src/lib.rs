use aws_config::meta::region::RegionProviderChain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use aws_sdk_dynamodb::{
    client::{self, fluent_builders::PutItem},
    model::AttributeValue,
    Client, Region,
};

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

struct TableConfig {
    subscriptions: Option<String>,
    leases: Option<String>,
    callbacks: Option<String>,
    messages: Option<String>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            subscriptions: Some("subscriptions".to_string()),
            leases: Some("subscription_leases".to_string()),
            callbacks: Some("subscription_handlers".to_string()),
            messages: Some("messages".to_string()),
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

    pub async fn create_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let table = self
            .tables
            .subscriptions
            .clone()
            .expect("TableConfig.subscriptions must be Some");

        subscription
            .into_put_item(&self.client)
            .table_name(table)
            .send()
            .await?;

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
