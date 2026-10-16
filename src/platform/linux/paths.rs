// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! XDG paths, from the standard library only.

use std::ffi::OsString;
use std::path::PathBuf;

/// Returns `<config dir>/tynex/tynex.toml`, or `None` without a home.
#[must_use]
pub fn config_file() -> Option<PathBuf> {
    config_file_from(std::env::var_os("XDG_CONFIG_HOME"), std::env::home_dir())
}

/// Uses an absolute `$XDG_CONFIG_HOME`, else `~/.config`, per the XDG
/// spec, which says relative values are ignored.
fn config_file_from(
    xdg: Option<OsString>,
    home: Option<PathBuf>,
) -> Option<PathBuf> {
    let dir = xdg
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| home.map(|h| h.join(".config")))?;

    Some(dir.join("tynex").join("tynex.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xdg_config_home_wins_when_absolute() {
        let home = Some(PathBuf::from("/u")); // Made-up home

        assert_eq!(
            config_file_from(Some("/cfg".into()), home.clone()),
            Some("/cfg/tynex/tynex.toml".into())
        );
        assert_eq!(
            config_file_from(Some("relative".into()), home.clone()),
            Some("/u/.config/tynex/tynex.toml".into())
        );
        assert_eq!(
            config_file_from(None, home),
            Some("/u/.config/tynex/tynex.toml".into())
        );
        assert_eq!(config_file_from(None, None), None);
    }
}
