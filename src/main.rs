use serenity::model::prelude::{Message, Ready};

use serenity::{async_trait, prelude::*};
use serenitybot::commands;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, _: Ready) {
        println!("Online");
}

async fn message(&self, ctx: Context, message: Message) {
        const PREFIX: &str = "?";

        let command = &message.content.replace(PREFIX, "")[..];
        match command {
            "ping" => commands::pong(ctx.clone(), message.clone()).await,
            "embed" => commands::hello_embed(ctx.clone(), message.clone()).await,
            "some" => commands::get_input(ctx.clone(), message.clone()).await,
            _ => (),
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
