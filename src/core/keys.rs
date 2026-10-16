// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Letter keys for list rows.

/// Default key order: home row first, then `g` and the top row.
pub const DEFAULT_KEYS: &str = "asdfhjklgqwertyuiop";

/// Assigns a letter from `keys` to each of `rows` rows, in order.
///
/// Rows beyond the key string get `None` and are reachable with arrows
/// or search.
#[must_use]
pub fn assign(rows: usize, keys: &str) -> Vec<Option<char>> {
    let mut letters = keys.chars();

    (0..rows).map(|_| letters.next()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_get_keys_in_order() {
        assert_eq!(assign(3, DEFAULT_KEYS), [Some('a'), Some('s'), Some('d')]);
    }

    #[test]
    fn rows_beyond_keys_get_none() {
        assert_eq!(assign(3, "ab"), [Some('a'), Some('b'), None]);
    }

    #[test]
    fn no_rows_no_keys() {
        assert_eq!(assign(0, DEFAULT_KEYS), Vec::<Option<char>>::new());
        assert_eq!(assign(2, ""), [None, None]);
    }
}
