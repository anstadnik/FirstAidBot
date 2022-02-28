use regex::Regex;
use std::collections::HashMap;

fn parse_link(link: &Option<String>) -> Option<String> {
    match link.to_owned() {
        None => (None),
        Some(link) if link.contains("file/d") => {
            let link = Regex::new(r".*file/d/").unwrap().replace(&link, "");
            let link = Regex::new(r"/.*").unwrap().replace(&link, "").to_string();
            Some(link)
        }
        Some(link) => Some(link),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FiniteStateMsg {
    pub link: Option<String>,
    pub message: String,
}

impl FiniteStateMsg {
    pub fn new(link: Option<String>, message: String) -> Self {
        let link = parse_link(&link);
        Self { link, message }
    }
}

pub type FiniteStateOptions = Option<HashMap<String, FiniteState>>;

#[derive(Debug, Clone)]
pub struct FiniteState {
    pub message: FiniteStateMsg,
    pub options: FiniteStateOptions,
}

impl FiniteState {
    pub fn new(message: FiniteStateMsg, options: FiniteStateOptions) -> Self {
        Self { message, options }
    }
}
