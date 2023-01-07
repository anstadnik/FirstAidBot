use crate::bot::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn make_keyboard(keys: &Vec<String>, lang: Lang, depth: usize, is_admin: bool) -> ReplyMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    if keys.is_empty() && !is_admin {
        return ReplyMarkup::kb_remove();
    }

    for key_texts in keys.chunks(2) {
        keyboard.push(key_texts.iter().map(KeyboardButton::new).collect());
    }

    if is_admin {
        keyboard.push(vec![KeyboardButton::new(lang.details().broadcast)]);
    }

    let special_keys = if depth == 0 {
        let f = |lang: Lang| KeyboardButton::new(lang.details().button_lang);
        Lang::iter().filter(|&l| l != lang).map(f).collect()
    } else {
        let mut special_keys = vec![KeyboardButton::new(lang.details().button_back)];
        if depth > 1 {
            special_keys.push(KeyboardButton::new(lang.details().button_home));
        };
        special_keys
    };
    keyboard.push(special_keys);

    ReplyMarkup::Keyboard(KeyboardMarkup::new(keyboard).resize_keyboard(true))
}
