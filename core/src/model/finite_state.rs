use super::lang::Lang;
use anyhow::anyhow;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           CSV entry                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct Row {
    pub key: String,
    pub question: String,
    pub answer: String,
    pub link: Option<String>,
}

impl Row {
    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
            && self.question.is_empty()
            && self.answer.is_empty()
            && self.link.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       Finite State types                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub type MultilangStates = HashMap<Lang, FS>;

#[derive(Debug, Clone)]
pub struct FS {
    pub link: Option<String>,
    pub message: String,
    pub next_states: IndexMap<String, FS>,
}

fn parse_link(link: &Option<String>) -> anyhow::Result<Option<String>> {
    match link.as_ref() {
        None => Ok(None),
        Some(link) if link.starts_with("https://drive.google.com/file/d/") => {
            let map_err = || anyhow!("Omg cannot strip prefix which is there WTF");
            let prefix = &"https://drive.google.com/file/d/";
            let link = link.strip_prefix(prefix).ok_or_else(map_err)?;
            let link = link.split_once('/').map_or(link, |p| p.0);
            Ok(Some(format!("https://drive.google.com/uc?id={link}")))
        }
        Some(link) => Err(anyhow!("{link} is not a google drive link")),
    }
}

impl FS {
    pub fn entry(lang: Lang, next_states: IndexMap<String, FS>,) -> Self {
        Self {
            link: None,
            message: lang.details().greeting.to_string(),
            next_states,
        }
    }
    pub fn parse_row(row: &Row, options: IndexMap<String, FS>) -> anyhow::Result<Self> {
        Ok(Self {
            link: parse_link(&row.link)?,
            message: row.answer.clone(),
            next_states: options,
        })
    }
    pub fn get_state<'a>(&'a self, context: &[String]) -> anyhow::Result<&'a Self> {
        let mut ret = self;
        for key in context {
            let map_err = || anyhow!("Cannot find next state: {key} being on {}", self.message);
            ret = ret.next_states.get(key).ok_or_else(map_err)?;
        }
        Ok(ret)
    }
}
