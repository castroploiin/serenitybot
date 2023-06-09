use rand::{thread_rng, Rng};
// use serenity::framework::standard::{
//     help_commands, Args, CommandGroup, CommandResult, HelpOptions,
// };
use serenity::model::prelude::{ChannelId, GuildId, Member, Message, UserId};
use serenity::prelude::Context;
use serenity::utils::{Color, MessageBuilder};
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use crate::structures::CmdDocumentation;

pub async fn pong(ctx: &Context, message: &Message) {
    let mut pong_buildup: MessageBuilder = MessageBuilder::new();
    let real_pong: &mut MessageBuilder = pong_buildup.push("Pong!");
    message
        .channel_id
        .say(&ctx.http, real_pong)
        .await
        .expect("Cannot send message");
}

pub async fn hello_embed(ctx: &Context, message: &Message) {
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|field| {
                field.title("Hello world");
                field.color(Color::from_rgb(141, 104, 246))
            })
        })
        .await
        .expect("Could not send message");
}

pub async fn get_input(ctx: &Context, message: &Message) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(3);
    let _ = message.reply(&ctx.http, "Say something").await;

    if let Some(answer) = &message
        .author
        .await_reply(&ctx)
        .timeout(TIMEOUT_DURATION)
        .await
    {
        let user_reply: String = format!("YOU SAID {}", &answer.content);
        answer
            .reply(&ctx, user_reply)
            .await
            .expect("Error parroting back user message");
    } else {
        message
            .channel_id
            .say(
                &ctx.http,
                format!(
                    "You did not send a message in {} seconds",
                    TIMEOUT_DURATION.as_secs()
                ),
            )
            .await
            .expect("Error sending timed out message");
    }
}

pub async fn get_help(ctx: &Context, message: &Message) {
    let _ = message
        .reply(&ctx.http, "What command do you want help with?")
        .await;

    if let Some(answer) = &message.author.await_reply(&ctx).await {
        let help: Result<String, serde_json::Error> = fetch_help_information(&answer.content).await;
        match help {
            Ok(command) => {
                answer
                    .channel_id
                    .send_message(&ctx.http, |msg| {
                        msg.embed(|e| {
                            e.title(&answer.content);
                            e.description(command)
                        })
                    })
                    .await
                    .expect("Could not send documentation message");
            }
            Err(err) => {
                message
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("Documentation does not exist for your input: {:?}", err),
                    )
                    .await
                    .expect("Couldn't send get_help error message");
            }
        }
    } else {
        message
            .channel_id
            .say(&ctx.http, "Something went wrong with input")
            .await
            .expect("Couldn't send get_help error message");
    }
}

pub async fn fetch_help_information(command: &String) -> Result<String, serde_json::Error> {
    let mut file: File = File::open("src/docs.json").expect("Could not open docs file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read JSON file");

    let commands_json: Vec<CmdDocumentation> = serde_json::from_str(&contents)?;
    for command_doc in commands_json {
        if command_doc.command == *command {
            return Ok(command_doc.information);
        }
    }

    Ok(String::from("Could not find command"))
}

pub async fn eightball(ctx: &Context, message: &Message) {
    let responses: Vec<&str> = vec!["yes", "no"];
    let num_responses: usize = responses.len();

    let random_response: &str = responses[thread_rng().gen_range(0..num_responses)];
    message
        .channel_id
        .say(&ctx.http, random_response)
        .await
        .expect("Could not send 8ball message");
}

pub async fn scan_for_deviants(ctx: &Context, current_guild: &GuildId) -> Vec<Member> {
    let mut list_of_deviants: Vec<Member> = Vec::new();
    let members: Vec<Member> = current_guild
        .to_partial_guild(ctx)
        .await
        .expect("Did not get Guild from GuildId")
        .members(ctx, None, UserId(00000000000000000000))
        .await
        .expect("");

    for member in members {
        let has_role: &bool = &member
            .user
            .has_role(ctx, *current_guild, 1126217605976436777)
            .await
            .expect("Could not determine if user has role or not");
        if !has_role {
            list_of_deviants.push(member)
        }
    }

    list_of_deviants
}

pub async fn print_deviants(
    ctx: &Context,
    channel_id: &ChannelId,
    current_guild: &Option<GuildId>,
) {
    match current_guild {
        Some(guild_id) => {
            let mut message = MessageBuilder::new();
            message.push("List of deviants: ");

            let mut names_of_deviants: Vec<String> = Vec::new();

            let list_of_deviants = scan_for_deviants(&ctx, guild_id).await;
            for deviant in list_of_deviants {
                names_of_deviants.push(deviant.user.name.to_string());
            }

            message.push(names_of_deviants.join(", "));
            channel_id
                .say(ctx, message)
                .await
                .expect("Could not send deviants list in channel");
        }
        _ => println!("No GuildId received in print_deviants"),
    }
}

// #[help]
// pub async fn my_help(
//     ctx: &Context,
//     message: &Message,
//     args: Args,
//     help_options: &'static HelpOptions,
//     groups: &[&'static CommandGroup],
//     owners: HashSet<UserId>,
// ) -> CommandResult {
//     let help = help_commands::with_embeds(ctx, message, args, help_options, groups, owners).await;
//     help
// }

pub async fn fix_deviants(ctx: &Context, message: &Message, current_guild: &Option<GuildId>) {
    match current_guild {
        Some(guild_id) => {
            let members: Vec<Member> = scan_for_deviants(&ctx, &guild_id).await;
            for mut member in members {
                member
                    .add_role(&ctx, 1126217605976436777)
                    .await
                    .expect("Could not add role to deviint");
            }

            let mut complete_signal = MessageBuilder::new();
            complete_signal.push("Added roles to all deviants");
            message
                .channel_id
                .say(ctx, complete_signal)
                .await
                .expect("Could not send success message from fix_deviants");
        }
        _ => println!("No GuildID received in fix_deviants"),
    }
}
