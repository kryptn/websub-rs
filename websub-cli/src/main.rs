use std::{fs::File, io::BufReader, path::Path};

use clap::{Parser, Subcommand};
use uuid::Uuid;
use websub::{Consumer, Message, Subscription, SubscriptionHandler, WebsubClient};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddSubscription {
        topic_url: String,
        hub_url: String,
    },
    GetSubscriptions,
    AddConsumer {
        config_file: String,
    },
    AddHandler {
        subscription_id: Uuid,
        consumer_id: Uuid,
    },
    Tell {
        consumer_name: String,
        message: String,
    },
}

fn read_consumer_config<P>(path: P) -> Result<Consumer, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = WebsubClient::default().await;

    match &cli.command {
        Commands::AddSubscription { topic_url, hub_url } => {
            let subscription = Subscription::new(topic_url.to_owned(), hub_url.to_owned());
            client.create_subscription(&subscription).await?;

            let out = serde_json::to_string(&subscription)?;
            println!("{}", out);
        }
        Commands::GetSubscriptions => {
            let subscriptions = client.get_subscriptions().await?;

            let out = serde_json::to_string(&subscriptions)?;
            println!("{}", out);
        }
        Commands::AddConsumer { config_file } => {
            let consumer = read_consumer_config(config_file)?;
            client.add_consumer(&consumer).await?;

            let out = serde_json::to_string(&consumer)?;
            println!("{}", out);
        }
        Commands::AddHandler {
            subscription_id,
            consumer_id,
        } => {
            let handler = SubscriptionHandler::new(*subscription_id, *consumer_id);
            client.add_handler(&handler).await?;

            let out = serde_json::to_string(&handler)?;
            println!("{}", out);
        }
        Commands::Tell {
            consumer_name,
            message,
        } => {
            let id = Uuid::new_v4().to_string();

            let message = Message::new(id, consumer_name.to_owned(), None, message.to_owned());
            client.put_message(&message).await?;

            let out = serde_json::to_string(&message)?;
            println!("{}", out);
        }
    }
    Ok(())
}
