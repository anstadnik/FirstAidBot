use crate::{bot::Message, REDIS_CONN};
use anyhow::{anyhow, Context, Result};
use first_aid_bot_core::prelude::FAContext;
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};

const ERR: &str = "Error writing a user to the redis db.";

pub async fn log_to_redis(msg: &Message, ctx: &FAContext) -> Result<()> {
    let mut conn = REDIS_CONN
        .get()
        .ok_or_else(|| anyhow!("No connection"))?
        .clone();
    let err = || anyhow!("Cannot get id from user");
    let id = msg.from().ok_or_else(err)?.id.0.to_string();
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
    Ok(())
}
