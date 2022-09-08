use crate::bot::prelude::*;
use anyhow::{bail, Context};
use redis::aio::MultiplexedConnection;
use teloxide::utils::markdown::code_block;

use super::logic::{move_to_state, state_transition};

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> anyhow::Result<Lang> {
    match lang.as_str().try_into() {
        Ok(lang) => Ok(lang),
        Err(err) => {
            bot.send_message(msg.chat.id, code_block(&err)).await?;
            bail!(err)
        }
    }
}

pub async fn start_handler(
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
        .report_if_err(&bot, msg.chat.id, &lang)
        .await
}

pub async fn handle_dialogue(
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
        .report_if_err(&bot, msg.chat.id, &Lang::default())
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
        .report_if_err(&bot, msg.chat.id, &lang)
        .await
    {
        move_to_state(&bot, &msg, &dialogue, &data, Vec::new(), lang, &mut conn).await?;
        bail!(err)
    }
    Ok(())
}
