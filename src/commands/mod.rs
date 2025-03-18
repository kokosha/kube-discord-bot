use std::sync::Arc;

use serenity::all::{Context, Message};

pub mod help;
pub use help::help;

pub mod ping;
pub use ping::ping;

pub mod reminder;
pub use reminder::reminder;

pub mod delete;
pub use delete::delete_dm;

pub async fn commands(context: Arc<Context>, message: Arc<Message>) {
    tokio::spawn(async move {
        if message.content == "!ping" {
            ping(context.clone(), message.clone()).await;
        } else if message.content == "!deletedm" {
            delete_dm(context.clone(), message.clone()).await;
        } else if message.content == "!help" {
            help(context.clone(), message.clone()).await;
        } else if message.content.starts_with("!reminder") {
            reminder(context.clone(), message.clone()).await;
        }
    });
}
