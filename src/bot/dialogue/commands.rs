use super::prelude::start_handler;
use crate::bot::dialogue::logic::send_state;
use crate::bot::prelude::*;
use crate::{MAINTAINER_USERNAMES, REDIS_USERS_SET_KEY};
use anyhow::{anyhow, bail, Context, Error};
use futures::{future::BoxFuture, FutureExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};
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
            start_handler(bot, msg, data, redis_con, dialogue, Lang::default().name()).await
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
                Ok(n) => bot
                    .send_message(msg.chat.id, n.to_string())
                    .await
                    .map(|_| ())
                    .map_err(Error::msg),
                Err(err) => {
                    let err = anyhow!(err);
                    report_error(&bot, msg.chat.id, &Lang::default(), &err).await;
                    bail!(err)
                }
            }
        }
        MaintainerCommands::Test => {
            test(data, &bot, &msg)
                .await
                .report_if_err(&bot, msg.chat.id, &Lang::default())
                .await
        }
    }
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
            send_state(bot, msg.chat.id, state, lang, &context)
                .await
                .with_context(|| format!("Error while processing state {state:?}"))?;
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
        for text in [
            lang.details().error,
            lang.details().greeting,
            lang.details().use_buttons_text,
            lang.details().error_due_to_update,
        ] {
            bot.send_message(msg.chat.id, text).await?;
        }
    }

    Ok(())
}

type FAHandler = Handler<'static, DependencyMap, Result<(), Error>, DpHandlerDescription>;

pub fn get_commands_branch() -> FAHandler {
    dptree::entry()
        .filter_command::<FACommands>()
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .endpoint(commands_handler)
}

pub fn get_maintainer_commands_branch() -> FAHandler {
    dptree::filter(
        |msg: Message, _bot: FABot, _data: Arc<Data>, _redis_con: MultiplexedConnection| {
            cfg!(debug_assertions)
                || msg.from().is_some_and(|user| {
                    user.username
                        .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
                })
        },
    )
    .filter_command::<MaintainerCommands>()
    .endpoint(maintainer_commands_handler)
}
