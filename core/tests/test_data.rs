use anyhow::{bail, Context, Result};
use first_aid_bot_core::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

fn test_md(s: &str) -> Result<()> {
    let re = Regex::new(r"(^|[^\\])(\*|_|__|~|\|\|)").unwrap();
    let mut q: VecDeque<String> = VecDeque::new();
    for cap in re.captures_iter(s) {
        if q.back().map(String::as_str) == Some(&cap[2]) {
            q.pop_back();
        } else {
            q.push_back(cap[2].to_owned());
        }
    }
    if !q.is_empty() {
        bail!("Unmatched brackets: {:?}", q);
    }
    Ok(())
}

fn test_fs(fs: Fs) -> Result<()> {
    test_md(&fs.message)?;
    for (s, fs) in fs
        .next_states
    {
        test_fs(fs).context(s)?;
    }
    Ok(())
}

#[tokio::test]
async fn test_table() -> Result<()> {
    let data = if cfg!(debug_assertions) {
        get_data_from_web().await?
    } else {
        get_data_from_file("table.csv")?
    };
    for fs in data.into_values() {
        test_fs(fs)?;
    }

    Ok(())
}
