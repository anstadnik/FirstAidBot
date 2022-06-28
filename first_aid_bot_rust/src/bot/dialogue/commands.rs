use crate::bot::keyboard::make_keyboard_from_state;
use crate::{MAINTAINER_USERNAMES, REDIS_USERS_SET_KEY};

use super::prelude::*;
use anyhow::bail;
use futures::{future::BoxFuture, FutureExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "FirstAidBot")]
pub enum FACommands {
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
    bot: FirstAirBot,
    cmd: FACommands,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    match cmd {
        FACommands::Start => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, Lang::default().name()).await
        }
    }
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: FirstAirBot,
    cmd: MaintainerCommands,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    match cmd {
        MaintainerCommands::GetNumber => {
            match redis_con.scard::<_, i32>(REDIS_USERS_SET_KEY).await {
                Ok(n) => {
                    bot.send_message(msg.chat.id, n.to_string()).await?;
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

async fn test(data: Arc<Data>, bot: &FirstAirBot, msg: &Message) -> anyhow::Result<()> {
    fn recursive_test<'a>(
        state: &'a FS,
        lang: Lang,
        context: Vec<String>,
        bot: &'a FirstAirBot,
        msg: &'a Message,
    ) -> BoxFuture<'a, anyhow::Result<()>> {
        let keyboard = make_keyboard_from_state(state, lang, &context);
        async move {
            send_state(bot, msg, state, lang, keyboard).await?;
            for (key, next_state) in state.next_states.iter() {
                let mut context = context.clone();
                context.push(key.to_string());
                recursive_test(next_state, lang, context, bot, msg).await?;
            }
            anyhow::Ok(())
        }
        .boxed()
    }
    for lang in Lang::iter() {
        if let Ok(state) = data.get(lang, &[]).await {
            recursive_test(&state, lang, Vec::new(), bot, msg).await?;
        }
    }

    Ok(())
}

pub fn get_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::entry()
        .filter_command::<FACommands>()
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch(
) -> Handler<'static, DependencyMap, Result<(), anyhow::Error>, DpHandlerDescription> {
    dptree::filter(
        |msg: Message, _bot: FirstAirBot, _data: Arc<Data>, _redis_con: MultiplexedConnection| {
            msg.from()
                .map(|user| {
                    cfg!(debug_assertions)
                        || (user.username.is_some()
                            && MAINTAINER_USERNAMES.contains(&user.username.as_ref().unwrap().as_str()))
                })
                .unwrap_or_default()
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
