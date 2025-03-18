use serenity::{
    all::{Channel, ChannelId, Context, GetMessages, Message},
    futures::future::join_all,
};
use std::sync::Arc;
use tracing::warn;

use crate::api::safe_say;

/// Implement command: Delete DM
/// Delete the bot messages from DM
pub async fn delete_dm(context: Arc<Context>, message: Arc<Message>) {
    let channel = message.channel_id.to_channel(&context.http).await.unwrap();
    if matches!(channel, Channel::Private(_)) {
        delete_all_dm_messages(context, message.channel_id).await;
    } else {
        safe_say(context, message, "This is not DM!").await;
    }
}

async fn delete_all_dm_messages(context: Arc<Context>, dm_channel_id: ChannelId) {
    let mut last_message_id = None;

    loop {
        let builder = last_message_id
            .map(|id| GetMessages::new().limit(100).before(id))
            .unwrap_or_else(|| GetMessages::new().limit(100));

        let messages = dm_channel_id
            .messages(&context.http, builder)
            .await
            .unwrap();

        if messages.is_empty() {
            break;
        }
        let current_user = context.http.get_current_user().await.unwrap();

        let messages_to_delete: Vec<_> = messages
            .clone()
            .into_iter()
            .filter(|message| message.author.id == current_user.id)
            .collect();

        for chunk in messages_to_delete.chunks(5) {
            let deletion_futures = chunk.iter().map(|message| async {
                match message.delete(&context.http).await {
                    Ok(_) => {}
                    Err(err) => warn!("Error deleting message {}: {:?}", message.id, err),
                }
            });
            join_all(deletion_futures).await;

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        last_message_id = messages.last().map(|message| message.id);
    }
}
