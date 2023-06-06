use crate::bot::FABot;
use crate::bot::FADialogue;
use first_aid_bot_core::prelude::*;

use super::commands::easter_egg;
use super::logic::get_fs_or_warn;
use super::logic::get_lang_or_warn;
use super::logic::{is_admin, move_to_state, process_broadcast, state_transition};
use anyhow::bail;
use rand::random;
use redis::aio::MultiplexedConnection;
use teloxide::requests::Requester;
use teloxide::types::Message;

pub async fn start_endpoint(
    bot: FABot,
    msg: Message,
    data: &'static Data,
    dialogue: FADialogue,
    lang: &str,
    mut conn: MultiplexedConnection,
) -> anyhow::Result<()> {
    if is_admin(&msg) && random::<u8>() % 50 == 0 {
        easter_egg(&bot, &msg).await?;
    }
    let lang = get_lang_or_warn(&bot, &msg, lang).await.unwrap_or_default();
    let ctx = FAContext {
        lang,
        context: Vec::new(),
    };
    let fs = get_fs_or_warn(&bot, data, &msg, &ctx).await?;
    move_to_state(&bot, &msg, &dialogue, &fs, ctx, &mut conn).await
}

pub async fn handle_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: &'static Data,
    conn: MultiplexedConnection,
    (lang, context): (String, Vec<String>),
) -> anyhow::Result<()> {
    async fn f(
        bot: &FABot,
        msg: &Message,
        dialogue: &FADialogue,
        data: &'static Data,
        mut conn: MultiplexedConnection,
        (lang, context): (&str, Vec<String>),
    ) -> anyhow::Result<()> {
        let lang = get_lang_or_warn(bot, msg, lang).await?;
        let ctx = FAContext{lang, context};
        // let fs = get_fs_or_warn(bot, data.clone(), msg, &ctx).await?;
        state_transition(bot, msg, dialogue, ctx, data, &mut conn).await
    }

    if let Err(e) = f(&bot, &msg, &dialogue, data, conn.clone(), (&lang, context)).await {
        start_endpoint(bot, msg, data, dialogue, &lang, conn).await?;
        // let fs = Default::default();
        // move_to_state(&bot, &msg, &dialogue, fs, &mut conn).await?;
        bail!(e);
    }
    Ok(())
}

pub async fn broadcast_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    _: &'static Data,
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
