// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `[browser_tabs]`: which browsers may register a host.

use std::ops::Range;

use serde::Deserialize;
use toml::Spanned;

use crate::config::validate::check_range;

/// Default `browser_tabs.wait`, in milliseconds.
const WAIT: u16 = 150;

/// One supported Chromium browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Browser {
    Brave,
    Chrome,
    Chromium,
}

/// `"auto"` or an explicit list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Browsers {
    #[default]
    Auto,
    List(Vec<Browser>),
}

/// The `[browser_tabs]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserTabsConfig {
    pub enabled: bool,
    pub browsers: Browsers,
    pub wait: u16, // Milliseconds the popup waits for tab lists; 0 never
}

impl Default for BrowserTabsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            browsers: Browsers::Auto,
            wait: WAIT,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawBrowserTabs {
    pub enabled: bool,
    pub browsers: Option<Spanned<toml::Value>>,
    pub wait: Option<Spanned<u16>>,
}

const NAMES: &[(&str, Browser)] = &[
    ("brave", Browser::Brave),
    ("chrome", Browser::Chrome),
    ("chromium", Browser::Chromium),
];

/// Checks `browsers` and `wait`. An unknown name is an error.
pub(super) fn check(
    raw: RawBrowserTabs,
    err: &mut impl FnMut(Range<usize>, String),
) -> BrowserTabsConfig {
    let browsers = match raw.browsers {
        None => Browsers::Auto,
        Some(value) => browsers_of(&value, err),
    };

    if let Some(wait) = &raw.wait {
        check_range("browser_tabs.wait", wait, 0..=1000, err);
    }
    BrowserTabsConfig {
        enabled: raw.enabled,
        browsers,
        wait: raw.wait.map_or(WAIT, Spanned::into_inner),
    }
}

/// `"auto"`, one browser name, or a list of them.
fn browsers_of(
    value: &Spanned<toml::Value>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Browsers {
    match value.get_ref() {
        toml::Value::String(text) if text == "auto" => Browsers::Auto,
        toml::Value::String(text) => Browsers::List(
            name_of(text, value.span(), err).into_iter().collect(),
        ),
        toml::Value::Array(items) => {
            Browsers::List(names_of(items, value.span(), err))
        }
        _ => {
            err(
                value.span(),
                "`browser_tabs.browsers` must be \"auto\", a browser name \
                 or a list"
                    .to_owned(),
            );
            Browsers::Auto
        }
    }
}

/// Maps names. A bad name is reported and skipped.
fn names_of(
    names: &[toml::Value],
    span: Range<usize>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Vec<Browser> {
    let mut out = Vec::new();

    for name in names {
        let Some(text) = name.as_str() else {
            err(span.clone(), "browser name must be text".to_owned());
            continue;
        };
        out.extend(name_of(text, span.clone(), err));
    }
    out
}

/// Maps one name. An unknown name is reported.
fn name_of(
    text: &str,
    span: Range<usize>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Option<Browser> {
    let found = NAMES.iter().find(|(known, _)| *known == text);

    if found.is_none() {
        err(
            span,
            format!(
                "`browser_tabs.browsers` names {text:?}; expected \"auto\", \
                 brave, chrome or chromium"
            ),
        );
    }
    found.map(|(_, browser)| *browser)
}
