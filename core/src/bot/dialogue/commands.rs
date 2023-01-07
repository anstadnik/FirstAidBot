use crate::bot::FirstAidStorage;
use std::sync::Arc;

use super::logic::is_admin;
use super::prelude::start_endpoint;
use crate::bot::dialogue::logic::send_state;
use crate::bot::report_error::{report_error, ReportError};
use crate::bot::{FABot, FADialogue};
use crate::prelude::{Data, Lang, State};
use crate::REDIS_USERS_SET_KEY;
use anyhow::{anyhow, bail, Context, Error};
use futures::{future::BoxFuture, FutureExt};
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
    data: Arc<Data>,
    conn: MultiplexedConnection,
    dialogue: FADialogue,
) -> anyhow::Result<()> {
    let lang = Lang::default().name();
    match cmd {
        FACommands::Start => start_endpoint(bot, msg, data, dialogue, lang, conn).await,
    }
}

pub async fn maintainer_commands_handler(
    bot: FABot,
    msg: Message,
    cmd: MaintainerCommands,
    data: Arc<Data>,
    mut conn: MultiplexedConnection,
) -> anyhow::Result<()> {
    let id = msg.chat.id;
    match cmd {
        MaintainerCommands::GetNumber => match conn.scard::<_, i32>(REDIS_USERS_SET_KEY).await {
            Ok(n) => bot
                .send_message(id, n.to_string())
                .await
                .map(|_| ())
                .map_err(Error::msg),
            Err(err) => {
                let err = anyhow!(err);
                report_error(&bot, id, Lang::default().details().error, &err).await;
                bail!(err)
            }
        },
        MaintainerCommands::Test => {
            let text = Lang::default().details().error;
            test(data, &bot, &msg)
                .await
                .report_if_err(&bot, id, text)
                .await
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

fn recursive_test<'a>(
    state: &'a mut State,
    data: &'a Data,
    bot: &'a FABot,
    msg: &'a Message,
) -> BoxFuture<'a, anyhow::Result<()>> {
    async move {
        send_state(bot, msg, state)
            .await
            .with_context(|| format!("Error while processing state {state}"))?;
        for next_state in state.button_texts.clone() {
            state.move_to_state(&next_state, data).await?;
            recursive_test(state, data, bot, msg).await?;
            state.back();
        }
        Ok(())
    }
    .boxed()
}

async fn test(data: Arc<Data>, bot: &FABot, msg: &Message) -> anyhow::Result<()> {
    for lang in Lang::iter() {
        let mut state = data.get(&[], lang).await?;
        recursive_test(&mut state, &data, bot, msg).await?;
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
