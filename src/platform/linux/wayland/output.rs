// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Output sizes, so the popup can fit the screen.

use wayland_client::globals::GlobalList;
use wayland_client::protocol::wl_output::{self, Transform, WlOutput};
use wayland_client::{Connection, Dispatch, QueueHandle, WEnum};

use crate::platform::linux::wayland::state::Wl;

/// Current mode of one output, in physical pixels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct Output {
    pub width: u32,
    pub height: u32,
    pub rotated: bool, // Turned by 90 or 270 degrees
}

impl Output {
    /// Returns the height on screen, after rotation.
    fn shown_height(self) -> u32 {
        if self.rotated {
            self.width
        } else {
            self.height
        }
    }
}

/// Binds every output announced at start. Hotplugged outputs are not
/// tracked.
pub(super) fn bind_all(
    globals: &GlobalList,
    qh: &QueueHandle<Wl>,
) -> Vec<(WlOutput, Output)> {
    globals
        .contents()
        .clone_list()
        .into_iter()
        .filter(|g| g.interface == "wl_output")
        .enumerate()
        .map(|(index, g)| {
            let proxy = globals.registry().bind::<WlOutput, _, Wl>(
                g.name,
                g.version.min(4),
                qh,
                index,
            );

            (proxy, Output::default())
        })
        .collect()
}

/// Returns the shortest output height, if any output reported a mode.
pub(super) fn shortest(outputs: &[(WlOutput, Output)]) -> Option<u32> {
    outputs
        .iter()
        .map(|(_, o)| o.shown_height())
        .filter(|&h| h > 0)
        .min()
}

impl Dispatch<WlOutput, usize> for Wl {
    fn event(
        s: &mut Self,
        _: &WlOutput,
        e: wl_output::Event,
        &index: &usize,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let Some((_, output)) = s.outputs.get_mut(index) else {
            return;
        };

        match e {
            wl_output::Event::Mode {
                flags: WEnum::Value(flags),
                width,
                height,
                ..
            } if flags.contains(wl_output::Mode::Current) => {
                output.width = u32::try_from(width).unwrap_or(0);
                output.height = u32::try_from(height).unwrap_or(0);
            }
            wl_output::Event::Geometry {
                transform: WEnum::Value(transform),
                ..
            } => {
                output.rotated = matches!(
                    transform,
                    Transform::_90
                        | Transform::_270
                        | Transform::Flipped90
                        | Transform::Flipped270
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rotated_output_uses_its_width() {
        let flat = Output {
            width: 1920,
            height: 1080,
            rotated: false,
        };
        let turned = Output {
            rotated: true,
            ..flat
        };

        assert_eq!(flat.shown_height(), 1080);
        assert_eq!(turned.shown_height(), 1920);
    }
}
