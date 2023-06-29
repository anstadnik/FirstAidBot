use anyhow::{bail, Context, Result};
use first_aid_bot_core::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

// https://github.com/rust-lang/rust/issues/93743
fn test_md(s: &str) -> Result<()> {
    let re = Regex::new(r"(```|\|\||__|`|\*|_|~)").unwrap();
    let mut q: VecDeque<&str> = VecDeque::new();
    for mat in re.find_iter(s) {
        let prev_i = mat.start() - 1;
        if mat.start() == 0 || !s.is_char_boundary(prev_i) || &s[prev_i..mat.start()] != "\\" {
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
    test_md(&fs.message)?;
    for (s, fs) in fs.next_states {
        test_fs(fs).context(s)?;
    }
    Ok(())
}

#[tokio::test]
async fn test_table() -> Result<()> {
    let data = if cfg!(debug_assertions) {
        get_data_from_web().await?
    } else {
        get_data_from_file("../table.csv")?
    };
    for fs in data.into_values() {
        test_fs(fs)?;
    }

    Ok(())
}

// Add tests using cfg

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
