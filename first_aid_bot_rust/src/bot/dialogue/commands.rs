use super::prelude::*;
use crate::{MAINTAINER_USERNAMES, REDIS_USERS_SET_KEY};
use anyhow::bail;
use futures::{future::BoxFuture, FutureExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;
use teloxide::dispatching::DpHandlerDescription;
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
    bot: FABot,
    cmd: FACommands,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FADialogue,
) -> anyhow::Result<()> {
    match cmd {
        FACommands::Start => {
            let args = FAMsgArgs::new(&bot, &msg, &dialogue, &data, redis_con);
            move_to_state(&args, Vec::new(), Lang::default()).await
        }
    }
}

pub async fn maintainer_commands_handler(
    msg: Message,
    bot: FABot,
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
                send_plain_string(&bot, msg.chat.id, &err.to_string()).await?;
                bail!(err)
            }
        }
    };
    Ok(())
}

async fn test(data: Arc<Data>, bot: &FABot, msg: &Message) -> anyhow::Result<()> {
    fn recursive_test<'a>(
        state: &'a FS,
        lang: Lang,
        context: Vec<String>,
        bot: &'a FABot,
        msg: &'a Message,
    ) -> BoxFuture<'a, anyhow::Result<()>> {
        async move {
            send_state(bot, msg.chat.id, state, lang, &context).await?;
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
        |msg: Message, _bot: FABot, _data: Arc<Data>, _redis_con: MultiplexedConnection| {
            msg.from()
                .map(|user| {
                    cfg!(debug_assertions)
                        || (user.username.is_some()
                            && MAINTAINER_USERNAMES
                                .contains(&user.username.as_ref().unwrap().as_str()))
                })
                .unwrap_or_default()
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
