// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Path tests in private directories under the system temp dir.

use std::os::unix::fs::{PermissionsExt, symlink};

use super::*;

/// A fresh, private directory under the system temp dir.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("tynex_paths_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);

    DirBuilder::new().mode(0o700).create(&dir).unwrap();
    dir
}

fn uid() -> u32 {
    rustix::process::getuid().as_raw()
}

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

#[test]
fn runtime_dir_is_created_private() {
    let base = scratch("create");
    let dir = runtime_dir_from(Some(base.clone().into()), uid()).unwrap();
    let mode = fs::metadata(&dir).unwrap().mode() & 0o777;

    assert_eq!(dir, base.join("tynex"));
    assert_eq!(mode, 0o700);
    assert_eq!(runtime_dir_from(Some(base.clone().into()), uid()), Ok(dir));
    fs::remove_dir_all(base).unwrap();
}

#[test]
fn runtime_dir_needs_a_private_absolute_xdg_dir() {
    let base = scratch("base");

    assert!(runtime_dir_from(None, uid()).is_err());
    assert!(runtime_dir_from(Some("relative".into()), uid()).is_err());
    assert!(runtime_dir_from(Some(base.join("no").into()), uid()).is_err());

    let other = runtime_dir_from(Some(base.clone().into()), uid() + 1);

    assert!(other.unwrap_err().ends_with("is not owned by the user"));
    fs::set_permissions(&base, fs::Permissions::from_mode(0o755)).unwrap();

    let open = runtime_dir_from(Some(base.clone().into()), uid());

    assert!(open.unwrap_err().ends_with("is open to other users"));
    fs::remove_dir_all(base).unwrap();
}

#[test]
fn unsafe_tynex_dir_is_rejected() {
    let base = scratch("unsafe");
    let target = scratch("target");

    symlink(&target, base.join("tynex")).unwrap();

    let err = runtime_dir_from(Some(base.clone().into()), uid());

    assert!(err.unwrap_err().ends_with("is a symlink"));
    fs::remove_file(base.join("tynex")).unwrap();
    DirBuilder::new()
        .mode(0o750)
        .create(base.join("tynex"))
        .unwrap();
    fs::set_permissions(base.join("tynex"), fs::Permissions::from_mode(0o750))
        .unwrap();

    let err = runtime_dir_from(Some(base.clone().into()), uid());

    assert!(err.unwrap_err().ends_with("is open to other users"));
    fs::remove_dir_all(base).unwrap();
    fs::remove_dir_all(target).unwrap();
}

#[test]
fn write_atomic_replaces_the_file_with_mode_0600() {
    let dir = scratch("write");
    let path = write_atomic(&dir, "a.js", b"one").unwrap();

    assert_eq!(path, dir.join("a.js"));
    assert_eq!(write_atomic(&dir, "a.js", b"two").unwrap(), path);
    assert_eq!(fs::read(&path).unwrap(), b"two");
    assert_eq!(fs::metadata(&path).unwrap().mode() & 0o777, 0o600);
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1); // No temp file left
    fs::remove_dir_all(dir).unwrap();
}
