use std::sync::Arc;

use serenity::all::{Context, Message};
use tracing::{info, warn};

pub(crate) async fn safe_say(
    context: Arc<Context>,
    message: Arc<Message>,
    content: impl AsRef<str>,
) {
    info!("{}", content.as_ref());

    if let Err(err) = message
        .channel_id
        .say(context.clone(), content.as_ref())
        .await
    {
        warn!("Error sending message with safe_say: {:?}", err);
    }
}

pub(crate) async fn safe_reply(
    context: Arc<Context>,
    message: Arc<Message>,
    content: impl AsRef<str>,
) {
    info!("{}", content.as_ref());
    if let Err(err) = message.reply(context.clone(), content.as_ref()).await {
        warn!("Error sending message with safe_reply: {:?}", err);
        warn!("Sending using safe_say");
        safe_say(context, message, content).await;
    }
}

pub(crate) async fn safe_reply_ping(
    context: Arc<Context>,
    message: Arc<Message>,
    content: impl AsRef<str>,
) {
    info!("{}", content.as_ref());
    if let Err(err) = message.reply_ping(context.clone(), content.as_ref()).await {
        warn!("Error sending message with safe_reply_ping: {:?}", err);
        warn!("Sending using safe_say");
        safe_say(context, message, content).await;
    }
}
