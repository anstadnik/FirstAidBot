use regex::Regex;
use std::collections::HashMap;

pub fn parse_link(link: &Option<String>) -> Option<String> {
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
