use crate::MAINTAINER_ID;

use super::prelude::*;
use anyhow::{bail, Context};
use futures::StreamExt;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{collections::VecDeque, sync::Arc};
use teloxide::dispatching::DpHandlerDescription;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "FirstAidBot")]
pub enum FirstAidCommands {
    #[command(description = "Reboot")]
    Start,
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Maintainer commands")]
pub enum MaintainerCommands {
    #[command(description = "Get a number of unique users")]
    GetNumber,
    #[command(description = "Test all messages")]
    Test,
}

pub async fn commands_handler(
    msg: Message,
    bot: FirstAidBot,
    cmd: FirstAidCommands,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    match cmd {
        FirstAidCommands::Start => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, Lang::default().name()).await
        }
    }
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: FirstAidBot,
    cmd: MaintainerCommands,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    match cmd {
        MaintainerCommands::GetNumber => {
            match redis_con.scan_match::<&str, String>("user_*").await {
                Ok(keys) => {
                    bot.send_message(msg.chat.id, keys.count().await.to_string())
                        .await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, "Error getting a number of users")
                        .await?;
                    bail!(err)
                }
            }
        }
        #[allow(deprecated)]
        MaintainerCommands::Test => {
            if let Err(err) = test(data, &bot, &msg).await {
                bot.send_message(msg.chat.id, err.to_string())
                    .parse_mode(ParseMode::Markdown)
                    .await?;
                bail!(err)
            }
        }
    };
    Ok(())
}

async fn test(data: Arc<Data>, bot: &FirstAidBot, msg: &Message) -> anyhow::Result<()> {
    for (&lang, states) in &*data.get().await? {
        let mut state_deque = VecDeque::new();
        state_deque.push_back((Vec::new(), states));
        while let Some((context, state)) = state_deque.pop_front() {
            let keyboard = make_keyboard(state.get_options(), lang, &context);
            send_state(bot, msg, state, lang, keyboard).await?;
            for option in state.get_options() {
                let mut context = context.clone();
                context.push(option.to_string());
                let next_state = state.get_next_state(option).context(format!(
                    "Cannot find next state: {state:?} being on {context:?}"
                ));
                state_deque.push_back((context, next_state?))
            }
        }
    }
    Ok(())
}

pub fn get_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::entry()
        .filter_command::<FirstAidCommands>()
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::filter(
        |msg: Message, _bot: FirstAidBot, _data: Arc<Data>, _redis_con: MultiplexedConnection| {
            msg.from()
                .map(|user| cfg!(debug_assertions) || user.id == MAINTAINER_ID)
                .unwrap_or_default()
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
