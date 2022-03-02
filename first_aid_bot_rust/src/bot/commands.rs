use std::sync::Arc;

use redis::{aio::MultiplexedConnection, AsyncCommands};
use teloxide::{prelude2::*, utils::command::BotCommand};

use crate::{model::FiniteState, REDIS_KEY};

use super::dialogue::{reset_dialogue, FirstAidDialogue};

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
}

pub async fn commands_handler(
    msg: Message,
    bot: AutoSend<Bot>,
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
    bot: AutoSend<Bot>,
    cmd: MaintainerCommands,
    _data: Arc<FiniteState>,
    mut redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    match cmd {
        MaintainerCommands::GetNumber => {
            match redis_con.scard::<&str, i64>(REDIS_KEY).await {
                Ok(num) => {
                    bot.send_message(msg.chat.id, num.to_string()).await?;
                }
                Err(_) => {
                    log::error!("Error writing a user to the redis db.");
                }
            };
            Ok(())
        }
    }
}
