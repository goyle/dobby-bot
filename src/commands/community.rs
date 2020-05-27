use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
#[aliases("ded", "dead", "death", "deaths")]
fn obituary(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Obituary :skull:");
            e.description("Collecting stories of the casualty. Grin is dead, long live Grin!");
            e.field("Grin Obituaries", "http://grin-obituaries.com/", false);

            e
        });
        m
    });
        
    Ok(())
}

#[command]
#[aliases("explorer")]
fn explorers(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Block Explorers :compass:");
            e.description("List of block explorers that provide detailed information about Grin blocks and transactions.");
            e.field("GrinExplorer", "https://grinexplorer.net/", false);
            e.field("GrinScan", "https://grinscan.net/", false);
            e.field("Blockscan", "https://grin.blockscan.com/", false);

            e
        });
        m
    });
        
    Ok(())
}

#[command]
#[aliases("request", "fundingrequest")]
fn funding_request(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Funding Request :pie:");
            e.description("Do you want to work full time or even part time on the Grin project? Come and submit a funding request to the community! Be prepared to state your motivation and respond to community feedback.");
            e.field("Funding Request Template", "https://github.com/mimblewimble/grin-pm/blob/master/financials/funding_request_template.md", false);
            e.field("Example of a full length funding request", "https://forum.grin.mw/t/request-for-funding-lehnberg-jan-mar-2020/6767", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("spending", "spendinglog")]
fn spending_log(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Spending Log :pound:");
            e.description("A log intended to keep the community up to date on how the Grin General Fund is being used.");
            e.field("Spending Log", "https://github.com/mimblewimble/grin-pm/blob/master/financials/spending_log.csv", false);
            e.field("Burn-rate & Runway Report", "https://github.com/mimblewimble/grin-pm/blob/master/financials/reports/burnrate_runway.md", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("wallet")]
fn wallets(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallets :purse:");
            e.description("List of Grin wallets.");
            e.image("attachment://grinplusplus.png");
            e.field("Reference CLI Wallet (Windows, Mac, Linux)", "https://github.com/mimblewimble/grin-wallet/releases", false);
            e.field("Grin++ GUI Wallet (Windows, Mac, Linux)", "https://grinplusplus.github.io/", false);
            e.field("Niffler GUI Wallet (Windows, Mac, Linux)", "https://github.com/grinfans/niffler/releases/tag/v0.5.0", false);
            e.field("Ironbelly Mobile Wallet (iOS, Android)","https://ironbelly.app/", false);
            e.footer(|f| {
                f.text("Grin++ Wallet v1.0.0");

                f
            });

            e
        });
        m.add_file(AttachmentType::Path(Path::new("./assets/grinplusplus.png")));
        m
    });

    Ok(())
}
