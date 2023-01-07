use super::prelude::*;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start { lang: String },
    Dialogue { lang: String, context: Vec<String> },
    Broadcast { message: Option<String> },
}

impl Default for State {
    fn default() -> Self {
        let lang = Lang::default().name();
        Self::Start { lang }
    }
}
