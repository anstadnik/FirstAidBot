use super::log_to_redis::log_to_redis;
use super::process_broadcast;
use super::{helpers::send_state, is_admin};
use crate::State;
use crate::bot::prelude::*;
use anyhow::{anyhow, Context};
use redis::aio::MultiplexedConnection;
use teloxide::utils::markdown::escape;

pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    data: &Arc<Data>,
    // mut context: Vec<String>,
    // lang: Lang,
    state: State,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    // let map_err = || format!("Error while moving into context {context:?}");
    // let state = &data.get(lang, &context).await.with_context(map_err)?;
    if let Err(err) = log_to_redis(msg, &state.lang, &state.context, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };
    send_state(bot, msg, state, data).await?;
    if state.next_states.is_empty() {
        context = Vec::new();

        let state = &data.get(lang, &context).await?;
        if let Err(err) = log_to_redis(msg, &lang, &context, conn).await {
            log::error!("Cannot log to redis: {err:?}");
        };
        send_state(bot, msg, state, lang, &context).await?;
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
    mut ctx: Vec<String>,
    mut lang: Lang,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    let state = &if let Ok(it) = data.get(lang, &ctx).await {
        it
    } else {
        bot.send_message(msg.chat.id, escape(lang.details().error_due_to_update))
            .await?;
        return move_to_state(bot, msg, dialogue, data, Vec::new(), lang, conn).await;
    };
    if let Err(err) = log_to_redis(msg, &lang, &ctx, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };

    match msg.text() {
        Some(text) if text == lang.details().button_home => ctx = Vec::new(),
        Some(text) if text == lang.details().button_back => drop(ctx.pop()),
        Some(text) if state.next_states.contains_key(text) => ctx.push(text.to_string()),
        Some(text) if ctx.is_empty() && Lang::iter().any(|l| l.details().button_lang == text) => {
            lang = Lang::iter()
                .find(|lang| lang.details().button_lang == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
        }
        Some(text) if text == lang.details().broadcast && is_admin(msg) => {
            dialogue.update(State::Broadcast { message: None }).await?;
            return process_broadcast(bot, msg, dialogue, None, conn).await;
        }
        _ => {
            bot.send_message(msg.chat.id, lang.details().use_buttons_text)
                .await?;
            return Ok(());
        }
    }
    move_to_state(bot, msg, dialogue, data, ctx, lang, conn).await
}
