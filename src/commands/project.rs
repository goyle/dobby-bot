use std::path::Path;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
#[aliases("coc", "CoC", "COC", "code", "conduct", "codeofconduct")]
fn code_of_conduct(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Code of Conduct :book:");
            e.description("We are committed to providing a friendly, safe and welcoming environment for all, regardless of level of experience, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, nationality, or other similar characteristic... \n\
            (_Continue reading_) \n\
            > https://github.com/mimblewimble/grin/blob/master/CODE_OF_CONDUCT.md");

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("info", "grininfo", "grinfo")]
fn coin(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title("Grin :no_mouth:");
            e.description("Basic information about Grin.");
            e.field("Statement", "Grin is electronic cash with a lightweight implementation of Mimblewimble. Mimblewimble is a blockchain protocol that provides privacy and scalability gains by verifying that all transactions are valid without needing to store the entire history of the chain.", false);
            e.field("For more info", "https://github.com/mimblewimble/grin/blob/master/doc/intro.md", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("emissions", "blocktime", "reward", "miningreward", "monetarypolicy")]
fn emission(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Emission :cloud_rain:");
            e.description("1 ツ per second forever. Each block contains a 60 ツ reward and the block confirmation time is targeted at 1 minute per block.");
            e.field("For more info on Grin's monetary policy", 
            "https://github.com/mimblewimble/docs/wiki/Monetary-Policy", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
fn governance(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title("Governance :man_mage:");
            e.description("Grin's governance today consists of the Grin Technocratic Council, and the rest of the community. The council came to be as there was a need for some sub-set of community members to manage the multi-sig keys of the Grin General Fund... \n\
            (_Continue reading_) \n\
            > https://github.com/mimblewimble/grin-rfcs/blob/master/text/0002-grin-governance.md");
            e.field("Core Team", "hashmap, antiochp, lehnberg, ~~ignotus~~, jaspervdm, tromp, j01tz, yeastplume, quentinlesceller", false);
            e.field("For more info", 
            "https://grin.mw/governance", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
fn philosophy(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title("Philosophy :brain:");
            e.description("Fundamental beliefs of Grin.");
            e.field("Statement", "Grin likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. Grin is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas. \n\
            We believe in pull requests, data, and scientific research. We do not believe in unfounded beliefs.", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
fn roadmap(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title("Roadmap :railway_track:");
            e.description("Areas and objectives that members of the Grin community would like to prioritize in 2020.");
            e.field("Grin Forum", "https://forum.grin.mw/t/grin2020-roadmap/7096", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
#[aliases("currency", "currencysymbol")]
fn symbol(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Currency Symbol :symbols:");
            e.description("The graphic symbol used as a shorthand for Grin.");
            e.field("Symbol", "ツ", false);

            e
        });
        m
    });

    Ok(())
}

#[command]
fn whitepaper(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.title("White Papers :page_facing_up:");
            e.field("The Mimblewimble White Paper", "On August 2nd, 2016, an anonymous person named Tom Elvis Jedusor dropped the original Mimblewimble white paper on the Bitcoin research IRC channel, #bitcoin-wizards, and then signed off. The white paper was a blockchain proposal that would greatly improve privacy, scalability, and fungibility in Bitcoin. \n\
            _Read the Mimlewimble white paper:_ \n\
            > https://github.com/mimblewimble/docs/wiki/MimbleWimble-Origin", false);
            e.field("Andrew Poelstra's Version", "A few months later, on October 6, 2016, Andrew Poelstra, a mathematician working for Blockstream, wrote his own paper that would make precise the original Mimblewimble white paper and contribute more scalability improvements. \n\
            _Read Andrew Poelstra's version:_ \n\
            > https://download.wpsoftware.net/bitcoin/wizardry/mimblewimble.pdf \n", false);
            e.field("Introduction to Mimblewimble and Grin", "However, Bitcoin’s scripting system was a heavy obstacle to adding Mimblewimble to the Bitcoin protocol and so, a few days later, an anonymous person named Ignotus Peverell launched the Grin project and began building the codebase. Ignotus would later write a technical introduction to Mimblewimble and Grin on March 20, 2017. \n\
            _Read \"Introduction to Mimblewimble and Grin\" by Ignotus Peverell:_ \n\
            > https://github.com/mimblewimble/grin/blob/master/doc/intro.md", false);

            e
        });
        m
    });

    Ok(())
}
