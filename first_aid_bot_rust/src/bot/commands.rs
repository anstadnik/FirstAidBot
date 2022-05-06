use super::{
    dialogue::{reset_dialogue, FirstAidDialogue, State},
    helpers::{send_message, ExtraKeys},
};
use crate::{lang::Lang, model::prelude::*, MAINTAINER_ID, REDIS_KEY};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{collections::VecDeque, sync::Arc};
use teloxide::dispatching::DpHandlerDescription;
use teloxide::{adaptors::DefaultParseMode, prelude::*, utils::command::BotCommands};
use teloxide::{
    adaptors::Throttle,
    dispatching::dialogue::{serializer::Bincode, RedisStorage},
};

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
    bot: AutoSend<DefaultParseMode<Throttle<Bot>>>,
    cmd: FirstAidCommands,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    let _ = match cmd {
        FirstAidCommands::Start => {
            dialogue.exit().await?;
            let lang = Lang::default();
            return reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await;
        }
    };
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: AutoSend<DefaultParseMode<Throttle<Bot>>>,
    cmd: MaintainerCommands,
    data: Arc<Data>,
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
            'outer: for (lang, states) in &*data.get().await? {
                let mut state_deque = VecDeque::new();
                state_deque.push_back(states);
                while let Some(state) = state_deque.pop_front() {
                    if send_message(&bot, &msg, state, ExtraKeys::empty(*lang))
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
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::entry()
        .filter_command::<FirstAidCommands>()
        .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::filter(
        |msg: Message,
         _bot: AutoSend<DefaultParseMode<Throttle<Bot>>>,
         _data: Arc<Data>,
         _redis_con: MultiplexedConnection| {
            msg.from()
                .map(|user| cfg!(debug_assertions) || user.id == MAINTAINER_ID)
                .unwrap_or_default()
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
