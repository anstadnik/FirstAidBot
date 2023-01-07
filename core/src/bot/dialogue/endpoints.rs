use super::commands::easter_egg;
use super::logic::{is_admin, move_to_state, process_broadcast, state_transition};
use crate::bot::prelude::*;
use anyhow::{bail, Context};
use rand::random;
use redis::aio::MultiplexedConnection;
use std::convert::TryInto;

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> anyhow::Result<Lang> {
    lang.as_str()
        .try_into()
        .report_if_err(bot, msg.chat.id, Lang::default().details().error)
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
    if is_admin(&msg) && random::<u8>() % 50 == 0 {
        easter_egg(&bot, &msg).await?;
    }
    move_to_state(&bot, &msg, &dialogue, &data, Vec::new(), lang, &mut conn)
        .await
        .context("Error while moving into initial state")
        .report_if_err(&bot, msg.chat.id, lang.details().error)
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
        .report_if_err(&bot, msg.chat.id, Lang::default().details().error)
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
        .report_if_err(&bot, msg.chat.id, lang.details().error)
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
    _: Arc<Data>,
    mut conn: MultiplexedConnection,
    message: Option<String>,
) -> anyhow::Result<()> {
    if !is_admin(&msg) {
        std::mem::drop(bot
            .send_message(msg.chat.id, "WTF you are not an admin bye")
            .await);
        let lang = Lang::default().to_string();
        dialogue.update(State::Start { lang }).await?;
        return Ok(());
    }
    process_broadcast(&bot, &msg, &dialogue, message, &mut conn).await?;
    Ok(())
}
