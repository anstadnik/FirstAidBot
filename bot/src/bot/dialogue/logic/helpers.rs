use super::keyboard::make_keyboard;
use crate::bot::report_error::ReportError;
use crate::bot::FABot;
use first_aid_bot_core::prelude::*;
use crate::MAINTAINER_USERNAMES;
use anyhow::{Result, Context};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;

pub async fn send_state(bot: &FABot, msg: &Message, state: &State) -> Result<()> {
    let id = msg.chat.id;
    if let Some(link) = &state.link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let keyboard = make_keyboard(&state.button_texts, state.lang, state.depth(), is_admin(msg));
    bot.send_message(id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    msg.from()
        .and_then(|user| user.username.as_ref())
        .filter(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
        .is_some()
}

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: String) -> Result<Lang> {
    lang.as_str()
        .try_into()
        .context("Unknown language {lang}")
        .report_if_err(bot, msg.chat.id, Lang::default().details().error)
        .await
}

pub async fn get_state_or_warn(
    bot: &FABot,
    data: Arc<Data>,
    msg: &Message,
    lang: Lang,
    ctx: &[String],
) -> Result<State> {
    data.get(ctx, lang)
        .await
        .report_if_err(bot, msg.chat.id, Lang::default().details().error)
        .await
}
