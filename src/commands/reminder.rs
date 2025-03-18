use std::sync::Arc;

use serenity::all::{Context, Message};
use tokio::time::{Duration, sleep};
use tracing::debug;

use crate::api::{safe_reply, safe_reply_ping, safe_say};

/// Implement command: Reminder
/// Create the reminder to do something after some time
pub async fn reminder(context: Arc<Context>, message: Arc<Message>) {
    let Some(args) = extract_args(message.clone()).await else {
        safe_reply(context, message, "Usage: !reminder <seconds> <message>").await;
        return;
    };

    let Some(seconds) = parse_seconds(context.clone(), message.clone(), &args).await else {
        return;
    };

    handle_timer(context.clone(), message.clone(), seconds).await;
}

async fn extract_args(message: Arc<Message>) -> Option<Vec<String>> {
    let mut parts = message.content.split_whitespace();
    assert!(parts.next().is_some());

    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    Some(args)
}

async fn parse_seconds(
    context: Arc<Context>,
    message: Arc<Message>,
    args: &Vec<String>,
) -> Option<u64> {
    if let Some(arg) = args.get(0) {
        match arg.parse::<u64>() {
            Ok(sec) => Some(sec),
            Err(_) => {
                safe_reply(context, message, "Invalid number of seconds.").await;
                None
            }
        }
    } else {
        safe_reply(context, message, "Missing seconds argument.").await;
        None
    }
}

async fn handle_timer(context: Arc<Context>, message: Arc<Message>, seconds: u64) {
    debug!("Setting timer");
    safe_say(
        context.clone(),
        message.clone(),
        format!("Timer set for {} seconds!", seconds),
    )
    .await;
    debug!("Using tokio sleep");
    sleep(Duration::from_secs(seconds)).await;
    debug!("Finish timer");
    safe_reply_ping(context.clone(), message.clone(), "Time's up!").await;
}
