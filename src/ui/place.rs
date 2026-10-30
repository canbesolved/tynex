// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Vertical placement of the popup: top margin and the fit limit.

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::config::Position;

/// Room kept free at the screen edge, logical pixels.
pub(crate) const SCREEN_MARGIN: f32 = 24.0;

/// Top margin in logical pixels.
///
/// `anchor` is the logical height locked when the popup opened. `center`
/// uses it; `top`, a missing screen and a missing anchor all use
/// `SCREEN_MARGIN`.
pub(crate) fn margin(
    position: Position,
    screen: Option<u32>,
    scale: f32,
    anchor: Option<u32>,
) -> i32 {
    let Some(top) = center_top(position, screen, scale, anchor) else {
        return margin_px();
    };

    top.max(SCREEN_MARGIN).round() as i32
}

/// Maximum popup height in physical pixels.
///
/// `top`, and `center` before the height is locked, keep `SCREEN_MARGIN`
/// above and below. A locked `center` keeps its top margin and
/// `SCREEN_MARGIN` below.
pub(crate) fn limit(
    position: Position,
    screen: u32,
    scale: f32,
    anchor: Option<u32>,
) -> f32 {
    let scale = scale.max(1.0);
    let top = margin(position, Some(screen), scale, anchor) as f32;

    screen as f32 - top * scale - SCREEN_MARGIN * scale
}

/// Centered top margin in logical pixels, when it can be computed.
fn center_top(
    position: Position,
    screen: Option<u32>,
    scale: f32,
    anchor: Option<u32>,
) -> Option<f32> {
    if position != Position::Center {
        return None;
    }
    let screen = screen?;
    let anchor = anchor?;
    let scale = scale.max(1.0);

    Some((screen as f32 / scale - anchor as f32) / 2.0)
}

/// `SCREEN_MARGIN` as an integer pixel count.
fn margin_px() -> i32 {
    SCREEN_MARGIN as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_margin_follows_the_locked_height() {
        let open = margin(Position::Center, Some(1000), 1.0, Some(80));
        let taller = margin(Position::Center, Some(1000), 1.0, Some(400));

        assert_eq!(open, 460);
        assert_ne!(open, taller);
    }

    #[test]
    fn top_margin_is_the_screen_clearance() {
        assert_eq!(margin(Position::Top, Some(1000), 1.0, Some(80)), 24);
        assert_eq!(margin(Position::Top, Some(1000), 2.0, Some(400)), 24);
        assert_eq!(margin(Position::Center, None, 1.0, Some(80)), 24);
        assert_eq!(margin(Position::Center, Some(1000), 1.0, None), 24);
    }

    #[test]
    fn center_margin_stays_on_screen() {
        assert_eq!(margin(Position::Center, Some(1000), 1.0, Some(990)), 24);
    }

    #[test]
    fn margin_uses_logical_screen_height() {
        assert_eq!(margin(Position::Center, Some(2000), 2.0, Some(100)), 450);
    }

    #[test]
    fn locked_center_has_a_tighter_limit() {
        let open = limit(Position::Center, 1000, 1.0, None);
        let locked = limit(Position::Center, 1000, 1.0, Some(80));
        let top = limit(Position::Top, 1000, 1.0, Some(80));

        assert_eq!(open, 1000.0 - 48.0);
        assert_eq!(top, open);
        assert_eq!(locked, 1000.0 - 460.0 - 24.0);
        assert!(locked < open);
    }
}
