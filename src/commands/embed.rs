use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{
        channel::Message,
        id::ChannelId,
    },
    prelude::*,
    utils::Colour,
};

#[group]
#[description = "공지를 임베드합니다."]
#[commands(embed_notice)]
pub struct Embed;

/// JSON data to embed
#[derive(Serialize, Deserialize)]
struct ToEmbed {
    #[serde(default)]
    bind: Option<u64>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    footer: Option<String>,
    #[serde(default)]
    fields: Option<Vec<(String, String, bool)>>,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    thumbnail: Option<String>,
    #[serde(default)]
    url: Option<String>,
}

/// embed the message of the user
#[command]
#[aliases("공지")]
#[description = "공지를 임베드합니다."]
pub async fn embed_notice(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_embed: ToEmbed = serde_json::from_str(&args.rest()).expect("JSON 형태로 전달하라요!!");

    let channel = match to_embed.bind {
        Some(id) => ChannelId(id),
        None => msg.channel_id,
    };

    channel.send_message(&ctx.http, |m| {
        m.embed(|e| {
            match to_embed.title {
                Some(title) => {
                    e.title(title);
                },
                None => (),
            }
            match to_embed.description {
                Some(description) => {
                    e.description(description);
                },
                None => (),
            }
            match to_embed.footer {
                Some(footer) => {
                    e.footer(|f| {
                        f.text(footer);

                        f
                    });
                },
                None => (),
            }
            match to_embed.fields {
                Some(fields) => {
                    e.fields(fields.into_iter());
                },
                None => (),
            }
            match to_embed.color {
                Some(color) => {
                    e.colour(Colour::from(i32::from_str_radix(&color[..], 16).unwrap()));
                },
                None => (),
            }
            match to_embed.image {
                Some(url) => {
                    e.image(url);
                },
                None => (),
            }
            match to_embed.thumbnail {
                Some(url) => {
                    e.thumbnail(url);
                },
                None => (),
            }
            match to_embed.url {
                Some(url) => {
                    e.url(url);
                },
                None => (),
            }

            e
        });

        m
    }).await?;

    Ok(())
}
