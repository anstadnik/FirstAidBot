use super::logic::{move_to_state, state_transition};
use crate::bot::prelude::*;
use anyhow::{bail, Context};
use redis::aio::MultiplexedConnection;
use teloxide::utils::markdown::code_block;

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> anyhow::Result<Lang> {
    match lang.as_str().try_into() {
        Ok(lang) => Ok(lang),
        Err(err) => {
            bot.send_message(msg.chat.id, code_block(&err)).await?;
            bail!(err)
        }
    }
}

pub struct FAMsgArgs<'a> {
    pub bot: &'a FABot,
    pub msg: &'a Message,
    pub dialogue: &'a FADialogue,
    pub data: &'a Arc<Data>,
    pub redis_con: MultiplexedConnection,
}

impl<'a> FAMsgArgs<'a> {
    pub const fn new(
        bot: &'a FABot,
        msg: &'a Message,
        dialogue: &'a FADialogue,
        data: &'a Arc<Data>,
        redis_con: MultiplexedConnection,
    ) -> Self {
        Self {
            bot,
            msg,
            dialogue,
            data,
            redis_con,
        }
    }
}

pub async fn start_handler(
    bot: FABot,
    msg: Message,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FADialogue,
    lang: String,
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn(&bot, &msg, lang).await.unwrap_or_default();
    let args = FAMsgArgs::new(&bot, &msg, &dialogue, &data, redis_con);
    move_to_state(&args, Vec::new(), lang)
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
    redis_con: MultiplexedConnection,
    (lang, context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let args = FAMsgArgs::new(&bot, &msg, &dialogue, &data, redis_con);
    let lang = match get_lang_or_warn(&bot, &msg, lang)
        .await
        .context("Unknown language {lang}")
        .report_if_err(&bot, msg.chat.id, &Lang::default())
        .await
    {
        Ok(lang) => lang,
        Err(err) => {
            move_to_state(&args, Vec::new(), Lang::default()).await?;
            bail!(err)
        }
    };
    if let Err(err) = state_transition(&args, context.clone(), lang)
        .await
        .context("The state transition broke")
        .report_if_err(&bot, msg.chat.id, &Lang::default())
        .await
    {
        move_to_state(&args, Vec::new(), lang).await?;
        bail!(err)
    }
    Ok(())
}
