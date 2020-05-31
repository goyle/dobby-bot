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
            Ironbelly is a mobile wallet for Grin blockchain. \n\
            You can: \n\
            - Send Grin via http(s) or File \n\
            - Receive Grin via File (via http(s) is coming soon!) \n\
            - Protect your funds with Touch ID or Face ID \n\
            - See your funds using alternative currency");
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
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Niffler Wallet");
            e.thumbnail("attachment://niffler-logo.png");
            e.description("_Out-of-the-box user-friendly GUI Grin wallet_ \n\
            \n\
            Features include a simple, straightforward user interface, multiple languages (English and 简体中文), and the Hedwig v1 relay service.
            Hedwig provides a temporary address for users without a public IP to receive Grin.");
            e.field("Github", "https://github.com/grinfans/niffler", false);
            e.image("attachment://niffler.png");
            e.footer(|f| {
                f.text("Niffler Wallet v0.5.0");
    
                f
            });
            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/niffler-logo.png")));
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/niffler.png")));
        m
    });

    Ok(())
}

#[command]
#[aliases("713")]
fn wallet713(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallet713");
            e.description("_A Swiss Army knife for Grin._ \n\
            \n\
            Wallet713 makes it easy to store, send and soon also swap grins seamlessly through a single interface. Built on top of the standard Grin wallet reference implementation, wallet713 extends its functionality to improve usability and reduce friction. For better privacy and usability, the grinbox messaging relay allows the steps to build transactions (partial transactions, or \"slates\") to be routed via the relay, protecting the user from exposing their IP address, and with no impact to the safety of their funds.");
            e.field("Website", "https://713.mw/", false);
            e.field("Github", "https://github.com/vault713/wallet713", false);
            e.image("attachment://wallet713.png");
            e.footer(|f| {
                f.text("wallet713 CLI interface");
    
                f
            });
            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/wallets/wallet713.png")));
        m
    });

    Ok(())
}
