use crate::bot::prelude::*;
use anyhow::{anyhow, Context};
use async_recursion::async_recursion;
use teloxide::utils::markdown::escape;

use super::{handlers::FAMsgArgs, helpers::{log_to_redis, send_state}};

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
        ..
    } = args;
    let state = &data.get(lang, &context).await?;
    log_to_redis(args, &lang, &context).await;
    send_state(bot, msg.chat.id, state, lang, &context).await?;
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

pub async fn state_transition(
    args: &FAMsgArgs<'_>,
    mut context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = &match args.data.get(lang, &context).await {
        Ok(it) => it,
        Err(_) => {
            send_plain_string(
                args.bot,
                args.msg.chat.id,
                lang.details().error_due_to_update,
            )
            .await?;
            return move_to_state(args, Vec::new(), lang).await;
        }
    };
    log_to_redis(args, &lang, &context).await;
    match args.msg.text() {
        Some(text) if text == lang.details().button_home => {
            move_to_state(args, Vec::new(), lang).await?;
        }
        Some(text) if text == lang.details().button_back => {
            context.pop();
            move_to_state(args, context, lang).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_lang_name == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
            move_to_state(args, Vec::new(), lang).await?;
        }
        Some(text) if state.next_states.contains_key(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(args, context.clone(), lang)
                .await
                .with_context(|| format!("Error while moving into context {context:?}"))?;
        }
        _ => {
            args.bot
                .send_message(args.msg.chat.id, escape(lang.details().use_buttons_text))
                .await?;
            move_to_state(args, context, lang).await?;
        }
    }
    anyhow::Ok(())
}
