// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row text: the title format and app name stripping.

use crate::core::text::sanitize_title;
use crate::core::window::WindowInfo;

/// Separators before an app name at the end of a title.
const SEPARATORS: [&str; 3] = [" — ", " – ", " - "];

/// One piece of a title format.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Part {
    Text(String),
    App,
    Title,
}

/// Turns a window into row text, from `title_format` and
/// `strip_app_name`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleFormat {
    parts: Vec<Part>,
    strip_app_name: bool,
}

impl Default for TitleFormat {
    fn default() -> Self {
        Self {
            parts: vec![Part::Title],
            strip_app_name: true,
        }
    }
}

impl TitleFormat {
    /// Parses `format`: text with `{app}` and `{title}` placeholders.
    ///
    /// # Errors
    ///
    /// Returns what is wrong: an unknown placeholder, a brace outside a
    /// placeholder, or no placeholder at all.
    pub fn parse(format: &str, strip_app_name: bool) -> Result<Self, String> {
        let mut parts = Vec::new();
        let mut rest = format;

        while let Some(start) = rest.find(['{', '}']) {
            let tail = &rest[start..];
            let end = tail
                .find('}')
                .filter(|_| tail.starts_with('{'))
                .ok_or("a brace outside a placeholder")?;

            push_text(&mut parts, &rest[..start]);
            parts.push(match &tail[..=end] {
                "{app}" => Part::App,
                "{title}" => Part::Title,
                other => return Err(format!("unknown placeholder `{other}`")),
            });
            rest = &tail[end + 1..];
        }
        push_text(&mut parts, rest);

        if parts.iter().all(|p| matches!(p, Part::Text(_))) {
            return Err("no placeholder".into());
        }
        Ok(Self {
            parts,
            strip_app_name,
        })
    }

    /// Returns the sanitized row text of `window`.
    #[must_use]
    pub fn apply(&self, window: &WindowInfo) -> String {
        let title = sanitize_title(&window.title);
        let app = sanitize_title(&window.app_name);
        let title = if self.strip_app_name {
            strip_app_name(&title, &app, &window.app_id)
        } else {
            &title
        };
        let text: String = self
            .parts
            .iter()
            .map(|part| match part {
                Part::Text(text) => text.as_str(),
                Part::App => app.as_str(),
                Part::Title => title,
            })
            .collect();

        sanitize_title(&text) // Trims around an empty app name
    }
}

fn push_text(parts: &mut Vec<Part>, text: &str) {
    if !text.is_empty() {
        parts.push(Part::Text(text.to_owned()));
    }
}

/// Drops ` — <app>` from the end of `title` when `<app>` is the app name
/// or the last part of the app id, ignoring case. Never returns an empty
/// title.
fn strip_app_name<'a>(title: &'a str, app: &str, app_id: &str) -> &'a str {
    let short_id = app_id.rsplit('.').next().unwrap_or_default();
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
