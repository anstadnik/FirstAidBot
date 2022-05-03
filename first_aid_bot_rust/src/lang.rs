use const_format::concatcp;

use crate::HELP_CHAT_URL;

use self::Lang::*;

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
    pub fn iter() -> impl Iterator<Item = Lang> {
        [Ua /* En, Ru */].iter().copied()
    }
    pub fn details(self) -> LangDetails {
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
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            UA_STR => Ok(Ua),
            /* EN_STR => Ok(En),
            RU_STR => Ok(Ru), */
            wtf => Err(format!("Cannot create a language from {wtf}")),
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Ua
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           LangDetails                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct LangDetails {
    pub name: &'static str,
    pub button_text: &'static str,
    pub error: &'static str,
    pub use_buttons_text: &'static str,
    pub greeting: &'static str,
}

const UA_STR: &str = "Ukrainian";
const UA_LD: LangDetails = LangDetails {
    name: UA_STR,
    button_text: "Українська",
    error: concatcp!(
        "Сталась помилка, будь ласка, повідомте про це у ",
        HELP_CHAT_URL
    ),
    use_buttons_text: "Використайте кнопки, або перезавантажте бота (/start)",
    greeting: "Що трапилось?",
};

// const EN_STR: &str = "Ukrainian";
// const EN_LD: LangDetails = LangDetails {
//     name: EN_STR,
//     button_text: "English",
//     error: "An error occured, please tell us about it at " + HELP_CHAT_URL,
//     use_buttons_text: "Use buttons, or restart bot if something is wrong (/start)",
//     greeting: "What happened?",
// };
//
// const RU_STR: &str = "Ukrainian";
// const RU_LD: LangDetails = LangDetails {
//     name: RU_STR,
//     button_text: "Русский",
//     error: "Произошла ошибка, пожалуйста, сообщине о ней в " + HELP_CHAT_URL,
//     use_buttons_text: "Используйте кнопки, или перезагрузите бота, если что-то сломалось (/start)",
//     greeting: "Что произошло?",
// };
