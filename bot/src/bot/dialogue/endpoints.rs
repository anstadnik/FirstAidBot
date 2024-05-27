use super::commands::easter_egg;
use super::logic::{is_admin, move_to_state, process_broadcast, transition_logic};
use crate::bot::{FABot, FADialogue};
use crate::{DataGetState, DATA};
use anyhow::{bail, Result};
use first_aid_bot_core::prelude::*;
use rand::random;
use teloxide::{requests::Requester, types::Message};

pub async fn start_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    lang: Lang,
) -> Result<()> {
    if is_admin(&msg) && random::<u8>() % 50 == 0 {
        easter_egg(&bot, &msg).await?;
    }
    let ctx = FAContext {
        lang,
        ..FAContext::default()
    };
    move_to_state(&bot, &msg, &dialogue, &*DATA.get_state(&ctx).await?, ctx).await
}

pub async fn transition_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    (lang, context): (String, Vec<String>),
) -> Result<()> {
    let lang = lang.as_str().try_into()?;
    let f = || async { transition_logic(&bot, &msg, &dialogue, FAContext { lang, context }).await };

    if let Err(e) = f().await {
        start_endpoint(bot, msg, dialogue, lang).await?;
        bail!(e);
    }
    Ok(())
}

pub async fn broadcast_endpoint(
    bot: FABot,
    msg: Message,
    dialogue: FADialogue,
    message: Option<String>,
) -> Result<()> {
    if !is_admin(&msg) {
        let _ = bot
            .send_message(msg.chat.id, "WTF you are not an admin bye")
            .await;
        return start_endpoint(bot, msg, dialogue, Lang::default()).await;
    }
    process_broadcast(&bot, &msg, &dialogue, message).await?;
    Ok(())
}
