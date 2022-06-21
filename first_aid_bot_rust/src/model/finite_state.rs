use crate::Lang;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           CSV entry                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct Record {
    pub hierarchy: String,
    pub option: String,
    pub answer: String,
    pub link: Option<String>,
}

impl Record {
    pub fn is_empty(self: &Record) -> bool {
        self.hierarchy.is_empty()
            && self.option.is_empty()
            && self.answer.is_empty()
            && self.link.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       Finite State types                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub type MultilangStates = HashMap<Lang, FiniteState>;

#[derive(Debug, Clone)]
pub struct FiniteStateOptions {
    pub ordered_keys: Vec<String>,
    pub next_states: HashMap<String, FiniteState>,
}

#[derive(Debug, Clone)]
pub struct FiniteState {
    pub link: Option<String>,
    pub message: String,
    options: Option<FiniteStateOptions>,
}

fn parse_link(link: &Option<String>) -> Option<String> {
    match link.as_ref() {
        None => None,
        Some(link) if link.contains("file/d") => {
            let link = Regex::new(r".*file/d/").unwrap().replace(link, "");
            let link = Regex::new(r"/.*").unwrap().replace(&link, "").to_string();
            Some(format!("https://drive.google.com/uc?id={link}"))
        }
        Some(link) => Some(link.to_string()),
    }
}

impl FiniteState {
    pub fn new(
        link: Option<String>,
        message: String,
        options: Option<FiniteStateOptions>,
    ) -> FiniteState {
        FiniteState {
            link,
            message,
            options,
        }
    }
    pub fn parse_row(row: &&Record, options: Option<FiniteStateOptions>) -> FiniteState {
        FiniteState {
            link: parse_link(&row.link),
            message: row.answer.to_owned(),
            options,
        }
    }
    pub fn get_options(&self) -> &[String] {
        self.options
            .as_ref()
            .map(|opts| &opts.ordered_keys[..])
            .unwrap_or_default()
    }
    pub fn get_next_state(&self, option: &String) -> Option<&FiniteState> {
        self.options.as_ref()?.next_states.get(option)
    }
}
