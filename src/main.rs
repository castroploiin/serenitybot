use serenity::model::user::User;
use serenity::utils::MessageBuilder;
use serenity::{prelude::*, async_trait};
use serenity::model::prelude::{Ready, ChannelId, Message, UserId};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
        let general_chat= ChannelId(1110107351287087227);
        let message = MessageBuilder::new().push("Hello World.").build();
        general_chat.say(&ctx.http, message).await.expect("Could not send message");
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.id != UserId(1110103541021954148) {
            let reply_message = MessageBuilder::new().push(format!("No {}, this is not true.", message.author.name)).build();
            message.channel_id.say(&ctx.http, reply_message).await.expect("Could not send message");
        }
    }
}


#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token");
    let intents = GatewayIntents::GUILD_MESSAGES 
                | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Something happened while creating a client");

    if let Err(error) = client.start().await {
        println!("ERROR: {:?}", error)
    }
}
