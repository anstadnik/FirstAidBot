use crate::bot::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn make_keyboard(keys: &Vec<String>, lang: Lang, depth: usize, is_admin: bool) -> ReplyMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    if keys.is_empty() && !is_admin {
        return ReplyMarkup::kb_remove();
    }

    for key_texts in keys.chunks(2) {
        let row = key_texts.iter().map(KeyboardButton::new).collect();
        keyboard.push(row);
    }

    if is_admin {
        keyboard.push(vec![KeyboardButton::new(lang.details().broadcast)]);
    }

    if depth != 0 {
        let mut special_keys = vec![KeyboardButton::new(lang.details().button_back)];
        if depth > 1 {
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

pub fn make_keyboard_from_state(
    state: &FS,
    lang: Lang,
    depth: usize,
    is_admin: bool,
) -> ReplyMarkup {
    make_keyboard(
        &state.next_states.keys().cloned().collect(),
        lang,
        depth,
        is_admin,
    )
}
