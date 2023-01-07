mod bot;
mod logic;
mod model;

pub use teloxide::types::UserId;
pub mod prelude {
    pub use super::bot::prelude::*;
    pub use super::logic::prelude::*;
    pub use super::model::prelude::*;
}

const MAINTAINER_IDS: [teloxide::types::UserId; 1] = [UserId(131_596_643)];
const MAINTAINER_USERNAMES: [&str; 2] = ["astadnik", "Oleksa_Lev"];
const HELP_CHAT_URL: &str = "https://t.me/+VOd-pnzhWvU2Yjg6";
const REDIS_URLS: [&str; 2] = ["redis://redis:6379", "redis://127.0.0.1:6379"];
const REDIS_USERS_SET_KEY: &str = "all_users";
const BROADCAST_ENABLED: bool = false;
