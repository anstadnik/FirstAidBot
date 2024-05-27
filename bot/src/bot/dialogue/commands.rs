use super::logic::is_admin;
use super::prelude::start_endpoint;
use crate::bot::dialogue::logic::send_state;
use crate::bot::FirstAidStorage;
use crate::bot::{FABot, FADialogue};
use crate::{DataGetState, DATA, REDIS_CONN, REDIS_USERS_SET_KEY};
use anyhow::{anyhow, Context, Error};
use first_aid_bot_core::prelude::*;
use redis::AsyncCommands;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "FirstAidBot")]
pub enum FACommands {
    #[command(description = "Reboot")]
    Start,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Maintainer commands")]
pub enum MaintainerCommands {
    #[command(description = "Get a number of unique users")]
    GetNumber,
    #[command(description = "Test all messages")]
    Test,
    #[command(description = "Hmm")]
    GifTest,
}

pub async fn commands_handler(
    bot: FABot,
    msg: Message,
    cmd: FACommands,
    dialogue: FADialogue,
) -> anyhow::Result<()> {
    match cmd {
        FACommands::Start => start_endpoint(bot, msg, dialogue, Lang::default()).await,
    }
}

pub async fn maintainer_commands_handler(
    bot: FABot,
    msg: Message,
    cmd: MaintainerCommands,
) -> anyhow::Result<()> {
    let id = msg.chat.id;
    match cmd {
        MaintainerCommands::GetNumber => {
            let mut conn = REDIS_CONN
                .get()
                .ok_or_else(|| anyhow!("Redis connection is not initialized"))?
                .clone();
            let n = conn.scard::<_, i32>(REDIS_USERS_SET_KEY).await?;
            bot.send_message(id, n.to_string()).await?;
            Ok(())
        }
        MaintainerCommands::Test => test(&bot, &msg).await,
        MaintainerCommands::GifTest => easter_egg(&bot, &msg).await,
    }
}

pub async fn easter_egg(bot: &FABot, msg: &Message) -> Result<(), Error> {
    for _ in 0..10 {
        let link = "https://media.tenor.com/O09x7_40xeIAAAAj/dance.gif";
        let text = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(msg.chat.id, text).parse_mode(Html).await?;
    }
    Ok(())
}

async fn recursive_test(fs: &Fs, ctx: FAContext, bot: &FABot, msg: &Message) -> anyhow::Result<()> {
    let mut q = vec![(fs, ctx)];
    while let Some((fs, ctx)) = q.pop() {
        send_state(bot, msg, &ctx, fs).await.with_context(|| {
            format!(
                "Error while processing state {ctx}. Message is {}",
                fs.message
            )
        })?;
        q.extend(fs.next_states.iter().map(|(s, fs)| {
            let mut ctx = ctx.clone();
            ctx.transition(s.to_string());
            (fs, ctx)
        }));
    }
    Ok(())
}

async fn test(bot: &FABot, msg: &Message) -> anyhow::Result<()> {
    for lang in Lang::iter() {
        let context = Vec::new();
        let ctx = FAContext { lang, context };
        recursive_test(&*DATA.get_state(&ctx).await?, ctx, bot, msg).await?;
    }

    Ok(())
}

type FAHandler = Handler<'static, DependencyMap, Result<(), Error>, DpHandlerDescription>;

pub fn get_commands_branch() -> FAHandler {
    dptree::entry()
        .filter_command::<FACommands>()
        .enter_dialogue::<Message, FirstAidStorage, crate::bot::State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch() -> FAHandler {
    dptree::filter(|msg: Message, _bot: FABot| cfg!(debug_assertions) || is_admin(&msg))
        .filter_command::<MaintainerCommands>()
        .endpoint(maintainer_commands_handler)
}
