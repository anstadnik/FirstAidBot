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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Message {
    link: Option<String>,
    message: String,
}

impl Message {
    pub fn new(link: Option<String>, message: String) -> Self {
        Self {
            link: parse_link(&link),
            message,
        }
    }
}

pub type FiniteStateOptions = Option<HashMap<Message, FiniteState>>;

#[derive(Debug)]
pub struct FiniteState {
    message: Message,
    options: FiniteStateOptions,
}

impl FiniteState {
    pub fn new(message: Message, options: FiniteStateOptions) -> Self {
        Self { message, options }
    }
}
