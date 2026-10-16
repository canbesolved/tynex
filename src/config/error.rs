// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Config errors with file positions.

use std::fmt;
use std::ops::Range;

/// One problem in the config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub position: Option<(usize, usize)>, // Line and column, from 1
    pub message: String,
}

impl ConfigError {
    /// Creates an error at byte `span` of `text`; an empty span has no
    /// position.
    pub(super) fn at(text: &str, span: Range<usize>, message: String) -> Self {
        let position = (!span.is_empty()).then(|| line_col(text, span.start));

        Self { position, message }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.position {
            Some((line, col)) => write!(f, "{line}:{col}: {}", self.message),
            None => f.write_str(&self.message),
        }
    }
}

/// Returns the line and column (from 1, in chars) of byte `offset`.
fn line_col(text: &str, offset: usize) -> (usize, usize) {
    let before = &text[..offset.min(text.len())];
    let line = before.matches('\n').count() + 1;
    let col = before.rsplit('\n').next().map_or(0, |l| l.chars().count());

    (line, col + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_count_lines_and_chars() {
        let text = "a = 1\né = 2\n";

        assert_eq!(line_col(text, 0), (1, 1));
        assert_eq!(line_col(text, 6), (2, 1));
        assert_eq!(line_col(text, 10), (2, 4)); // After the 2-byte é
    }

    #[test]
    fn empty_span_has_no_position() {
        let e = ConfigError::at("x", 0..0, "msg".into());

        assert_eq!(e.to_string(), "msg");
    }
}
