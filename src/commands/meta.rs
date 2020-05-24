use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Dobby Bot");
            e.description("Gives instant answers to Grin related questions!");
            e.image("attachment://dobby.png");
            e.footer(|f| {
                f.text("Dobby is a free house-elf!");

                f
            });

            e
        });
        m.add_file(AttachmentType::Path(Path::new("./img/dobby.png")));
        m
    });

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}
