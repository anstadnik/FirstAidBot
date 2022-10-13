use self::Lang::*;
use crate::HELP_CHAT_URL;
use anyhow::bail;
use const_format::concatcp;
use std::fmt::Display;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Lang                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Lang {
    Ua,
    /* En,
    Ru, */
}

impl Lang {
    // https://github.com/rust-lang/rfcs/issues/284
    pub fn iter() -> impl Iterator<Item = Self> {
        [Ua /* En, Ru */].iter().copied()
    }
    pub const fn details(self) -> LangDetails {
        match self {
            Ua => UA_LD,
            /* En => EN_LD,
            Ru => RU_LD, */
        }
    }
    pub fn name(self) -> String {
        self.details().name.to_string()
    }
}

impl TryFrom<&str> for Lang {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            UA_STR => Ok(Ua),
            /* EN_STR => Ok(En),
            RU_STR => Ok(Ru), */
            unknown_lang => {
                log::error!("Cannot create a language from {unknown_lang}");
                bail!("Error, unknown language. If issue persists, ask for help in {HELP_CHAT_URL}")
            }
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Ua
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
    pub button_lang_name: &'static str,
    pub button_back: &'static str,
    pub button_home: &'static str,
    pub error: &'static str,
    pub error_due_to_update: &'static str,
    pub use_buttons_text: &'static str,
    pub greeting: &'static str,
    pub broadcast: &'static str,
}

const UA_STR: &str = "Ukrainian";
const UA_LD: LangDetails = LangDetails {
    name: UA_STR,
    button_lang_name: "Українська",
    button_back: "◀️ Повернутись",
    button_home: "◀️ На початок",
    error: concatcp!(
        "Сталась помилка :\\( Перезапустіть бота \\(/start\\), і якщо це не допоможе, будь ласка, повідомте про це у ",
        HELP_CHAT_URL
    ),
    error_due_to_update: concatcp!(
        "Схоже, що бота було оновлено. Повертаюсь на початок. Якщо бот далі не працює, будь ласка, повідомте про це у ",
        HELP_CHAT_URL
    ),
    use_buttons_text: "Використайте кнопки, або перезавантажте бота \\(/start\\)",
    // TODO: Change it <26-06-22, astadnik> //
    greeting: "Що трапилось?",
    broadcast: "Транслювати",
};

// const EN_STR: &str = "Ukrainian";
// const EN_LD: LangDetails = LangDetails {
//     name: EN_STR,
//     button_lang_name: "English",
//     error: "An error occured, please tell us about it at " + HELP_CHAT_URL,
//     use_buttons_text: "Use buttons, or restart bot if something is wrong (/start)",
//     greeting: "What happened?",
// };
//
// const RU_STR: &str = "Ukrainian";
// const RU_LD: LangDetails = LangDetails {
//     name: RU_STR,
//     button_lang_name: "Русский",
//     error: "Произошла ошибка, пожалуйста, сообщине о ней в " + HELP_CHAT_URL,
//     use_buttons_text: "Используйте кнопки, или перезагрузите бота, если что-то сломалось (/start)",
//     greeting: "Что произошло?",
// };
