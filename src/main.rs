use serenity::model::prelude::{Member, Message, Ready};
use serenity::{async_trait, prelude::*};
use serenitybot::commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, mut new_member: Member) {
        new_member
            .add_role(ctx, 1126217605976436777)
            .await
            .expect(stringify!(
                "Could not add role to member with ID: {}",
                new_member.ID
            ));
    }

    async fn message(&self, ctx: Context, message: Message) {
        const PREFIX: &str = "&";

        if message.content.starts_with(PREFIX) {
            let command = &*message.content.replace(PREFIX, "");
            match command {
                // "help" => commands::my_help(ctx, message).await,
                "ping" => commands::pong(&ctx, &message).await,
                "embed" => commands::hello_embed(&ctx, &message).await,
                "some" => commands::get_input(&ctx, &message).await,
                "doc" => commands::get_help(&ctx, &message).await,
                "8ball" => commands::eightball(&ctx, &message).await,
                "showdeviants" => {
                    commands::print_deviants(&ctx, &message.channel_id, &message.guild_id).await
                }
                "fixdeviants" => commands::fix_deviants(&ctx, &message, &message.guild_id).await,
                _ => (),
            }
        }
    }

    async fn ready(&self, _ctx: Context, _: Ready) {
        println!("Online");
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Something happened while creating a client");

    if let Err(error) = client.start().await {
        println!("ERROR: {:?}", error)
    }
}
