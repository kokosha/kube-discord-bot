use crate::api::safe_say;
use serenity::all::{Context, Message, MessageBuilder};
use std::sync::Arc;

/// Implement command: Help
/// Show the commands available!
pub async fn help(context: Arc<Context>, message: Arc<Message>) {
    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&message.author.name)
        .push(" the available commands in the bot is:\n")
        .push_bold_safe("- !deletedm\n- !help\n- !ping\n- !reminder <seconds> <message>\n")
        .build();
    safe_say(context, message, response).await;
}
