use anyhow::{bail, Context, Result};
use first_aid_bot_core::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

// https://github.com/rust-lang/rust/issues/93743
fn test_md(s: &str) -> Result<()> {
    let re = Regex::new(r"(```|\|\||__|`|\*|_|~)").unwrap();
    let mut q: VecDeque<&str> = VecDeque::new();
    for mat in re.find_iter(s) {
        if mat.start() == 0
            || !s.is_char_boundary(mat.start() - 1)
            || &s[(mat.start() - 1)..mat.start()] != "\\"
        {
            if q.back() == Some(&mat.as_str()) {
                q.pop_back();
            } else {
                q.push_back(mat.as_str());
            }
        }
    }
    if !q.is_empty() {
        bail!("Unmatched brackets: {:?}", q);
    }
    Ok(())
}

fn test_fs(fs: Fs) -> Result<()> {
    if fs.message.chars().all(char::is_whitespace) {
        bail!("Empty message");
    }
    if fs
        .link
        .as_ref()
        .is_some_and(|s| s.chars().all(char::is_whitespace))
    {
        bail!("Empty link");
    }
    test_md(&fs.message)?;
    for (s, fs) in fs.next_states {
        test_fs(fs).context(s)?;
    }
    Ok(())
}

// Add tests using cfg

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_example() -> Result<()> {
        let s = r" *bold \*text*
_italic \*text_
__underline__
~strikethrough~
||spoiler||
*bold _italic bold ~italic bold strikethrough ||italic bold strikethrough spoiler||~ __underline italic bold___ bold*
`inline fixed-width code`
```
pre-formatted fixed-width code block
```
```python
pre-formatted fixed-width code block written in the Python programming language
``` ";
        test_md(s)
    }

    #[test_log::test(tokio::test)]
    async fn test_table() -> Result<()> {
        let data = if cfg!(debug_assertions) {
            get_data_from_web().await?
        } else {
            get_data_from_file("../table.csv")?
        };
        assert!(!data.is_empty());
        assert!(data.iter().all(|(_, fs)| fs.num_nodes() > 1));
        assert!(Lang::iter().all(|lang| data.contains_key(&lang)));
        for (lang, fs) in &data {
            log::info!("First keys for lang {lang} are: ");
            for key in fs.next_states.keys() {
                log::info!("{}", key);
            }
        }
        data.into_iter()
            .inspect(|(lang, fs)| log::info!("Testing {lang} with {} nodes", fs.num_nodes()))
            .try_for_each(|(_, fs)| test_fs(fs))?;

        Ok(())
    }
}
