use super::*;

// TODO: add messages test when https://github.com/teloxide/teloxide/issues/702 is resolved
#[tokio::test]
async fn test_data_loading() {
    assert!(get_data().await.is_ok())
}
