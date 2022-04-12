use self::Lang::*;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Lang                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Lang {
    Ua,
    /* En,
    Ru, */
}

impl Lang {
    // https://github.com/rust-lang/rfcs/issues/284
    pub fn iter() -> impl Iterator<Item = Lang> {
        [Ua, /* En, Ru */].iter().copied()
    }
    pub fn details(self) -> LangDetails {
        match self {
            Ua => UA_LD,
            /* En => EN_LD,
            Ru => RU_LD, */
        }
    }
}

impl TryFrom<&str> for Lang {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ukrainian" => Ok(Ua),
            /* "English" => Ok(En),
            "Russian" => Ok(Ru), */
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

const UA_LD: LangDetails = LangDetails {
    name: "Ukrainian",
    button_text: "Українська",
    error: "Сталась помилка, будь ласка, повідомте про це у https://t.me/+SvnzzsxStydmNGI6",
    use_buttons_text: "Використайте кнопки, або перезавантажте бота (/start)",
    greeting: "Що трапилось?",
};

/* const EN_LD: LangDetails = LangDetails {
    name: "English",
    button_text: "English",
    error: "An error occured, please tell us about it at https://t.me/+SvnzzsxStydmNGI6",
    use_buttons_text: "Use buttons, or restart bot if something is wrong (/start)",
    greeting: "What happened?",
};

const RU_LD: LangDetails = LangDetails {
    name: "Russian",
    button_text: "Русский",
    error: "Произошла ошибка, пожалуйста, сообщине о ней в https://t.me/+SvnzzsxStydmNGI6",
    use_buttons_text: "Используйте кнопки, или перезагрузите бота, если что-то сломалось (/start)",
    greeting: "Что произошло?",
}; */
