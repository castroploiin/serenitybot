use serenity::model::prelude::{Message, Ready, ChannelId};
use serenity::{async_trait, prelude::*};
use serenitybot::commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Online");

        const GENERAL_CHAT: ChannelId = ChannelId(1110107351287087227);
        GENERAL_CHAT.say(&ctx.http, "I AM ALIVE").await.expect("WHY CAN'T I SEND A MESSAGE?");
}

async fn message(&self, ctx: Context, message: Message) {
        const PREFIX: &str = "?";

        if message.content.starts_with("?") {
            let command = &message.content.replace(PREFIX, "")[..];
            match command {
                "ping" => commands::pong(ctx.clone(), message.clone()).await,
                "embed" => commands::hello_embed(ctx.clone(), message.clone()).await,
                "some" => commands::get_input(ctx.clone(), message.clone()).await,
                "doc" => commands::get_help(ctx.clone(), message.clone()).await,
                _ => (),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Something happened while creating a client");

    if let Err(error) = client.start().await {
        println!("ERROR: {:?}", error)
    }
}