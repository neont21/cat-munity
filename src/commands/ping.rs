use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "핑퐁이 아니고 핑뇽"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "뇽!! :)").await?;

    Ok(())
}
