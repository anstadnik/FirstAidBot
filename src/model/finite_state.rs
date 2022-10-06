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

pub type FSNextStates = IndexMap<String, FS>;

#[derive(Debug, Clone)]
pub struct FS {
    pub link: Option<String>,
    pub message: String,
    pub next_states: FSNextStates,
}

fn parse_link(link: &Option<String>) -> anyhow::Result<Option<String>> {
    match link.as_ref() {
        None => Ok(None),
        Some(link) if link.starts_with("https://drive.google.com/file/d/") => {
            let link = link
                .strip_prefix("https://drive.google.com/file/d/")
                .ok_or_else(|| anyhow!("Omg cannot strip prefix which is there WTF"))?;
            let link = link.split_once('/').map(|p| p.0).unwrap_or(link);
            Ok(Some(format!("https://drive.google.com/uc?id={link}")))
        }
        Some(link) => Err(anyhow!("{link} is not a google drive link",)),
    }
}

impl FS {
    pub fn entry(lang: &Lang, next_states: FSNextStates) -> Self {
        Self {
            link: None,
            message: lang.details().greeting.to_string(),
            next_states,
        }
    }
    pub fn parse_row(row: &Row, options: FSNextStates) -> anyhow::Result<Self> {
        Ok(Self {
            link: parse_link(&row.link)?,
            message: row.answer.to_owned(),
            next_states: options,
        })
    }
    pub fn get_state<'a>(&'a self, context: &[String]) -> anyhow::Result<&'a Self> {
        let mut ret = self;
        for key in context {
            ret = ret.next_states.get(key).ok_or_else(|| {
                anyhow!("Cannot find next state: {key} being on {}", self.message)
            })?;
        }
        Ok(ret)
    }
}
