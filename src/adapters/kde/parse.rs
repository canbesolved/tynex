// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Decodes the window list sent by the `KWin` script.

use std::collections::HashSet;

use crate::core::window::{WindowId, WindowInfo};

/// Separates windows.
const RECORD: char = '\x1e';

/// Separates the fields of one window.
const FIELD: char = '\x1f';

/// Turns `<uuid> US <file> US <class> US <caption> US <pid> RS ...`
/// into windows, in the order sent. Malformed records and repeats of
/// an id are skipped.
pub fn parse(payload: &str) -> Vec<WindowInfo> {
    let mut seen = HashSet::new();

    payload
        .split(RECORD)
        .filter_map(record)
        .filter(|w| seen.insert(w.id.clone()))
        .collect()
}

/// Decodes one record: exactly five fields and a non-empty id.
fn record(text: &str) -> Option<WindowInfo> {
    let mut fields = text.split(FIELD);
    let (id, desktop_file, class, caption, pid) = (
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
    );
    let id = id.trim_start_matches('{').trim_end_matches('}');

    if id.is_empty() || fields.next().is_some() {
        return None;
    }

    let app_id = if desktop_file.is_empty() {
        class
    } else {
        desktop_file
    };

    Some(WindowInfo {
        id: WindowId(id.to_owned()),
        app_id: app_id.to_owned(),
        app_name: class.to_owned(), // Until a desktop file names the app
        title: caption.to_owned(),  // Sanitized by the core
        icon: None,
        pid: pid_of(pid),
    })
}

/// A pid of `0` or not a number is missing.
fn pid_of(text: &str) -> Option<u32> {
    match text.parse() {
        Ok(0) | Err(_) => None,
        Ok(pid) => Some(pid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UUID_A: &str = "0b4c7e2a-1f3d-4c5b-9a6e-7d8f9a0b1c2d"; // Made up
    const UUID_B: &str = "5e6f7a8b-9c0d-4e1f-8a2b-3c4d5e6f7a8b";

    fn window(id: &str, app: (&str, &str), title: &str) -> WindowInfo {
        WindowInfo {
            id: WindowId(id.into()),
            app_id: app.0.into(),
            app_name: app.1.into(),
            title: title.into(),
            icon: None,
            pid: None,
        }
    }

    #[test]
    fn empty_payload_is_an_empty_list() {
        assert_eq!(parse(""), []);
    }

    #[test]
    fn records_keep_their_order() {
        let payload = format!(
            "{{{UUID_A}}}\x1forg.kde.konsole\x1fkonsole\x1f~ : bash\x1f100\x1e\
             {{{UUID_B}}}\x1ffirefox\x1ffirefox\x1fCafé — Firefox\x1f0"
        );

        let mut first =
            window(UUID_A, ("org.kde.konsole", "konsole"), "~ : bash");

        first.pid = Some(100);
        assert_eq!(
            parse(&payload),
            [
                first,
                window(UUID_B, ("firefox", "firefox"), "Café — Firefox"),
            ]
        );
    }

    #[test]
    fn empty_fields() {
        let no_desktop_file = format!("{UUID_A}\x1f\x1fxterm\x1f\x1f");
        let no_app = format!("{UUID_A}\x1f\x1f\x1fTitle\x1fnope");

        assert_eq!(
            parse(&no_desktop_file),
            [window(UUID_A, ("xterm", "xterm"), "")]
        );
        assert_eq!(parse(&no_app), [window(UUID_A, ("", ""), "Title")]);
    }

    #[test]
    fn malformed_records_are_skipped() {
        let good = format!("{UUID_B}\x1fa\x1fa\x1fA\x1f2");
        let payload = format!(
            "\x1fa\x1fa\x1fNo id\x1f1\x1e\
             {UUID_A}\x1fa\x1fa\x1fToo\x1fmany\x1fextra\x1e\
             {UUID_A}\x1fa\x1fToo few\x1e\
             \x1e{good}\x1e"
        );

        let mut kept = window(UUID_B, ("a", "a"), "A");

        kept.pid = Some(2);
        assert_eq!(parse(&payload), [kept]);
    }

    #[test]
    fn repeated_ids_keep_the_first_record() {
        let payload = format!(
            "{UUID_A}\x1fa\x1fa\x1fNew\x1f3\x1e\
             {UUID_B}\x1fb\x1fb\x1fOld\x1f4\x1e\
             {{{UUID_A}}}\x1fa\x1fa\x1fNew\x1f3"
        );

        let mut first = window(UUID_A, ("a", "a"), "New");
        let mut second = window(UUID_B, ("b", "b"), "Old");

        first.pid = Some(3);
        second.pid = Some(4);
        assert_eq!(parse(&payload), [first, second]);
    }
}
