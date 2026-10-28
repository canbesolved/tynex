// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `tynex daemon`: startup checks and wiring.

use std::path::Path;
use std::process::ExitCode;

use crate::config::{Config, Loaded, load_or_create};

/// Exit code for an invalid config (`EX_CONFIG` from `sysexits.h`).
pub const EX_CONFIG: u8 = 78;

/// Runs the daemon until it is told to quit.
#[cfg(target_os = "linux")]
#[must_use]
pub fn run() -> ExitCode {
    let Some(path) = crate::platform::linux::paths::config_file() else {
        log::error!("cannot find the config directory: no home directory");
        return ExitCode::from(EX_CONFIG);
    };
    let config = match load_config(&path) {
        Ok(config) => config,
        Err(errors) => {
            for e in &errors {
                log::error!("{e}");
            }
            return ExitCode::from(EX_CONFIG);
        }
    };

    match start(&config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("{e}");
            ExitCode::FAILURE
        }
    }
}

/// Builds every part once, then runs the event loop until `Quit`.
#[cfg(target_os = "linux")]
fn start(config: &Config) -> Result<(), String> {
    use crate::adapters;
    use crate::core::queue::channel;
    use crate::core::switcher::Switcher;
    use crate::core::traits::WindowSource;
    use crate::modes::prompt::PromptMode;
    use crate::platform::linux::dbus::Bus;
    use crate::platform::linux::event_loop::{self, Parts};
    use crate::platform::linux::icon_theme::IconTheme;
    use crate::platform::linux::wayland::WaylandPresenter;
    use crate::platform::linux::{dbus_ipc, fonts};
    use crate::ui::painter::Painter;
    use crate::ui::render::icons::Icons;
    use crate::ui::render::text::Text;

    let bus = Bus::session().map_err(|e| e.to_string())?;

    bus.own_name(dbus_ipc::NAME).map_err(|e| e.to_string())?;

    let (tx, rx) = channel();
    let current = std::env::var("XDG_CURRENT_DESKTOP").ok();
    let mut source =
        adapters::select(config.main.desktop, current.as_deref(), &bus)
            .map_err(|e| e.to_string())?;
    let presenter = WaylandPresenter::connect().map_err(|e| e.to_string())?;
    let face = fonts::default_face(&config.main.font)
        .map_err(|e| format!("cannot load the font: {e}"))?;
    let text = Text::new(face);

    dbus_ipc::serve(&bus, tx.clone());
    source.start(tx).map_err(|e| e.to_string())?;
    log::info!("daemon started, desktop {:?}", config.main.desktop);

    let mut parts = Parts {
        app: Switcher::new(
            config.hotkeys.action,
            config.title_bar.strip_app_name,
            config.title_bar.sort_by,
            PromptMode,
        ),
        source,
        presenter,
        bus,
        painter: Painter::new(
            text,
            Icons::new(Box::new(IconTheme::default())),
            &config.main,
            &config.output_bar,
            &config.title_bar,
        ),
        rx,
    };

    let result = event_loop::run(&mut parts);

    parts.source.stop();
    log::info!("daemon stopped");
    result
}

/// Loads the config at `path`, writing the default file if missing.
///
/// # Errors
///
/// Returns one message per problem, each starting with the path.
pub fn load_config(path: &Path) -> Result<Config, Vec<String>> {
    match load_or_create(path) {
        Ok(Loaded::Existing(config)) => Ok(config),
        Ok(Loaded::Created(config)) => {
            log::warn!("created default config {}", path.display());
            Ok(config)
        }
        Err(errors) => Err(errors
            .iter()
            .map(|e| match e.position {
                Some(_) => format!("{}:{e}", path.display()),
                None => format!("{}: {e}", path.display()),
            })
            .collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn errors_start_with_the_path() {
        let dir = std::env::temp_dir()
            .join(format!("tynex_daemon_test_{}", std::process::id()));
        let path = dir.join("tynex.toml");

        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, "adaptr = \"kde\"\n").unwrap();

        let errors = load_config(&path).unwrap_err();

        assert!(
            errors[0]
                .starts_with(&format!("{}:1:1: unknown key", path.display()))
        );
        fs::write(&path, "").unwrap();
        assert_eq!(load_config(&path), Ok(Config::default()));
        fs::remove_dir_all(dir).unwrap();
    }
}
