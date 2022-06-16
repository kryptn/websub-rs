use aws_config::meta::region::RegionProviderChain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use aws_sdk_dynamodb::{Client, client, Region};



trait IntoPutItem {
    type Object;
}


#[derive(Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub topic_url: String,
    pub hub_url: String,
}


impl Into<PutItem> for PutSubscription {

}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionLease {
    pub subscription_id: Uuid,
    pub expiry: isize,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionHandler {
    pub subscription_id: Uuid,
    pub handler: String,
    //pub expiry: isize
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub body: String,
    pub expiry: isize,
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeCommand {
    pub subscription_id: Option<Uuid>,
    pub topic_url: String,
    pub hub_url: String,
    pub lease_seconds: isize,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum HandlerKind {
    Slack,
}

#[derive(Serialize, Deserialize)]
pub struct AddHandlerCommand {
    pub subscription_id: Option<Uuid>,
    pub handler_kind: HandlerKind,
}

async fn thing() {
    let region = "us-west-2";
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
