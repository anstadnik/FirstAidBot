use anyhow::Result;

use super::*;

// TODO: add messages test when https://github.com/teloxide/teloxide/issues/702 is resolved
#[tokio::test]
async fn test_data_loading() -> Result<()> {
    if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached()?
    }
    .get()
    .await?;
    Ok(())
}
