use std::{collections::VecDeque, sync::Arc};

use redis::{aio::MultiplexedConnection, AsyncCommands};
use teloxide::{adaptors::DefaultParseMode, prelude2::*, utils::command::BotCommand};

use crate::{model::FiniteState, REDIS_KEY};

use super::{
    dialogue::{reset_dialogue, FirstAidDialogue},
    helpers::send_message,
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
    data: Arc<FiniteState>,
    redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    let _ = match cmd {
        FirstAidCommands::Start => {
            dialogue.exit().await?;
            return reset_dialogue(bot, msg, data, redis_con, dialogue).await;
        }
    };
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: AutoSend<DefaultParseMode<Bot>>,
    cmd: MaintainerCommands,
    data: Arc<FiniteState>,
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
            let mut states = VecDeque::new();
            states.push_back(data.as_ref());
            while let Some(state) = states.pop_front() {
                if send_message(&bot, &msg, state).await.is_err() {
                    break;
                }
                if let Some(children) = &state.options {
                    states.extend(children.next_states.values());
                }
            }
        }
    };
    Ok(())
}
