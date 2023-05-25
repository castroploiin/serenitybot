use std::time::Duration;
use serenity::model::prelude::{Message};
use serenity::prelude::Context;
use serenity::utils::{MessageBuilder, Color};

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
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|field| {
                field.title("Hello world");
                field.color(Color::from_rgb(255, 0, 255))
            })
        })
        .await
        .expect("Could not send message");
}

pub async fn get_input(ctx: Context, message: Message) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(5);
    let _ = message.reply(&ctx.http, "Say something").await;

    if let Some(answer) = &message.author.await_reply(&ctx).timeout(TIMEOUT_DURATION).await {
        let user_reply = format!("YOU SAID {}", &answer.content);
        answer.reply(&ctx, user_reply).await.expect("Error parroting back user message");
    } else {
        message.channel_id.say(&ctx.http, "You did not send a message in 3 seconds").await.expect("Error sending timed out message");
    }
}