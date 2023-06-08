// use std::collections::VecDeque;

use anyhow::Result;

use super::*;

#[tokio::test]
async fn test_data_loading() -> Result<()> {
    let data = if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached()?
    };
    let mlfs = data.get().await?;
    for (_lang, fs) in mlfs.iter() {
        test_fs(fs)?;
    }
    Ok(())
}

fn test_fs(_fs: &Fs) -> Result<()> {
    // let mut q = VecDeque::new();
    // let Fs {
    //     link,
    //     message,
    //     next_states,
    // } = fs;
    // let mut it = message.chars();
    // while let Some(c) = it.next() {
    //     if c == '\\' {
    //         it.next();
    //         continue;
    //     }
    // }
    Ok(())
}
