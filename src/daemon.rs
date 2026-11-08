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
    use crate::core::traits::WindowSource;
    use crate::platform::linux::dbus::Bus;
    use crate::platform::linux::event_loop::{self, Parts};
    use crate::platform::linux::icon_theme::IconTheme;
    use crate::platform::linux::signals::Signals;
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
    let mut presenter =
        WaylandPresenter::connect().map_err(|e| e.to_string())?;

    presenter.set_position(config.main.position);
    presenter.set_us_layout(config.main.switch_layout);
    let face = fonts::default_face(&config.main.font)
        .map_err(|e| format!("cannot load the font: {e}"))?;
    let text = Text::new(face);
    let (apps, app_list, pinned) = catalog(config);

    crate::platform::linux::hint_file::prepare(&config.input.hint);
    let mut painter = Painter::new(
        text,
        Icons::new(Box::new(IconTheme::default())),
        &config.main,
        &config.output,
        &config.input.hint,
        &config.taskbar,
    );

    painter.prepare_icons(
        app_list
            .iter()
            .chain(&pinned)
            .filter_map(|a| a.icon.as_deref()),
        presenter.scale(),
    );
    dbus_ipc::serve(&bus, tx.clone());
    source.start(tx).map_err(|e| e.to_string())?;
    log::info!("daemon started, desktop {:?}", config.main.desktop);

    let mut parts = Parts {
        app: switcher(config, app_list, pinned, &apps),
        source,
        presenter,
        bus,
        painter,
        apps,
        hint: crate::platform::linux::hint::Hint::new(&config.input.hint),
        rx,
        tabs: adapters::browser_tabs::start(&config.browser_tabs),
        signals: Signals::open(),
        wait: std::time::Duration::from_millis(config.browser_tabs.wait.into()),
        hold: None,
        counter: crate::platform::linux::counter_file::SwitchCounter::new(
            config.main.switch_counter,
        ),
    };

    let result = event_loop::run(&mut parts);

    drop(parts.signals.take()); // A second Ctrl+C now ends at once
    parts.source.stop();
    log::info!("daemon stopped");
    result
}

/// Builds the switcher and its prompt mode, with the saved usage
/// scores.
#[cfg(target_os = "linux")]
fn switcher(
    config: &Config,
    app_list: Vec<crate::core::app::AppInfo>,
    pinned: Vec<crate::core::app::AppInfo>,
    apps: &crate::platform::linux::apps::Apps,
) -> crate::core::switcher::Switcher<crate::modes::prompt::PromptMode> {
    use crate::core::switcher::Switcher;
    use crate::modes::prompt::PromptMode;
    use crate::modules::input::Input;
    use crate::platform::linux::usage_file;

    let mode = PromptMode::new(
        Input::new(config.input.launcher.clone(), app_list),
        config.taskbar.mode,
        config.output.max_results,
    )
    .with_tabs(config.browser_tabs.enabled)
    .with_hotkeys(&config.hotkeys)
    .with_select_on_hover(
        config.taskbar.titlebar.select_on_hover,
        config.taskbar.iconbar.select_on_hover,
    );
    let ids = Some(apps.ids()).filter(|ids| !ids.is_empty());

    Switcher::new(
        config.hotkeys.action,
        config.taskbar.titlebar.strip_app_name,
        config.taskbar.titlebar.sort_by,
        mode,
    )
    .with_pinned(pinned)
    .with_usage(usage_file::load(), ids)
    .with_history(config.main.history.into())
}

/// Scans the app catalog once. Returns it with the launcher list and
/// the pinned apps.
#[cfg(target_os = "linux")]
fn catalog(
    config: &Config,
) -> (
    crate::platform::linux::apps::Apps,
    Vec<crate::core::app::AppInfo>,
    Vec<crate::core::app::AppInfo>,
) {
    let ids = &config.taskbar.iconbar.pinned;
    let apps = crate::platform::linux::apps::Apps::scan(
        config.input.launcher.mode,
        ids,
    );
    let list = apps.list();
    let pinned = apps.pinned(ids);

    (apps, list, pinned)
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
