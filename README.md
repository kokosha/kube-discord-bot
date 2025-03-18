# Kube Discord Bot

Kube is a lightweight Discord bot built using [serenity-rs](https://github.com/serenity-rs/serenity). The Discord bot is designed to help you set reminders so that you never miss an important event.

## Features

- **Help System:** Check the commands available from the bot.
- **Ping System:** Check if the bot is working.
- **Reminder System:** Easily create reminders for your events.
- **Private Chat (DM) Deletion System:** Delete the bot messages from private chat (DM), when they are no longer needed.

## Future planned features

- **Improved Reminder System:** Enhanced reminder functionality with support for hour, day, weeks and month and also with direct date input.
- **Chat Moderation System:** Creation of chat or voice room on demand, detection of bad words in messages and displaying warning to users.
- **Additional System:** As the time passes and new ideas emerge, more features will be added.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.
- A Discord account and a bot token, which can be obtained from [Discord Developer Portal](https://discord.com/developers/applications).

## Installation

### 1. **Clone the Repository:**
```bash
git clone https://github.com/kokosha/kube-discord-bot.git
cd kube-discord-bot
```

### 2. **Set up .env:**
Create a `.env` file in the root directory and add your Discord token:
```env
DISCORD_TOKEN=YOUR_DISCORD_BOT_TOKEN
```

### 3. **Run Kube:**
```bash
cargo run
```