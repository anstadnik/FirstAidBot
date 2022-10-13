use super::prelude::*;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start { lang: String },
    Dialogue { lang: String, context: Vec<String> },
    Broadcast { message: Option<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start {
            lang: Lang::default().name(),
        }
    }
}
