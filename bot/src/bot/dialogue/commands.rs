use crate::bot::FirstAidStorage;
use first_aid_bot_core::prelude::*;
use std::collections::VecDeque;
use std::sync::Arc;

use super::logic::is_admin;
use super::prelude::start_endpoint;
use crate::bot::dialogue::logic::send_state;
use crate::bot::report_error::report_error;
use crate::bot::{FABot, FADialogue};
use crate::REDIS_USERS_SET_KEY;
use anyhow::{anyhow, bail, Context, Error};
use redis::{aio::MultiplexedConnection, AsyncCommands};
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
    data: &'static Data,
    conn: MultiplexedConnection,
    dialogue: FADialogue,
) -> anyhow::Result<()> {
    let lang = Lang::default().name();
    match cmd {
        FACommands::Start => start_endpoint(bot, msg, data, dialogue, &lang, conn).await,
    }
}

pub async fn maintainer_commands_handler(
    bot: FABot,
    msg: Message,
    cmd: MaintainerCommands,
    data: &Data,
    mut conn: MultiplexedConnection,
) -> anyhow::Result<()> {
    let id = msg.chat.id;
    match cmd {
        MaintainerCommands::GetNumber => match conn.scard::<_, i32>(REDIS_USERS_SET_KEY).await {
            Ok(n) => {
                bot.send_message(id, n.to_string()).await?;
                Ok(())
            }
            Err(err) => {
                let err = anyhow!(err);
                report_error(&bot, id, Lang::default().details().error, &err).await;
                bail!(err)
            }
        },
        MaintainerCommands::Test => {
            let text = Lang::default().details().error;
            match test(data, &bot, &msg).await {
                Err(err) => {
                    report_error(&bot, id, text, &err).await;
                    Err(err)
                }
                rez => rez,
            }
        }
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
    let mut q: VecDeque<_> = [(fs, ctx)].into();
    while let Some((fs, ctx)) = q.pop_front() {
        send_state(bot, msg, &ctx, fs)
            .await
            .with_context(|| format!("Error while processing state {ctx}"))?;
        q.extend(fs.next_states.iter().map(|(s, fs)| {
            let mut ctx = ctx.clone();
            ctx.transition(s);
            (fs, ctx)
        }));
    }
    Ok(())
}

async fn test(data: &Data, bot: &FABot, msg: &Message) -> anyhow::Result<()> {
    for lang in Lang::iter() {
        let ctx = FAContext {
            lang,
            context: Vec::new(),
        };
        let fs = data.get().await?.get_state(&ctx)?;
        recursive_test(&fs, ctx, bot, msg).await?;
    }

    Ok(())
}

type FAHandler = Handler<'static, DependencyMap, Result<(), Error>, DpHandlerDescription>;

pub fn get_commands_branch() -> FAHandler {
    dptree::entry()
        .filter_command::<FACommands>()
        .enter_dialogue::<Message, FirstAidStorage, crate::bot::state::State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch() -> FAHandler {
    dptree::filter(
        |msg: Message, _bot: FABot, _data: Arc<Data>, _conn: MultiplexedConnection| {
            cfg!(debug_assertions) || is_admin(&msg)
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
