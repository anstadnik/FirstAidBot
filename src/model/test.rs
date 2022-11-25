use super::*;

// TODO: add messages test when https://github.com/teloxide/teloxide/issues/702 is resolved
#[tokio::test]
async fn test_data_loading() {
    let filename = (!cfg!(debug_assertions)).then_some("table.csv");
    assert!(
        get_data(filename).await.is_ok(),
        "Cannot load data with filename = {filename:?}"
    );
}
