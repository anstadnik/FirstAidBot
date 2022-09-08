use super::fa_logic::FALogic;
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

pub async fn start_handler(
    bot: FABot,
    msg: Message,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    dialogue: FADialogue,
    lang: String,
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn(&bot, &msg, lang).await.unwrap_or_default();
    let args = FALogic::new(bot, msg, dialogue, data);
    args.move_to_state(Vec::new(), lang, redis_con)
        .await
        .context("Error while moving into initial state")
        .report_if_err(&args.bot, args.msg.chat.id, &lang)
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
    let lang = match get_lang_or_warn(&bot, &msg, lang)
        .await
        .context("Unknown language {lang}")
        .report_if_err(&bot, msg.chat.id, &Lang::default())
        .await
    {
        Ok(lang) => lang,
        Err(err) => {
            FALogic::new(bot, msg, dialogue, data)
                .move_to_state(Vec::new(), Lang::default(), redis_con)
                .await?;
            bail!(err)
        }
    };
    let logic = FALogic::new(bot, msg, dialogue, data);
    if let Err(err) = logic
        .state_transition(context.clone(), lang, redis_con.clone())
        .await
        .context("The state transition broke")
        .report_if_err(&logic.bot, logic.msg.chat.id, &lang)
        .await
    {
        logic.move_to_state(Vec::new(), lang, redis_con).await?;
        bail!(err)
    }
    Ok(())
}
