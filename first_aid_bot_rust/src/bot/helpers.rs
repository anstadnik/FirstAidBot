use crate::lang::Lang;
use crate::model::prelude::*;
use anyhow::bail;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ParseMode};
use teloxide::{adaptors::DefaultParseMode, payloads::SendMessageSetters, prelude2::*};

pub const GO_TO_BEGINNING_TEXT: &str = "◀️ На початок";
pub const GO_BACK_TEXT: &str = "◀️ Повернутись";

pub fn get_state<'a>(data: &'a FiniteState, context: &[String]) -> &'a FiniteState {
    let mut current_state = data;
    for choise in context {
        current_state = &current_state.options.as_ref().unwrap().next_states[choise];
    }
    current_state
}

#[derive(Debug, Clone)]
pub struct ExtraKeys {
    back: bool,
    home: bool,
    select_lang: Option<Lang>,
}

impl ExtraKeys {
    pub fn new(context: &[String], select_lang: Option<Lang>) -> ExtraKeys {
        ExtraKeys {
            back: !context.is_empty(),
            home: context.len() > 1,
            select_lang,
        }
    }
    pub fn empty(select_lang: Lang) -> ExtraKeys {
        ExtraKeys {
            back: false,
            home: false,
            select_lang: Some(select_lang),
        }
    }

    fn get_special_keys(&self) -> Option<Vec<KeyboardButton>> {
        let mut special_keys = Vec::new();
        if self.back {
            special_keys.push(KeyboardButton::new(GO_BACK_TEXT));
        };
        if self.home {
            special_keys.push(KeyboardButton::new(GO_TO_BEGINNING_TEXT));
        };
        if let Some(current_lang) = self.select_lang {
            for lang in Lang::iter() {
                if lang != current_lang {
                    special_keys.push(KeyboardButton::new(lang.details().button_text));
                }
            }
        };
        Some(special_keys).filter(|keys| !keys.is_empty())
    }
}

pub fn make_keyboard(ordered_keys: &[String], extra_keys: ExtraKeys) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for key_texts in ordered_keys.chunks(2) {
        let row = key_texts.iter().map(KeyboardButton::new).collect();
        keyboard.push(row);
    }
    if let Some(special_keys) = extra_keys.get_special_keys() {
        keyboard.push(special_keys);
    }

    KeyboardMarkup::new(keyboard).resize_keyboard(true)
}

pub async fn send_message(
    bot: &AutoSend<DefaultParseMode<Bot>>,
    msg: &Message,
    state: &FiniteState,
    extra_keys: ExtraKeys,
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(msg.chat.id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let sent_message = bot.send_message(msg.chat.id, &state.message);
    let rez = if let Some(options) = &state.options {
        sent_message.reply_markup(make_keyboard(&options.ordered_keys, extra_keys))
    } else {
        sent_message
    }
    .await;

    #[allow(deprecated)]
    if let Err(err) = rez {
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
        bail!(err);
    }
    Ok(())
}
