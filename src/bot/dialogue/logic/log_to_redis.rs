use crate::bot::prelude::*;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn log_to_redis(
    msg: &Message,
    lang: &Lang,
    context: &[String],
    conn: &mut MultiplexedConnection,
) {
    let mut conn = conn.clone();
    if let Some(user) = msg.from() {
        let user_id = user.id.0.to_string();

        // TODO: Remove it when we'll have dashboards <21-06-22, astadnik> //
        if context.is_empty() && conn.sadd::<_, _, ()>("all_users", &user_id).await.is_err() {
            log::error!("Error writing a user to the redis db.");
        }

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let key = "user_".to_string() + &user_id;
        let context = context.join("->");
        let value = format!("{context}; {lang}");
        if conn.hset::<_, _, _, ()>(key, time, value).await.is_err() {
            log::error!("Error writing a user to the redis db.");
        }
    };
}
