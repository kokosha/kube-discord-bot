use std::{env, sync::Arc};

use dotenv::dotenv;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::token::validate;
use tracing::info;

mod api;
mod commands;
mod shard;
use commands::commands;
use shard::{spawn_latency_metrics, spawn_shard_manager_shutdown};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        use serenity::gateway::ActivityData;
        use serenity::model::user::OnlineStatus;
        if let Some(shard) = ready.shard {
            info!(
                "{} is connected on shard {} from the total of {} shards!",
                ready.user.name, shard.id, shard.total
            );
            let activity = ActivityData::listening(format!("you"));
            let status = OnlineStatus::Online;
            context.set_presence(Some(activity), status);
        }
    }

    async fn message(&self, context: Context, message: Message) {
        let context = Arc::new(context).clone();
        let message = Arc::new(message).clone();

        commands(context, message).await;
    }
}

#[tokio::main]
async fn main() {
    // Activate tracing
    tracing_subscriber::fmt::init();

    // Check for the DISCORD_TOKEN=XXXXXXXX inside .env and get token
    dotenv().expect("Something is wrong with .env");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    assert!(validate(&token).is_ok());

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone();
    spawn_latency_metrics(shard_manager.clone(), 1800).await;
    spawn_shard_manager_shutdown(shard_manager.clone()).await;

    // Start listening for events by starting on multiple shard
    if let Err(err) = client.start_shards(4).await {
        println!("Client error: {err:?}");
    }
}
