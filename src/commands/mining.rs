use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
#[aliases("calc", "miningcalc", "miningcalculator")]
fn calculator(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Mining Calculator :abacus:");
            e.description("Calculate how profitable it is to mine Grin using your hashrates on the c29, c31, or c32 algorithms.");
            e.field("Grinmint Pool Calculator", "https://www.grinmint.com/calculator.html", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("miner")]
fn miners(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Miners :pick:");
            e.description("List of mining programs for Grin.");
            e.field("Bminer (For Nvidia GPUs)", "https://www.bminer.me", false);
            e.field("Gminer (For Nvidia GPUs)", "https://github.com/develsoftware/GMinerRelease/releases", false);
            e.field("lolMiner (For AMD GPUs)", "https://github.com/Lolliedieb/lolMiner-releases/releases", false);

            e
        });
        m
    });

    Ok(())
}
