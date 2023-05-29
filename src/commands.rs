use std::fs::File;
use std::io::Read;
use std::time::Duration;
use rand::{thread_rng, Rng};
use serenity::model::prelude::{Message};
use serenity::prelude::Context;
use serenity::utils::{MessageBuilder, Color};

use crate::structures::CmdDocumentation;

pub async fn pong(ctx: Context, message: Message) {
    let mut pong_buildup: MessageBuilder = MessageBuilder::new();
    let real_pong: &mut MessageBuilder = pong_buildup.push("Pong!");
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
        let user_reply: String = format!("YOU SAID {}", &answer.content);
        answer.reply(&ctx, user_reply).await.expect("Error parroting back user message");
    } else {
        message.channel_id.say(&ctx.http, format!("You did not send a message in {} seconds", TIMEOUT_DURATION.as_secs())).await.expect("Error sending timed out message");
    }
}

pub async fn get_help(ctx: Context, message: Message) {
    let _ = message.reply(&ctx.http, "What command do you want help with?").await;

    if let Some(answer) = &message.author.await_reply(&ctx).await {
        let help: Result<String, serde_json::Error> = fetch_help_information(answer.content.clone()).await;
        match help {
            Ok(command) => {
                answer.channel_id.send_message(&ctx.http, |msg| {
                    msg.embed(|e| {
                        e.title(answer.content.clone());
                        e.description(command)
                    })
                }).await.expect("Could not send documentation message");
            },
            Err(err) => {
                message.channel_id.say(&ctx.http, format!("Documentation does not exist for your input: {:?}", err)).await.expect("Couldn't send get_help error message");
            }
        }
    } else {
        message.channel_id.say(&ctx.http, "Something went wrong with input").await.expect("Couldn't send get_help error message");
    }
}
 
pub async fn fetch_help_information(command: String) -> Result<String, serde_json::Error> {
    let mut file: File = File::open("src/docs.json").expect("Could not open docs file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Could not read JSON file");

    let commands_json: Vec<CmdDocumentation> = serde_json::from_str(&contents)?;
    for command_doc in commands_json {
        if command_doc.command == command {
            return Ok(command_doc.information)
        }
    }

    Ok(String::from("Could not find command"))
}

pub async fn eightball(ctx: Context, message: Message) {
    let responses: Vec<&str> = vec!["yes", "no"];
    let num_responses: usize = responses.len();
    
    let random_response: &str = responses[thread_rng().gen_range(0..num_responses)];
    message.channel_id.say(&ctx.http, random_response).await.expect("Could not send 8ball message");
}