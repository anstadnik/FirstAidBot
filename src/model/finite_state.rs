use crate::Lang;
use anyhow::{anyhow, Context};
use indexmap::IndexMap;
use regex::Regex;
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
    pub fn is_empty(self: &Row) -> bool {
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
        Some(link) if link.contains("file/d") => {
            let link = Regex::new(r".*file/d/").unwrap().replace(link, "");
            let link = Regex::new(r"/.*").unwrap().replace(&link, "").to_string();
            Ok(Some(format!("https://drive.google.com/uc?id={link}")))
        }
        Some(link) => Err(anyhow!("{link} is not a google drive link",)),
    }
}

impl FS {
    pub fn entry(lang: &Lang, next_states: FSNextStates) -> FS {
        FS {
            link: None,
            message: lang.details().greeting.to_string(),
            next_states,
        }
    }
    pub fn parse_row(row: &Row, options: FSNextStates) -> anyhow::Result<FS> {
        Ok(FS {
            link: parse_link(&row.link)?,
            message: row.answer.to_owned(),
            next_states: options,
        })
    }
    pub fn get_state<'a>(&'a self, context: &[String]) -> anyhow::Result<&'a FS> {
        if context.is_empty() {
            return Ok(self);
        }
        let next_state = self.next_states.get(&context[0]).context(format!(
            "Cannot find next state: {} being on {context:?}",
            self.message
        ))?;
        next_state.get_state(&context[1..])
    }
}
