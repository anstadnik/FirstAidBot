use teloxide::{
    payloads::SendMessageSetters,
    prelude2::*,
    types::{KeyboardButton, KeyboardMarkup, ParseMode},
};

use crate::model::FiniteState;

pub const GO_TO_BEGINNING_TEXT: &str = "◀️ На початок";
pub const GO_BACK_TEXT: &str = "◀️ Повернутись";

pub async fn get_state<'a>(data: &'a FiniteState, context: &[String]) -> &'a FiniteState {
    let mut current_state = data;
    for choise in context {
        current_state = &current_state.options.as_ref().unwrap().next_states[choise];
    }
    current_state
}

pub async fn make_keyboard(ordered_keys: &[String]) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for key_texts in ordered_keys.chunks(2) {
        let row = key_texts.iter().map(KeyboardButton::new).collect();

        keyboard.push(row);
    }
    keyboard.push(vec![
        KeyboardButton::new(GO_BACK_TEXT),
        KeyboardButton::new(GO_TO_BEGINNING_TEXT),
    ]);

    KeyboardMarkup::new(keyboard)
}

pub async fn send_message(
    bot: &AutoSend<Bot>,
    msg: &Message,
    state: &FiniteState,
) -> anyhow::Result<()> {
    // TODO: Hide the link <28-02-22, astadnik> //
    if let Some(link) = &state.link {
        bot.send_message(msg.chat.id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }
    let msg = bot.send_message(msg.chat.id, &state.message);
    if let Some(options) = &state.options {
        msg.reply_markup(make_keyboard(&options.ordered_keys).await)
    } else {
        msg
    }
    .await?;
    Ok(())
}
