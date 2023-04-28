use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
pub async fn booru(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut response = MessageBuilder::new();
    response
        .push("user ")
        .push_italic_safe(&msg.author.name)
        .push(" has requested ");
    while !args.is_empty() {
        response
            .push_bold(args.single::<String>()?)
            .push(" ");
    }
    response
        .push_line("- shame on them!")
        .build();

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}