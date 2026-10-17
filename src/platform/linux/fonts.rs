// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The font via `fc-match` or a configured path. The font file is
//! memory-mapped once and kept for the whole run.

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{io, ptr};

use rustix::mm::{MapFlags, ProtFlags, mmap};

use crate::config::Font;
use crate::ui::render::glyphs::Face;

/// Loads the font from the config: a path, or `fc-match` for `auto`.
///
/// # Errors
///
/// Fails when `fc-match` finds nothing or the file cannot be loaded.
pub fn default_face(font: &Font) -> io::Result<Face> {
    let (path, index) = match font {
        Font::Path(path) => (path.clone(), 0),
        Font::Auto => fc_match("sans-serif").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "fc-match found no sans-serif font",
            )
        })?,
    };

    load(&path, index)
}

/// Runs `fc-match` for `pattern` and returns the file and face index.
fn fc_match(pattern: &str) -> Option<(PathBuf, u32)> {
    let out = Command::new("fc-match")
        .args(["-f", "%{file}:%{index}", pattern])
        .output()
        .ok()
        .filter(|o| o.status.success())?;

    parse_match(&String::from_utf8(out.stdout).ok()?)
}

/// Parses `file:index`; fontconfig keeps the named instance of variable
/// fonts in the upper 16 bits of the index.
fn parse_match(line: &str) -> Option<(PathBuf, u32)> {
    let (file, index) = line.trim().rsplit_once(':')?;
    let index: u32 = index.parse().ok()?;

    (!file.is_empty()).then(|| (PathBuf::from(file), index & 0xffff))
}

/// Maps face `index` of the font file at `path`.
fn load(path: &Path, index: u32) -> io::Result<Face> {
    let data = map_file(path)?;

    Face::new(data, index).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "not a usable font file")
    })
}

/// Maps `path` read-only for the rest of the program.
///
/// Only the pages used for glyphs count towards RAM. The mapping is never
/// unmapped, because `Face` borrows it for the whole run.
fn map_file(path: &Path) -> io::Result<&'static [u8]> {
    let file = File::open(path)?;
    let len =
        usize::try_from(file.metadata()?.len()).map_err(io::Error::other)?;

    if len == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "empty font file",
        ));
    }

    // SAFETY: a private read-only mapping of a whole regular file, kept for
    // the program's lifetime. Font files are not expected to change while
    // mapped; system fonts are owned by root.
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            len,
            ProtFlags::READ,
            MapFlags::PRIVATE,
            &file,
            0,
        )?
    };

    // SAFETY: the mapping above is valid for `len` bytes and never unmapped.
    Ok(unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), len) })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FONT: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fonts/dejavu_sans.ttf");

    #[test]
    fn parses_fc_match_output() {
        assert_eq!(parse_match("/f/a.ttf:0\n"), Some(("/f/a.ttf".into(), 0)));
        assert_eq!(parse_match("/f/b:c.ttc:2"), Some(("/f/b:c.ttc".into(), 2)));
        // Variable font, named instance 4, face 0.
        assert_eq!(
            parse_match("/f/cjk.ttc:262144"),
            Some(("/f/cjk.ttc".into(), 0))
        );
        assert_eq!(parse_match(":0"), None);
        assert_eq!(parse_match("/f/a.ttf"), None);
    }

    #[test]
    fn configured_path_is_mapped() {
        assert!(default_face(&Font::Path(FONT.into())).is_ok());
        assert!(load(Path::new("/nonexistent.ttf"), 0).is_err());
    }
}
