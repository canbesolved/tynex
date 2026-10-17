// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Freedesktop key files: desktop entries, icon theme indexes and KDE
//! config files share one `[Group]` and `Key=Value` format.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Largest key file read; real ones are well under this.
const MAX_SIZE: u64 = 1 << 20;

/// Reads `path`, at most 1 MiB, with invalid UTF-8 replaced.
#[must_use]
pub fn read(path: &Path) -> Option<String> {
    let mut bytes = Vec::new();

    File::open(path)
        .ok()?
        .take(MAX_SIZE)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// Yields `(group, key, value)` for each entry of `text`, in order.
/// Comments, blank lines and lines without `=` are skipped; spaces
/// around `=` are trimmed.
pub fn entries(text: &str) -> impl Iterator<Item = (&str, &str, &str)> {
    let mut group = "";

    text.lines().map(str::trim).filter_map(move |line| {
        if let Some(name) =
            line.strip_prefix('[').and_then(|l| l.strip_suffix(']'))
        {
            group = name;
            return None;
        }
        if line.starts_with('#') {
            return None;
        }

        let (key, value) = line.split_once('=')?;

        Some((group, key.trim_end(), value.trim_start()))
    })
}

/// Returns the value of `key` in `group`, if any.
#[must_use]
pub fn value<'a>(text: &'a str, group: &str, key: &str) -> Option<&'a str> {
    entries(text)
        .find(|&(g, k, _)| g == group && k == key)
        .map(|(_, _, v)| v)
}

/// Decodes `\s`, `\n`, `\t`, `\r` and `\\`.
#[must_use]
pub fn unescape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('s') => out.push(' '),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\
# Comment
top=1
[Icons]
Theme = breeze-dark
no equals sign
[Other]
Theme=x
";

    #[test]
    fn entries_carry_their_group() {
        let all: Vec<_> = entries(TEXT).collect();

        assert_eq!(
            all,
            [
                ("", "top", "1"),
                ("Icons", "Theme", "breeze-dark"),
                ("Other", "Theme", "x"),
            ]
        );
    }

    #[test]
    fn value_looks_in_one_group() {
        assert_eq!(value(TEXT, "Icons", "Theme"), Some("breeze-dark"));
        assert_eq!(value(TEXT, "Icons", "Size"), None);
    }

    #[test]
    fn escapes_are_decoded() {
        assert_eq!(unescape(r"a\sb\tc\\d\"), "a b\tc\\d\\");
    }
}
