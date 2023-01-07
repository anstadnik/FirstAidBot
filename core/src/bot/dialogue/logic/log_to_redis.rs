use crate::{bot::Message, prelude::State};
use anyhow::{Context as AnyhowContext, Result};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::time::{SystemTime, UNIX_EPOCH};

const ERR: &str = "Error writing a user to the redis db.";

pub async fn log_to_redis(
    msg: &Message,
    ctx: &State,
    conn: &mut MultiplexedConnection,
) -> Result<()> {
    if let Some(id) = msg.from().map(|user| user.id.0.to_string()) {
        // TODO: Remove it when we'll have dashboards <21-06-22, astadnik> //
        if ctx.is_empty() {
            conn.sadd::<_, _, ()>("all_users", &id).await.context(ERR)?;
        }

        let ts: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .try_into()?;
        let key = "user_".to_string() + &id;
        let val = ctx.to_string();
        conn.hset::<_, _, _, ()>(key, ts, val).await.context(ERR)?;
    };
    Ok(())
}
