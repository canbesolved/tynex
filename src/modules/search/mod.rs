// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Search: fuzzy matching over window titles and app ids.

pub mod fold;
pub mod fuzzy;

use fold::Folded;

use crate::core::title::short_app_id;

/// Text matched against the query for the app column: the display
/// name and the short app id. A reverse-DNS id is not used whole, so
/// a query like `code` does not hit `org.rustdesk.RustDesk`.
#[must_use]
pub fn app_haystack(app_name: &str, app_id: &str) -> String {
    let short = short_app_id(app_id);
    // A dotted name is usually the raw id before a desktop Name.
    let name = app_name.trim();
    let name = if name.contains('.') { "" } else { name };

    if name.is_empty() {
        short.to_owned()
    } else if name.eq_ignore_ascii_case(short) {
        name.to_owned()
    } else {
        format!("{name} {short}")
    }
}

/// Whether `text` starts with `query`, using the same case and accent
/// rules as ranking. An empty query matches nothing.
#[must_use]
pub fn starts_with_query(text: &str, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    let folded = if query.is_ascii() {
        Folded::new(text).text
    } else {
        text.to_owned()
    };

    if query.chars().any(char::is_uppercase) {
        folded.starts_with(query)
    } else if query.is_ascii() {
        folded.len() >= query.len()
            && folded.as_bytes()[..query.len()]
                .eq_ignore_ascii_case(query.as_bytes())
    } else {
        let mut hay = folded.chars().flat_map(char::to_lowercase);

        query
            .chars()
            .flat_map(char::to_lowercase)
            .all(|letter| hay.next() == Some(letter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haystack_uses_short_id_not_reverse_dns() {
        assert_eq!(
            app_haystack("RustDesk", "org.rustdesk.RustDesk"),
            "RustDesk"
        );
        assert_eq!(app_haystack("Konsole", "org.kde.konsole"), "Konsole");
        assert_eq!(app_haystack("", "org.kde.konsole"), "konsole");
        assert_eq!(
            app_haystack("org.telegram.desktop", "org.telegram.desktop"),
            "desktop"
        );
        assert_eq!(app_haystack("Firefox", "firefox"), "Firefox");
        assert_eq!(app_haystack("Code", "com.visualstudio.code"), "Code");
        assert_eq!(
            app_haystack("VS Code", "com.visualstudio.code"),
            "VS Code code"
        );
    }

    #[test]
    fn a_leading_match_is_the_start_of_the_text() {
        assert!(starts_with_query("Cursor Agents", "c"));
        assert!(starts_with_query("Café", "c"));
        assert!(!starts_with_query("Google Chrome", "c"));
        assert!(starts_with_query("Cursor", "C"));
        assert!(!starts_with_query("cursor", "C"));
        assert!(!starts_with_query("Cursor", ""));
    }
}
