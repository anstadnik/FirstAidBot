use crate::bot::FABot;
use crate::bot::FADialogue;
use crate::prelude::{Data, Lang};

use super::commands::easter_egg;
use super::logic::get_lang_or_warn;
use super::logic::get_state_or_warn;
use super::logic::{is_admin, move_to_state, process_broadcast, state_transition};
use anyhow::bail;
use rand::random;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use teloxide::requests::Requester;
use teloxide::types::Message;

pub async fn start_endpoint(
    bot: FABot,
    msg: Message,
    data: Arc<Data>,
    dialogue: FADialogue,
    lang: String,
    mut conn: MultiplexedConnection,
) -> anyhow::Result<()> {
    if is_admin(&msg) && random::<u8>() % 50 == 0 {
        easter_egg(&bot, &msg).await?;
    }
    let lang = get_lang_or_warn(&bot, &msg, lang).await.unwrap_or_default();
    let state = get_state_or_warn(&bot, data, &msg, lang, &[])
        .await
        .unwrap_or_default();
    move_to_state(&bot, &msg, &dialogue, state, &mut conn).await
}

pub async fn handle_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: Arc<Data>,
    mut conn: MultiplexedConnection,
    (lang, context): (String, Vec<String>),
) -> anyhow::Result<()> {
    async fn f(
        bot: &FABot,
        msg: &Message,
        dialogue: &FADialogue,
        data: Arc<Data>,
        mut conn: MultiplexedConnection,
        (lang, context): (String, Vec<String>),
    ) -> anyhow::Result<()> {
        let lang = get_lang_or_warn(bot, msg, lang).await?;
        let state = get_state_or_warn(bot, data.clone(), msg, lang, &context).await?;
        state_transition(bot, msg, dialogue, state, data, &mut conn).await
    }

    if let Err(e) = f(&bot, &msg, &dialogue, data, conn.clone(), (lang, context)).await {
        let state = Default::default();
        move_to_state(&bot, &msg, &dialogue, state, &mut conn).await?;
        bail!(e);
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
        std::mem::drop(
            bot.send_message(msg.chat.id, "WTF you are not an admin bye")
                .await,
        );
        let lang = Lang::default().to_string();
        dialogue
            .update(crate::bot::state::State::Start { lang })
            .await?;
        return Ok(());
    }
    process_broadcast(&bot, &msg, &dialogue, message, &mut conn).await?;
    Ok(())
}
