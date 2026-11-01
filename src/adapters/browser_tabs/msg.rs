// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! JSON messages of the tab bridge, version 1.
//!
//! A frame is read twice: once for `v` and `type`, skipping the rest,
//! then into the struct of that type. No `Value` tree is built.

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::adapters::browser_tabs::frame;
use crate::core::browser_tabs::TabInfo;

/// A parsed message from a browser.
#[derive(Debug, PartialEq)]
pub enum Incoming {
    Hello {
        pid: u32,
    }, // 0 when the host could not tell
    Tabs {
        req: u64,
        browser: String,
        tabs: Vec<TabInfo>,
    },
    Icons(Vec<Icon>),
    Activated {
        req: u64,
        ok: bool,
        window: i32,
    },
    Skip, // Unknown type, another version, or bad fields
}

/// One favicon, still base64 text.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Icon {
    pub key: String,
    pub png: String,
}

#[derive(Deserialize)]
struct Head {
    v: Option<u64>,
    #[serde(rename = "type")]
    kind: Option<String>,
}

#[derive(Deserialize)]
struct HelloBody {
    pid: u64,
}

#[derive(Deserialize)]
struct TabsBody {
    req: u64,
    browser: String,
    tabs: Vec<Row>,
}

#[derive(Deserialize)]
struct Row {
    id: i64,
    window: i64,
    active: bool,
    last: f64,
    title: String,
    url: String,
    icon: String,
}

#[derive(Deserialize)]
struct IconsBody {
    icons: Vec<Icon>,
}

#[derive(Deserialize)]
struct ActivatedBody {
    req: u64,
    ok: bool,
    window: i64,
}

/// Parses one frame.
///
/// # Errors
///
/// Returns an error when the bytes are not a JSON object.
pub fn parse(bytes: &[u8]) -> Result<Incoming, ()> {
    let head: Head = serde_json::from_slice(bytes).map_err(|_| ())?;

    if head.v != Some(1) {
        return Ok(Incoming::Skip);
    }
    Ok(match head.kind.as_deref() {
        Some("hello") => body(bytes, |b: HelloBody| Incoming::Hello {
            pid: u32::try_from(b.pid).unwrap_or(0),
        }),
        Some("tabs") => body(bytes, |b: TabsBody| Incoming::Tabs {
            req: b.req,
            browser: b.browser,
            tabs: b.tabs.into_iter().filter_map(tab).collect(),
        }),
        Some("icons") => body(bytes, |b: IconsBody| Incoming::Icons(b.icons)),
        Some("activated") => {
            body(bytes, |b: ActivatedBody| Incoming::Activated {
                req: b.req,
                ok: b.ok,
                window: i32::try_from(b.window).unwrap_or(-1),
            })
        }
        _ => Incoming::Skip,
    })
}

/// `list` frame for request `req`.
#[must_use]
pub fn list(req: u64) -> Vec<u8> {
    frame::encode(format!(r#"{{"v":1,"type":"list","req":{req}}}"#).as_bytes())
}

/// `activate` frame for tab `id`.
#[must_use]
pub fn activate(req: u64, id: i32) -> Vec<u8> {
    frame::encode(
        format!(r#"{{"v":1,"type":"activate","req":{req},"id":{id}}}"#)
            .as_bytes(),
    )
}

/// Reads the body of a known type. Bad fields make it `Skip`.
fn body<T: DeserializeOwned>(
    bytes: &[u8],
    make: impl FnOnce(T) -> Incoming,
) -> Incoming {
    serde_json::from_slice(bytes).map_or(Incoming::Skip, make)
}

/// A row with ids that fit browser ids, else `None`.
fn tab(row: Row) -> Option<TabInfo> {
    Some(TabInfo {
        id: i32::try_from(row.id).ok()?,
        window: i32::try_from(row.window).ok()?,
        active: row.active,
        last: millis(row.last),
        title: row.title,
        url: row.url,
        icon: row.icon,
    })
}

/// Milliseconds as sent by the browser, `0` when not a positive number.
fn millis(value: f64) -> u64 {
    if value.is_finite() && value > 0.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        return value as u64;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{Incoming, parse};

    #[test]
    fn reads_each_type() {
        let hello = parse(br#"{"v":1,"type":"hello","pid":4}"#).unwrap();

        assert_eq!(hello, Incoming::Hello { pid: 4 });
        assert_eq!(
            parse(br#"{"v":2,"type":"hello","pid":4}"#).unwrap(),
            Incoming::Skip
        );
        assert_eq!(
            parse(br#"{"v":1,"type":"hello"}"#).unwrap(),
            Incoming::Skip
        );
        assert_eq!(parse(br#"{"v":1,"type":"x"}"#).unwrap(), Incoming::Skip);
        assert!(parse(b"not json").is_err());
        let done =
            br#"{"v":1,"type":"activated","req":2,"ok":true,"window":5}"#;

        assert_eq!(
            parse(done).unwrap(),
            Incoming::Activated {
                req: 2,
                ok: true,
                window: 5
            }
        );
    }

    #[test]
    fn tabs_keep_fields_and_drop_bad_ids() {
        let raw = concat!(
            r#"{"v":1,"type":"tabs","req":3,"browser":"Brave","tabs":["#,
            r#"{"id":1,"window":2,"active":true,"last":9.5,"title":"A","#,
            r#""url":"https://a.example","icon":"https://a.example"},"#,
            r#"{"id":4294967296,"window":2,"active":false,"last":-1,"#,
            r#""title":"B","url":"","icon":""}]}"#,
        );
        let Incoming::Tabs { req, tabs, .. } = parse(raw.as_bytes()).unwrap()
        else {
            panic!("tabs");
        };

        assert_eq!(req, 3);
        assert_eq!(tabs.len(), 1);
        assert_eq!((tabs[0].title.as_str(), tabs[0].last), ("A", 9));
        assert!(tabs[0].active);
    }
}
