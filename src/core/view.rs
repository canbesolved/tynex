// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! What the presenter must draw. Popup modules read it, never the
//! switcher.

use std::ops::Range;

use crate::core::browser_tabs::{SourceId, TabRef};
use crate::core::window::{WindowId, WindowInfo};

/// What selecting a row does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowTarget {
    Window(WindowId),
    Launch(String), // App id
    Tab(TabRef),
}

/// Icon drawn for a row: a theme name or path, or a tab favicon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowIcon {
    Named(String),
    /// Favicon of one source. `fallback` is used when it is missing.
    Tab {
        source: SourceId,
        key: String,
        fallback: String,
    },
}

/// Opaque `#RRGGBB` text color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Builds a color from channels.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parses `#RRGGBB`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix('#')?;

        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        Some(Self {
            r: u8::from_str_radix(&hex[0..2], 16).ok()?,
            g: u8::from_str_radix(&hex[2..4], 16).ok()?,
            b: u8::from_str_radix(&hex[4..6], 16).ok()?,
        })
    }
}

/// One list entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub target: RowTarget,
    pub app_id: String,   // Matched by search, not drawn
    pub app_name: String, // Matched by search, not drawn
    pub icon: Option<RowIcon>,
    pub title: String, // Sanitized, app name stripped
    pub title_color: Option<Rgb>, // None uses the theme text color
    pub highlights: Vec<Range<usize>>, // Byte ranges in `title`
}

impl Row {
    /// Builds the row of an open window.
    #[must_use]
    pub fn window(window: &WindowInfo) -> Self {
        Self {
            target: RowTarget::Window(window.id.clone()),
            app_id: window.app_id.clone(),
            app_name: window.app_name.clone(),
            icon: window.icon.clone().map(RowIcon::Named),
            title: window.title.clone(),
            title_color: None,
            highlights: Vec::new(),
        }
    }
}

/// One running app in the icon bar.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppIcon {
    pub app_id: String,
    pub icon: Option<String>,
    /// Windows of this app, most recently used first.
    pub windows: Vec<WindowId>,
    /// Set for the previous place when it is a browser tab.
    pub tab: Option<TabIcon>,
}

/// A browser tab in the icon strip, drawn with its favicon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabIcon {
    pub tab: TabRef,
    pub key: String,      // Favicon key of the source
    pub fallback: String, // Icon name when the favicon is missing
}

/// A title being renamed, borrowed for one frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Edit<'a> {
    pub id: &'a WindowId,
    pub text: &'a str,
    pub cursor: usize, // Byte index in `text`
}

/// The icon strip, borrowed for one frame.
pub(crate) struct Strip<'a> {
    pub icon: usize,
    pub dropdown: Option<usize>,
    pub icons: &'a [AppIcon],
}

/// Read-only snapshot of the popup, borrowed from the switcher state.
#[derive(Debug, Clone, Copy)]
pub struct View<'a> {
    pub query: &'a str,
    pub hint: &'a str,           // Shown while `query` is empty
    pub cursor: usize,           // Byte index in `query`
    pub selected: usize,         // Index into the output rows
    pub icon: usize,             // Index into `icons`
    pub dropdown: Option<usize>, // Selected title while open
    edit: Option<Edit<'a>>,      // Title being renamed
    windows: &'a [Row],
    titles: &'a [Row], // Title list: windows and tabs, history first
    output: &'a [Row],
    icons: &'a [AppIcon],
}

impl<'a> View<'a> {
    pub(crate) fn new(
        query: &'a str,
        hint: &'a str,
        cursor: usize,
        selected: usize,
        (windows, titles): (&'a [Row], &'a [Row]),
        output: &'a [Row],
        strip: &Strip<'a>,
    ) -> Self {
        Self {
            query,
            hint,
            cursor,
            selected,
            icon: strip.icon,
            dropdown: strip.dropdown,
            edit: None,
            windows,
            titles,
            output,
            icons: strip.icons,
        }
    }

    /// Shows `edit` in place of its window's title.
    #[must_use]
    pub(crate) fn with_edit(mut self, edit: Option<Edit<'a>>) -> Self {
        self.edit = edit;
        self
    }

    /// Returns the edited text and its cursor when `target`'s title is
    /// being renamed. The row title is then drawn as this text.
    #[must_use]
    pub fn edit(&self, target: &RowTarget) -> Option<(&'a str, usize)> {
        let edit = self.edit?;

        matches!(target, RowTarget::Window(w) if w == edit.id)
            .then_some((edit.text, edit.cursor))
    }

    /// Returns the title list rows: windows and tabs, the history
    /// first. The title bar draws these. Search does not filter them.
    pub fn titles(&self) -> impl Iterator<Item = &'a Row> + 'a {
        self.titles.iter()
    }

    /// Returns the number of title list rows.
    #[must_use]
    pub fn title_count(&self) -> usize {
        self.titles.len()
    }

    /// Returns the output rows, in display order. The output bar draws
    /// these: matches of the current query.
    pub fn rows(&self) -> impl Iterator<Item = &'a Row> + 'a {
        self.output.iter()
    }

    /// Returns the number of output rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.output.len()
    }

    /// Returns true when there is nothing to draw.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }

    /// Returns the open windows as a slice.
    #[must_use]
    pub fn window_rows(&self) -> &'a [Row] {
        self.windows
    }

    /// Returns the running apps, most recently used first.
    #[must_use]
    pub fn icons(&self) -> &'a [AppIcon] {
        self.icons
    }
}
