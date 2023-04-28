use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    msg.channel_id.say(&ctx.http, "not yet implemented :(").await?;

    Ok(())
}