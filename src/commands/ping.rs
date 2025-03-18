use crate::api::safe_say;
use serenity::all::{Context, Message};
use std::sync::Arc;

/// Implement command: Ping
/// To test if the bot is working
pub async fn ping(context: Arc<Context>, message: Arc<Message>) {
    safe_say(context, message, "Pong!").await;
}
