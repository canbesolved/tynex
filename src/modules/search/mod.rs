// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Search: fuzzy matching over window titles and app ids.

pub mod fold;
pub mod fuzzy;

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
}
