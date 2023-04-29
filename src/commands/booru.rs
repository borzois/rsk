use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use booru_rs::client::gelbooru::GelbooruClient;
use booru_rs::client::generic::BooruClient;
use booru_rs::client::generic::BooruBuilder;
use booru_rs::model::gelbooru::GelbooruRating;
use booru_rs::model::gelbooru::GelbooruSort;
use tracing::error;


#[command]
pub async fn booru(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut _posts = GelbooruClient::builder();
    while !args.is_empty() {
        _posts = _posts
            .tag(args.single::<String>()?);
    }
    let _posts = _posts
        .rating(GelbooruRating::General)
        .sort(GelbooruSort::Score)
        .limit(1)
        .random(true)
        .blacklist_tag(GelbooruRating::Explicit)
        .get()
        .await;
        // .expect("There was an error retrieving posts from the API");

    match _posts {
        Ok(posts) => {
            for val in posts.iter(){
                let response = &val.file_url;
                msg.channel_id.say(&ctx.http, response).await?;
            }
        },
        Err(error) => {
            msg.channel_id.say(&ctx.http,"i couldn't find that!").await?;
            error!("{}", error.to_string())
        }
    }

    Ok(())
}