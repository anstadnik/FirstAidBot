use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

use crate::lang::Lang;

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
    pub options: Option<FiniteStateOptions>,
}

fn parse_link(link: &Option<String>) -> Option<String> {
    match link.to_owned() {
        None => None,
        Some(link) if link.contains("file/d") => {
            let link = Regex::new(r".*file/d/").unwrap().replace(&link, "");
            let link = Regex::new(r"/.*").unwrap().replace(&link, "").to_string();
            Some(format!("https://drive.google.com/uc?id={link}"))
        }
        Some(link) => Some(link),
    }
}

impl FiniteState {
    pub fn new(row: &&Record, options: Option<FiniteStateOptions>) -> FiniteState {
        FiniteState {
            link: parse_link(&row.link),
            message: row.answer.to_owned(),
            options,
        }
    }
}
