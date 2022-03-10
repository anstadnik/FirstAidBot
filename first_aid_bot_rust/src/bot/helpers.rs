use anyhow::Error;
use teloxide::{
    adaptors::DefaultParseMode,
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

#[derive(Debug, Clone)]
pub struct KeyboardOptions {
    back_button: bool,
    home_button: bool,
}

impl KeyboardOptions {
    pub fn new(context: &[String]) -> KeyboardOptions {
        KeyboardOptions {
            back_button: !context.is_empty(),
            home_button: context.len() > 1,
        }
    }
    pub fn empty() -> KeyboardOptions {
        KeyboardOptions::new(&[])
    }

    fn get_special_keys(&self) -> Option<Vec<KeyboardButton>> {
        if !(self.home_button || self.back_button) {
            return None;
        }
        let mut special_keys = Vec::new();
        if self.back_button {
            special_keys.push(KeyboardButton::new(GO_BACK_TEXT));
        };
        if self.home_button {
            special_keys.push(KeyboardButton::new(GO_TO_BEGINNING_TEXT));
        };
        Some(special_keys)
    }
}

pub async fn make_keyboard(
    ordered_keys: &[String],
    keyboard_options: KeyboardOptions,
) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for key_texts in ordered_keys.chunks(2) {
        let row = key_texts.iter().map(KeyboardButton::new).collect();
        keyboard.push(row);
    }
    if let Some(special_keys) = keyboard_options.get_special_keys() {
        keyboard.push(special_keys);
    }

    KeyboardMarkup::new(keyboard).resize_keyboard(true)
}

pub async fn send_message(
    bot: &AutoSend<DefaultParseMode<Bot>>,
    msg: &Message,
    state: &FiniteState,
    keyboard_options: KeyboardOptions
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(msg.chat.id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }
    let sent_message = bot.send_message(msg.chat.id, &state.message);
    #[allow(deprecated)]
    if let Err(err) = if let Some(options) = &state.options {
        sent_message.reply_markup(
            make_keyboard(&options.ordered_keys, keyboard_options).await,
        )
    } else {
        sent_message
    }
    .await
    {
        bot.send_message(
            msg.chat.id,
            "Сталась помилка, будь ласка, повідомте про це у https://t.me/+SvnzzsxStydmNGI6.",
        )
        .parse_mode(ParseMode::Markdown)
        .await?;
        bot.send_message(msg.chat.id, format!("{err:#?}"))
            .parse_mode(ParseMode::Markdown)
            .await?;
        bot.send_message(msg.chat.id, &state.message)
            .parse_mode(ParseMode::Markdown)
            .await?;
        return Err(Error::new(err));
    }
    Ok(())
}
