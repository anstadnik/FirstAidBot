use super::{
    dialogue::{reset_dialogue, FirstAidDialogue, State},
    helpers::{send_message, ExtraKeys},
    MultilangStates,
};
use crate::{LANGS, MAINTAINER_ID, REDIS_KEY};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{collections::VecDeque, sync::Arc};
use teloxide::{
    adaptors::DefaultParseMode,
    dispatching2::dialogue::{serializer::Bincode, RedisStorage},
    prelude2::*,
    utils::command::BotCommand,
};

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "FirstAidBot")]
pub enum FirstAidCommands {
    #[command(description = "Перезавантажити")]
    Start,
}

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Maintainer commands")]
pub enum MaintainerCommands {
    #[command(description = "Get a number of unique users")]
    GetNumber,
    #[command(description = "Test all messages")]
    Test,
}

pub async fn commands_handler(
    msg: Message,
    bot: AutoSend<DefaultParseMode<Bot>>,
    cmd: FirstAidCommands,
    data: Arc<MultilangStates>,
    redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    let _ = match cmd {
        FirstAidCommands::Start => {
            dialogue.exit().await?;
            let lang = LANGS[0].name.to_string();
            return reset_dialogue(bot, msg, data, redis_con, dialogue, (lang,)).await;
        }
    };
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: AutoSend<DefaultParseMode<Bot>>,
    cmd: MaintainerCommands,
    data: Arc<MultilangStates>,
    mut redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    match cmd {
        MaintainerCommands::GetNumber => {
            match redis_con.scard::<&str, i64>(REDIS_KEY).await {
                Ok(num) => {
                    bot.send_message(msg.chat.id, num.to_string()).await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, "Error getting a number of ppl")
                        .await?;
                    bot.send_message(msg.chat.id, format!("{err:#?}")).await?;
                }
            };
        }
        MaintainerCommands::Test => {
            'outer: for states in data.values() {
                let mut state_deque = VecDeque::new();
                state_deque.push_back(states);
                while let Some(state) = state_deque.pop_front() {
                    if send_message(&bot, &msg, state, ExtraKeys::empty())
                        .await
                        .is_err()
                    {
                        break 'outer;
                    }
                    if let Some(children) = &state.options {
                        state_deque.extend(children.next_states.values());
                    }
                }
            }
        }
    };
    Ok(())
}

pub fn get_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, std::convert::Infallible> {
    dptree::entry()
        .filter_command::<FirstAidCommands>()
        .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, std::convert::Infallible> {
    dptree::filter(
        |msg: Message,
         _bot: AutoSend<DefaultParseMode<Bot>>,
         _data: Arc<MultilangStates>,
         _redis_con: MultiplexedConnection| {
            msg.from()
                .map(|user| user.id == MAINTAINER_ID)
                .unwrap_or_default()
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
