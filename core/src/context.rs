use core::fmt::Display;
use crate::prelude::Lang;

#[derive(Clone, Default)]
pub struct FAContext {
    pub lang: Lang,
    pub context: Vec<String>,
}

impl FAContext {
    pub fn transition(&mut self, text: String) {
        self.context.push(text);
    }
    pub fn back(&mut self) {
        self.context.pop();
    }
    pub fn home(&mut self) {
        self.context = Vec::new();
    }
    pub fn depth(&self) -> usize {
        self.context.len()
    }
    pub fn is_empty(&self) -> bool {
        self.context.is_empty()
    }
}

impl Display for FAContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; {}", self.context.join("->"), self.lang)
    }
}
