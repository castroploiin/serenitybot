use serenity::model::prelude::{Embed, Message};
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

pub async fn pong(ctx: Context, message: Message) {
    let mut pong_buildup = MessageBuilder::new();
    let real_pong = pong_buildup.push("Pong!");
    message
        .channel_id
        .say(&ctx.http, real_pong)
        .await
        .expect("Cannot send message");
}

pub async fn hello_embed(ctx: Context, message: Message) {
    let embed = Embed::fake(|e| e.title("Hello!").description("Testing"));
    message
        .channel_id
        .say(&ctx.http, embed)
        .await
        .expect("Could not send message");
}

