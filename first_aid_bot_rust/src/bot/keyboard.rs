use super::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn make_keyboard(keys: &[String], lang: Lang, context: &[String]) -> ReplyMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    if keys.is_empty() {
        return ReplyMarkup::kb_remove();
    }

    for key_texts in keys.chunks(2) {
        let row = key_texts.iter().map(KeyboardButton::new).collect();
        keyboard.push(row);
    }

    if !context.is_empty() {
        let mut special_keys = vec![KeyboardButton::new(lang.details().button_back)];
        if context.len() > 1 {
            special_keys.push(KeyboardButton::new(lang.details().button_home));
        };
        keyboard.push(special_keys);
    } else {
        let special_keys = Lang::iter()
            .filter(|&l| l != lang)
            .map(|lang| KeyboardButton::new(lang.details().button_lang_name))
            .collect();
        keyboard.push(special_keys);
    }

    ReplyMarkup::Keyboard(KeyboardMarkup::new(keyboard).resize_keyboard(true))
}
