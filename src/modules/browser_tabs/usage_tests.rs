// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! A tab list from a browser drops the usage scores of its closed tabs.

use super::{list, tab};
use crate::core::state::State;
use crate::core::usage::Usage;
use crate::modules::browser_tabs::Session;

#[test]
fn a_new_list_drops_the_scores_of_closed_tabs() {
    let mut state = Some(State::new(Vec::new()));
    let open = "https://a.example";

    state.as_mut().unwrap().usage = Usage::parse(
        "2\ttab\tBrave\thttps://a.example\n\
         2\ttab\tBrave\thttps://closed.example\n\
         2\ttab\tChrome\thttps://closed.example\n",
    );
    Session::default().apply(
        list(vec![tab(1, 1, true, 1, "A", open)]),
        &mut state,
        &[],
    );
    assert_eq!(
        state.unwrap().usage.to_text(),
        "2\ttab\tBrave\thttps://a.example\n\
         2\ttab\tChrome\thttps://closed.example\n"
    );
}
