// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row titles: sanitized, with the app name stripped from the end.

use crate::core::text::sanitize_title;
use crate::core::window::WindowInfo;

/// Separators before an app name at the end of a title.
const SEPARATORS: [&str; 3] = [" — ", " – ", " - "];

/// Returns the sanitized title of `window`; with `strip`, a trailing
/// ` — <app name>` is dropped (see `strip_app_name`).
#[must_use]
pub fn row_title(window: &WindowInfo, strip: bool) -> String {
    let title = sanitize_title(&window.title);

    if !strip {
        return title;
    }

    let app = sanitize_title(&window.app_name);

    strip_app_name(&title, &app, &window.app_id).to_owned()
}

/// Last segment of a reverse-DNS app id (`org.kde.konsole` →
/// `konsole`).
#[must_use]
pub fn short_app_id(app_id: &str) -> &str {
    app_id.rsplit('.').next().unwrap_or(app_id)
}

/// Drops ` — <app>` from the end of `title` when `<app>` is the app name
/// or the last part of the app id, ignoring case. Never returns an empty
/// title.
fn strip_app_name<'a>(title: &'a str, app: &str, app_id: &str) -> &'a str {
    let short_id = short_app_id(app_id);
    let is_app = |text: &str| {
        [app, short_id]
            .iter()
            .any(|name| !name.is_empty() && same_ignoring_case(text, name))
    };

    SEPARATORS
        .iter()
        .filter_map(|sep| title.rsplit_once(sep))
        .find(|&(rest, suffix)| !rest.is_empty() && is_app(suffix))
        .map_or(title, |(rest, _)| rest)
}

/// Compares with Unicode lowercase mapping, without allocating.
fn same_ignoring_case(a: &str, b: &str) -> bool {
    a.chars()
        .flat_map(char::to_lowercase)
        .eq(b.chars().flat_map(char::to_lowercase))
}

#[cfg(test)]
#[path = "title_tests.rs"]
mod tests;
