//! The v0 eval window — a re-export of the rules window `pistol-core` owns.
//!
//! A *v0* eval window and the rules window coincide: v0 scores length-six line
//! segments, and six is `WIN_LEN` because rule 2 says so. So this module is
//! [`pistol_core::window`] under its old name, and every path that imported
//! `pistol_eval::window::{Window, WINDOW_LEN, WINDOWS_PER_CELL,
//! windows_through}` still resolves (docs/decisions.md D-67, D-253).
//!
//! **The coincidence is v0's and not the trait's, which is why this is a
//! re-export rather than a use of core's module everywhere.** A window length
//! is a property of an eval backend: the Stage-2 codebook reads length-eleven
//! windows under the same [`Eval`](crate::Eval) trait. When it lands it defines
//! its own window type here, at its own length, and the two coexist — core's,
//! whose length is a rule and moves only when `WIN_LEN` moves, and the
//! backend's, whose length is a modelling choice. This module is where that
//! second one belongs; nothing in `pistol-core` will grow a length parameter to
//! host it.

pub use pistol_core::window::{
    WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through, windows_through_indexed,
};
