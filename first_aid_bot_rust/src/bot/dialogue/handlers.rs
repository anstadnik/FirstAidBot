use super::prelude::*;
use crate::bot::{error_handler::send_err, keyboard::make_keyboard_from_state};
use anyhow::{anyhow, bail};
use async_recursion::async_recursion;
use redis::aio::MultiplexedConnection;
use teloxide::types::ParseMode;

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

#[async_recursion]
pub async fn move_to_state(
    args: &FAMsgArgs,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let FAMsgArgs {
        bot,
        msg,
        dialogue,
        data,
        redis_con,
    } = args;
    let state = &data.get(lang, &context).await?;
    log_to_redis(&args, &lang, &context).await;
    let keyboard = make_keyboard_from_state(state, lang, &context);
    send_state(bot, msg, state, lang, keyboard).await?;
    if state.next_states.is_empty() {
        return move_to_state(args, Vec::new(), lang).await;
    }
    let new_dialogue = State::Dialogue {
        lang: lang.name(),
        context,
    };
    dialogue.update(new_dialogue).await?;
    Ok(())
}

pub async fn handle_dialogue(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
    (lang, mut context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn(&bot, &msg, lang).await?;
    // TODO: Move to the initial state <12-07-22, astadnik> //
    // if let Err(err) = {
    let args = FAMsgArgs::new(&bot, &msg, &dialogue, &data, redis_con);
    let state = &data.get(lang, &context).await?;
    if let Err(err) = {
        log_to_redis(&args, &lang, &context).await;
        let mut context = context.clone();
        match msg.text() {
            Some(text) if text == lang.details().button_home => {
                move_to_state(&args, Vec::new(), lang).await?;
            }
            Some(text) if text == lang.details().button_back => {
                context.pop();
                move_to_state(&args, context, lang).await?;
            }
            Some(text)
                if context.is_empty()
                    && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
            {
                let lang = Lang::iter()
                    .find(|lang| lang.details().button_lang_name == text)
                    .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
                move_to_state(&args, Vec::new(), lang).await?;
            }
            Some(text) if state.next_states.contains_key(&text.to_string()) => {
                context.push(text.to_string());
                move_to_state(&args, context, lang).await?;
            }
            _ => {
                let keyboard = make_keyboard_from_state(state, lang, &context);
                #[allow(deprecated)]
                bot.send_message(msg.chat.id, lang.details().use_buttons_text)
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(keyboard)
                    .await?;
            }
        }
        anyhow::Ok(())
    } {
        send_err(&bot, msg.chat.id, &lang, err.to_string()).await;
        // reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await?;
        move_to_state(&args, context, lang);
        bail!(err)
    }
    Ok(())
}
