use super::lang::Lang;
use anyhow::anyhow;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;

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

pub type MultilangFs = HashMap<Lang, Fs>;

#[derive(Debug, Clone)]
pub struct Fs {
    pub link: Option<String>,
    pub message: String,
    pub next_states: IndexMap<String, Fs>,
}

const PREFIX: &str = "https://drive.google.com/file/d/";
fn parse_link(link: &Option<String>) -> anyhow::Result<Option<String>> {
    match link.as_ref() {
        None => Ok(None),
        Some(link) if link.starts_with(PREFIX) => {
            let map_err = || anyhow!("Omg cannot strip prefix which is there WTF");
            let link = link.strip_prefix(PREFIX).ok_or_else(map_err)?;
            let link = link.split_once('/').map_or(link, |p| p.0);
            Ok(Some(format!("https://drive.google.com/uc?id={link}")))
        }
        Some(link) => Err(anyhow!("{link} is not a google drive link")),
    }
}

impl Fs {
    pub fn entry(lang: Lang, next_states: IndexMap<String, Fs>) -> Self {
        Self {
            link: None,
            message: lang.details().greeting.to_string(),
            next_states,
        }
    }
    pub fn parse_row(row: &Row, options: IndexMap<String, Fs>) -> anyhow::Result<Self> {
        Ok(Self {
            link: parse_link(&row.link)?,
            message: row.answer.clone(),
            next_states: options,
        })
    }
}
