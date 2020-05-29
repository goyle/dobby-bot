use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
#[aliases("grin++","grinplusplus")]
fn grinpp(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Grin++ Wallet");
            e.thumbnail("attachment://grinplusplus-logo.png");
            e.description("_Fast, Private and Secure Grin Wallet._ \n\
            \n\
            Features include multiplatform, coin control, multi-user, multi-language, and security audited code. Based on Grin++, a Lightning-Fast C++ Implementation of Grin.");
            e.field("Grin++ Website", "https://grinplusplus.github.io/", false);
            e.image("attachment://grinplusplus.png");
            e.footer(|f| {
                f.text("Grin++ Wallet v1.0.0");
    
                f
            });
            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/grinplusplus-logo.png")));
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/grinplusplus.png")));
        m
    });

    Ok(())
}

#[command]
fn ironbelly(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Ironbelly Wallet");
            e.thumbnail("attachment://ironbelly-logo.png");
            e.description("_Grin wallet you've deserved._ \n\
            \n\
            Ironbelly is a mobile wallet for Grin blockchain. You can: \n\
            - Send Grin via http(s) or File \n\
            - Receive Grin via File (via http(s) is coming soon!) \n\
            - Protect your funds with Touch ID or Face ID \n\
            - See your funds using alternative currency ");
            e.field("Ironbelly Website", "https://ironbelly.app/", false);
            e.image("attachment://ironbelly.png");
            e.footer(|f| {
                f.text("Ironbelly Wallet v3.1.0");
    
                f
            });
            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/ironbelly-logo.png")));
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/ironbelly.png")));
        m
    });

    Ok(())
}

#[command]
fn niffler(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "_Work-in-progress_");

    Ok(())
}
