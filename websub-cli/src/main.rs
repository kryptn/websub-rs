use clap::{Parser, Subcommand};
use websub::{Subscription, WebsubClient};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddSubscription { topic_url: String, hub_url: String },
    GetSubscriptions,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = WebsubClient::default().await;

    match &cli.command {
        Commands::AddSubscription { topic_url, hub_url } => {
            let subscription = Subscription::new(topic_url.to_owned(), hub_url.to_owned());
            client.create_subscription(&subscription).await?;
        }
        Commands::GetSubscriptions => {
            let subscriptions = client.get_subscriptions().await?;
            dbg!(subscriptions);
        }
    }
    Ok(())
}
