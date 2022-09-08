use super::helpers::send_state;
use super::log_to_redis::log_to_redis;
use super::process_broadcast;
use crate::{bot::prelude::*, MAINTAINER_USERNAMES};
use anyhow::{anyhow, Context};
use redis::aio::MultiplexedConnection;
use teloxide::utils::markdown::{escape, escape_code};

pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    data: &Arc<Data>,
    context: Vec<String>,
    lang: Lang,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    let state = &data.get(lang, &context).await?;
    log_to_redis(msg, &lang, &context, conn).await;
    send_state(bot, msg, state, lang, &context).await?;
    if state.next_states.is_empty() {
        let context = Vec::new();

        let state = &data.get(lang, &context).await?;
        log_to_redis(msg, &lang, &context, conn).await;
        send_state(bot, msg, state, lang, &context).await?;
        let lang = lang.name();
        dialogue.update(State::Dialogue { lang, context }).await?;
        return Ok(());
    }
    let lang = lang.name();
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}

pub async fn state_transition(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    data: &Arc<Data>,
    mut context: Vec<String>,
    lang: Lang,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    let state = &match data.get(lang, &context).await {
        Ok(it) => it,
        Err(_) => {
            bot.send_message(msg.chat.id, escape_code(lang.details().error_due_to_update))
                .await?;
            return move_to_state(bot, msg, dialogue, data, Vec::new(), lang, conn).await;
        }
    };
    log_to_redis(msg, &lang, &context, conn).await;
    let is_admin = msg.from().is_some_and(|user| {
        user.username
            .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
    });
    match msg.text() {
        Some(text) if text == lang.details().button_home => {
            move_to_state(bot, msg, dialogue, data, Vec::new(), lang, conn).await?;
        }
        Some(text) if text == lang.details().button_back => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, context, lang, conn).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_lang_name == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
            move_to_state(bot, msg, dialogue, data, Vec::new(), lang, conn).await?;
        }
        Some(text) if state.next_states.contains_key(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, context.clone(), lang, conn)
                .await
                .with_context(|| format!("Error while moving into context {context:?}"))?;
        }
        Some(text) if text == lang.details().broadcast && is_admin => {
            dialogue
                .update(State::Broadcast {
                    lang: lang.to_string(),
                    message: None,
                })
                .await?;
            process_broadcast(bot, msg, dialogue, None, lang, conn).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, escape(lang.details().use_buttons_text))
                .await?;
            move_to_state(bot, msg, dialogue, data, context, lang, conn).await?;
        }
    }
    Ok(())
}
