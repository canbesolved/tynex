// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! One-line text editing with a byte cursor, for the query and for a
//! title being renamed.

use crate::core::event::Key;

/// Inserts `c` at `cursor` and moves the cursor past it.
pub fn insert(text: &mut String, cursor: &mut usize, c: char) {
    text.insert(*cursor, c);
    *cursor += c.len_utf8();
}

/// Deletes the character before `cursor`.
///
/// Returns false when the cursor is already at the start.
pub fn backspace(text: &mut String, cursor: &mut usize) -> bool {
    let Some(start) = prev_char(text, *cursor) else {
        return false;
    };

    text.drain(start..*cursor);
    *cursor = start;
    true
}

/// Deletes the character at `cursor`.
///
/// Returns false when the cursor is already at the end.
pub fn delete(text: &mut String, cursor: usize) -> bool {
    let Some(end) = next_char(text, cursor) else {
        return false;
    };

    text.drain(cursor..end);
    true
}

/// Returns the cursor one character left, or `cursor` at the start.
#[must_use]
pub fn left(text: &str, cursor: usize) -> usize {
    prev_char(text, cursor).unwrap_or(cursor)
}

/// Returns the cursor one character right, or `cursor` at the end.
#[must_use]
pub fn right(text: &str, cursor: usize) -> usize {
    next_char(text, cursor).unwrap_or(cursor)
}

/// Returns the start of the word before `cursor`.
#[must_use]
pub fn word_left(text: &str, cursor: usize) -> usize {
    let head = text[..cursor].trim_end();

    match head.rfind(char::is_whitespace) {
        Some(i) => i + head[i..].chars().next().map_or(0, char::len_utf8),
        None => 0,
    }
}

/// Returns the start of the word after `cursor`.
#[must_use]
pub fn word_right(text: &str, cursor: usize) -> usize {
    let rest = &text[cursor..];
    let word = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let spaces = rest[word..].len() - rest[word..].trim_start().len();

    cursor + word + spaces
}

/// Applies an editing `key`: a character, a delete or a cursor move.
///
/// Returns `None` when `key` does not edit; otherwise whether the text
/// changed.
pub fn apply(text: &mut String, cursor: &mut usize, key: Key) -> Option<bool> {
    match key {
        Key::Char { text: c, .. } => {
            insert(text, cursor, c);
            Some(true)
        }
        Key::Backspace => Some(backspace(text, cursor)),
        Key::Delete => Some(delete(text, *cursor)),
        Key::Left => Some(moved(cursor, left(text, *cursor))),
        Key::Right => Some(moved(cursor, right(text, *cursor))),
        Key::WordLeft => Some(moved(cursor, word_left(text, *cursor))),
        Key::WordRight => Some(moved(cursor, word_right(text, *cursor))),
        Key::Home => Some(moved(cursor, 0)),
        Key::End => Some(moved(cursor, text.len())),
        _ => None,
    }
}

/// Moves `cursor` to `to`. Returns false: the text did not change.
fn moved(cursor: &mut usize, to: usize) -> bool {
    *cursor = to;
    false
}

/// Returns the byte index of the character before `index`.
fn prev_char(text: &str, index: usize) -> Option<usize> {
    text[..index].char_indices().next_back().map(|(i, _)| i)
}

/// Returns the byte index after the character at `index`.
fn next_char(text: &str, index: usize) -> Option<usize> {
    text[index..].chars().next().map(|c| index + c.len_utf8())
}
