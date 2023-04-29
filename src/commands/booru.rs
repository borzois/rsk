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

    match _posts {
        Ok(posts) => {
            for val in posts.iter() {
                msg
                    .channel_id
                    .send_message(&ctx.http, |m| { m
                        .embed(|e| { e
                                .title(&val.source)
                                .description(&val.tags)
                                .image(&val.file_url)
                        })
                    })
                    .await?;
            }
        },
        Err(error) => {
            msg
                .channel_id
                .send_message(&ctx.http,|m| { m
                    .embed(|e| { e
                        .image("https://i.guim.co.uk/img/media/d82ea2d029ea86532dc4a37ea920540065cd35b2/0_59_4000_2400/master/4000.jpg?width=1200&height=1200&quality=85&auto=format&fit=crop&s=7fc5febdff3241661f172fb216df523a")
                        .description("i couldn't find anything")
                    })
                })
                .await?;
            error!("{}", error.to_string())
        }
    }

    Ok(())
}