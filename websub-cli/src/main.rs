use clap::{Parser, Subcommand};
use websub::{dynamodb_client, IntoPutItem, Subscription, WebsubClient};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    AddSubscription {
        #[clap(value_parser)]
        topic_url: String,
        hub_url: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = WebsubClient::default().await;



    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::AddSubscription { topic_url, hub_url } => {
            let subscription = Subscription::new(topic_url.to_owned(), hub_url.to_owned());
            client.create_subscription(subscription).await?;

        }
    }
    Ok(())
}
