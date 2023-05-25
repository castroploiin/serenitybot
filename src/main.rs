use serenity::model::prelude::{ChannelId, Message, Ready};
use serenity::utils::MessageBuilder;
use serenity::{async_trait, prelude::*};
use serenitybot::commands;

const PREFIX: &str = "?";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
        let general_chat = ChannelId(1110107351287087227);
        let message = MessageBuilder::new().push("Hello World.").build();
        general_chat
            .say(&ctx.http, message)
            .await
            .expect("Could not send message");
    }

    async fn message(&self, ctx: Context, message: Message) {
        for command in message.content.split(PREFIX) {
            match command {
                "ping" => commands::pong(ctx.clone(), message.clone()).await,
                "embed" => commands::hello_embed(ctx.clone(), message.clone()).await,
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
