use super::prelude::*;
use crate::bot::error_handler::send_err;
use anyhow::bail;
use redis::aio::MultiplexedConnection;

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> anyhow::Result<Lang> {
    match lang.as_str().try_into() {
        Ok(lang) => Ok(lang),
        Err(err) => {
            send_plain_string(bot, msg.chat.id, &err).await?;
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
    pub fn new(
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
    if let Err(err) = { move_to_state(&args, Vec::new(), lang).await } {
        send_err(&bot, msg.chat.id, &lang, err.to_string()).await;
        bail!(err)
    }
    Ok(())
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
    let lang = match get_lang_or_warn(&bot, &msg, lang).await {
        Ok(lang) => lang,
        Err(err) => {
            send_err(&bot, msg.chat.id, &Lang::default(), err.to_string()).await;
            move_to_state(&args, Vec::new(), Lang::default());
            bail!(err)
        }
    };
    if let Err(err) = { state_transition(&args, context.clone(), lang).await } {
        send_err(&bot, msg.chat.id, &lang, err.to_string()).await;
        move_to_state(&args, Vec::new(), lang);
        bail!(err)
    }
    Ok(())
}
