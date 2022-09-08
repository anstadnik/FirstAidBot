use super::logic::{move_to_state, process_broadcast, state_transition};
use crate::{bot::prelude::*, MAINTAINER_USERNAMES};
use anyhow::{bail, Context, Error};
use redis::aio::MultiplexedConnection;
use std::convert::TryInto;

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> anyhow::Result<Lang> {
    lang.as_str()
        .try_into()
        .map_err(Error::msg)
        .report_if_err(bot, msg.chat.id, &Lang::default(), None)
        .await
}

pub async fn start_endpoint(
    bot: FABot,
    msg: Message,
    data: Arc<Data>,
    dialogue: FADialogue,
    lang: String,
    mut conn: MultiplexedConnection,
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn(&bot, &msg, lang).await.unwrap_or_default();
    move_to_state(&bot, &msg, &dialogue, &data, Vec::new(), lang, &mut conn)
        .await
        .context("Error while moving into initial state")
        .report_if_err(&bot, msg.chat.id, &lang, None)
        .await
}

pub async fn handle_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: Arc<Data>,
    mut conn: MultiplexedConnection,
    (lang, context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let lang = match get_lang_or_warn(&bot, &msg, lang)
        .await
        .context("Unknown language {lang}")
        .report_if_err(&bot, msg.chat.id, &Lang::default(), None)
        .await
    {
        Ok(lang) => lang,
        Err(err) => {
            let lang = Lang::default();
            let context = Vec::new();
            move_to_state(&bot, &msg, &dialogue, &data, context, lang, &mut conn).await?;
            bail!(err)
        }
    };
    if let Err(err) = state_transition(&bot, &msg, &dialogue, &data, context, lang, &mut conn)
        .await
        .context("The state transition broke")
        .report_if_err(&bot, msg.chat.id, &lang, None)
        .await
    {
        move_to_state(&bot, &msg, &dialogue, &data, Vec::new(), lang, &mut conn).await?;
        bail!(err)
    }
    Ok(())
}

pub async fn broadcast_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: Arc<Data>,
    mut conn: MultiplexedConnection,
    (lang, message): (String, Option<String>),
) -> anyhow::Result<()> {
    let is_admin = msg.from().is_some_and(|user| {
        user.username
            .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
    });
    if !is_admin {
        let _ = bot
            .send_message(msg.chat.id, "WTF you are not an admin bye")
            .await;
        dialogue
            .update(State::Start {
                lang: Lang::default().to_string(),
            })
            .await?;
        return Ok(());
    }
    let lang = match get_lang_or_warn(&bot, &msg, lang)
        .await
        .context("Unknown language {lang}")
        .report_if_err(&bot, msg.chat.id, &Lang::default(), None)
        .await
    {
        Ok(lang) => lang,
        Err(err) => {
            let lang = Lang::default();
            let context = Vec::new();
            move_to_state(&bot, &msg, &dialogue, &data, context, lang, &mut conn).await?;
            bail!(err)
        }
    };
    process_broadcast(&bot, &msg, &dialogue,  message, lang, &mut conn).await?;
    // process_broadcast();
    Ok(())
}
