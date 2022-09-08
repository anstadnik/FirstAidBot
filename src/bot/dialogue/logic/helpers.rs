use super::keyboard::make_keyboard_from_state;
use crate::bot::prelude::*;
use teloxide::types::ParseMode;

pub async fn send_state(
    bot: &FABot,
    id: ChatId,
    state: &FS,
    lang: Lang,
    context: &[String],
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let keyboard = make_keyboard_from_state(state, lang, context);
    bot.send_message(id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
