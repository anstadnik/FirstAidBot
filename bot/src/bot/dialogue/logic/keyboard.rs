use first_aid_bot_core::prelude::Lang;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn make_keyboard(keys: &[&str], lang: Lang, depth: usize, is_admin: bool) -> ReplyMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    if keys.is_empty() && !is_admin {
        return ReplyMarkup::kb_remove();
    }

    for key_texts in keys.chunks(2) {
        keyboard.push(key_texts.iter().copied().map(KeyboardButton::new).collect());
    }

    if is_admin {
        keyboard.push(vec![KeyboardButton::new(lang.details().broadcast)]);
    }

    let ld = lang.details();
    let special_keys = match depth {
        0 => Lang::iter()
            .filter(|&l| l != lang)
            .map(|l| l.details().button_lang)
            .collect(),
        1 => vec![ld.button_back],
        2.. => vec![ld.button_back, ld.button_home],
    }
    .into_iter()
    .map(KeyboardButton::new)
    .collect();
    keyboard.push(special_keys);

    ReplyMarkup::Keyboard(KeyboardMarkup::new(keyboard).resize_keyboard(true))
}
