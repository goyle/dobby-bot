use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
#[aliases("igno", "Igno", "Ignotus")]
fn ignotus(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Ignotus Peverell :bust_in_silhouette: ");
            e.description("The one and only, Ignotus Peverell, the anonymous founder of Grin. Left the project into the community's hands in June of 2019.");
            e.image("attachment://ignotus.jpeg");
            e.field("Github", "https://github.com/ignopeverell", false);
            e.field("Grin Forum", "https://forum.grin.mw/u/igno.peverell/", false);
            e.footer(|f| {
                f.text("\"Ceci n’est pas un Ignotus.\" \n\
                \t -Ignotus Peverell's last words");

                f
            });

            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/people/ignotus.jpeg")));
        m
    });
        
    Ok(())
}
