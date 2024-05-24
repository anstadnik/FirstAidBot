use crate::HELP_CHAT_URL;
use anyhow::anyhow;
use const_format::concatcp;
use std::fmt::Display;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Lang                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum Lang {
    #[default]
    // Ua,
    En,
    // Ru
}

impl Lang {
    // https://github.com/rust-lang/rfcs/issues/284
    pub fn iter() -> impl Iterator<Item = Self> {
        // [Lang::Ua].into_iter()
        [Lang::En].into_iter()
    }
    pub const fn details(self) -> LangDetails {
        match self {
            // Lang::Ua => UA_LD,
            Lang::En => EN_LD,
            // Lang::Ru => RU_LD,
        }
    }
    pub fn name(self) -> String {
        self.details().name.to_string()
    }
}

impl TryFrom<&str> for Lang {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Lang::iter()
            .find(|lang| lang.name() == value)
            .ok_or_else(|| {
                log::error!("Cannot create a language from {value}");
                anyhow!(
                    "Error, unknown language. If issue persists, ask for help in {HELP_CHAT_URL}"
                )
            })
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           LangDetails                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LangDetails {
    pub name: &'static str,
    pub button_lang: &'static str,
    pub button_back: &'static str,
    pub button_home: &'static str,
    pub error: &'static str,
    pub error_due_to_update: &'static str,
    pub use_buttons_text: &'static str,
    pub greeting: &'static str,
    pub broadcast: &'static str,
}

// const UA_LD: LangDetails = LangDetails {
//     name:  "Ukrainian",
//     button_lang: "Українська",
//     button_back: "◀️ Повернутись",
//     button_home: "◀️ На початок",
//     error: concatcp!(
//         "Сталась помилка :( Перезапустіть бота (/start), і якщо це не допоможе, будь ласка, повідомте про це у ",
//         HELP_CHAT_URL
//     ),
//     error_due_to_update: concatcp!(
//         "Схоже, що бота було оновлено. Повертаюсь на початок. Якщо бот далі не працює, будь ласка, повідомте про це у ",
//         HELP_CHAT_URL
//     ),
//     use_buttons_text: "Використайте кнопки, або перезавантажте бота \\(/start\\)",
//     // TODO: Change it <26-06-22, astadnik> //
//     greeting: "Що трапилось?",
//     broadcast: "Транслювати",
// };

const EN_LD: LangDetails = LangDetails {
    name: "English",
    button_lang: "English",
    button_back: "◀️ Back",
    button_home: "◀️ Home",
    error: concatcp!(
        "An error occurred :( Restart the bot (/start), and if that doesn't help, please report it in ",
        HELP_CHAT_URL
    ),
    error_due_to_update: concatcp!(
        "It seems that the bot has been updated. Returning to the beginning. If the bot still doesn't work, please report it in ",
        HELP_CHAT_URL
    ),
    use_buttons_text: "Use buttons, or restart bot if something is wrong (/start)",
    greeting: "What happened?",
    broadcast: "Broadcast",
};

// const RU_LD: LangDetails = LangDetails {
//     name: "Russian",
//     button_lang_name: "Русский",
//     error: "Произошла ошибка, пожалуйста, сообщине о ней в " + HELP_CHAT_URL,
//     use_buttons_text: "Используйте кнопки, или перезагрузите бота, если что-то сломалось (/start)",
//     greeting: "Что произошло?",
// };
