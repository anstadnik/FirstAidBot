use super::commands::easter_egg;
use super::logic::{is_admin, move_to_state, process_broadcast, state_transition};
use crate::bot::{FABot, FADialogue, State};
use anyhow::{bail, Result};
use first_aid_bot_core::prelude::*;
use rand::random;
use redis::aio::MultiplexedConnection;
use teloxide::{requests::Requester, types::Message};

pub async fn start_endpoint(
    bot: FABot,
    msg: Message,
    data: &'static Data,
    dialogue: FADialogue,
    mut conn: MultiplexedConnection,
) -> Result<()> {
    if is_admin(&msg) && random::<u8>() % 50 == 0 {
        easter_egg(&bot, &msg).await?;
    }
    let ctx = FAContext::default();
    let fs = data.get().await?.get_state(&ctx)?;
    move_to_state(&bot, &msg, &dialogue, &fs, ctx, &mut conn).await
}

pub async fn handle_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    data: &'static Data,
    mut conn: MultiplexedConnection,
    (lang, context): (String, Vec<String>),
) -> Result<()> {
    let f = || async {
        let lang = lang.as_str().try_into()?;
        let ctx = FAContext { lang, context };
        state_transition(&bot, &msg, &dialogue, ctx, data, &mut conn).await
    };

    if let Err(e) = f().await {
        start_endpoint(bot, msg, data, dialogue, conn).await?;
        bail!(e);
    }
    Ok(())
}

pub async fn broadcast_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    mut conn: MultiplexedConnection,
    message: Option<String>,
) -> Result<()> {
    if !is_admin(&msg) {
        let _ = bot
            .send_message(msg.chat.id, "WTF you are not an admin bye")
            .await;
        dialogue.update(State::Start).await?;
        return Ok(());
    }
    process_broadcast(&bot, &msg, &dialogue, message, &mut conn).await?;
    Ok(())
}
