//! `pistol-solver` — forcing-sequence machinery.
//!
//! Reserved purpose: threat generation first, then threat-space search and
//! dependency-based search, then the df-pn family. It opens here with the
//! THREAT STATE alone and grows only along the staged plan in
//! docs/research/minimax_report.md.
//!
//! # What is here
//!
//! [`ThreatState`] is an incremental, per-window record of what each side holds
//! — apply and undo one stone at a time — plus the ten sorted sets and the
//! integer queries a forcing search asks of them. It composes `pistol-core` and
//! re-implements no rule: the lattice, the win length, the window enumeration
//! and the turn structure all come from there (CLAUDE.md rule 2). It takes one
//! dependency and no dev-dependencies.
//!
//! # What is NOT here
//!
//! **Nothing in this crate decides a game.** The theorem it serves —
//! docs/decisions.md D-243 — is stones-remaining conditioned and side-to-move
//! conditioned, and both conditions live in the CALLER:
//! [`ThreatState::can_win_this_turn`] takes the stones left as an argument
//! rather than guessing, and [`ThreatState::unblockable_double_threat`] is a
//! statement about hitting sets that becomes a statement about the game only
//! under the two conditions its own doc names. A primitive that folded them in
//! would return a mate score for the losing side in exactly the position where
//! it matters.
//!
//! Nothing links this crate yet: no search, engine or binary calls it, which is
//! deliberate for this work package (docs/decisions.md D-249).
//!
//! # The surface is the queries, and nothing under them
//!
//! What this crate exports is [`ThreatState`], the ELEVEN QUERIES on it —
//! `query`'s eight and `cover`'s three — the closed conditioning types they
//! take, the answers they return, and [`WindowMasks`].
//!
//! NONE OF THE ELEVEN HANDS OUT A [`WindowMasks`]. The two methods that do,
//! [`ThreatState::masks`] and [`ThreatState::table_snapshot`], sit on the type
//! BESIDE the queries rather than among them, as do
//! [`ThreatState::window_count`] and [`ThreatState::is_empty`]; those four are
//! the whole of the public surface that is not a query, an argument type or an
//! answer.
//!
//! The STORE is not exported: the packed key, its hasher, the table type and
//! the class sets are `pub(crate)` and the modules holding them are private.
//! That is not tidiness — the whole ground for the table being its own file is
//! that a different store replaces exactly that file and nothing else, and
//! every internal name a consumer can reach is a commitment that replacement
//! would have to unwind (docs/decisions.md D-254, D-261).
//!
//! ## And the privacy is CHECKED, not only claimed
//!
//! A doc sentence about visibility is falsified by any commit that re-publishes
//! what it names, silently and with every gate green. The first example below
//! goes through the public door and compiles; the two after it reach for the
//! store's own and must not.
//!
//! ```
//! use pistol_core::window::Window;
//! use pistol_core::{Axis, Coord};
//! let window = Window::new(Axis::ConstR, Coord::new(0, 0)).unwrap();
//! let state = pistol_solver::ThreatState::new();
//! let _ = state.masks(window);
//! ```
//!
//! ```compile_fail
//! use pistol_core::window::Window;
//! use pistol_core::{Axis, Coord};
//! let window = Window::new(Axis::ConstR, Coord::new(0, 0)).unwrap();
//! let state = pistol_solver::ThreatState::new();
//! let _ = pistol_solver::table::empty_cells(window, state.masks(window));
//! ```
//!
//! ```compile_fail
//! let _ = pistol_solver::table::unpack(0);
//! ```
//!
//! A bare `compile_fail` passes on ANY compilation error, and the error-code
//! form does not repair it: this toolchain accepts `compile_fail,E0999` on code
//! whose real error is `E0603`, so the annotation validates nothing and is not
//! used. What makes the second example non-vacuous is the FIRST: every line of
//! it appears there and compiles, so the only line it can fail on is the one
//! that differs. The third is one line whose only other outcome is a type
//! error. WHAT THIS DOES NOT COVER: a re-export of the same item under another
//! path — `pub use table::unpack` here at the root — leaves both examples
//! failing, so that door is judged at the `pub use` list below and is not
//! mechanized (docs/decisions.md D-261).
//!
//! # Determinism
//!
//! The maintained sets are sorted `Vec<Window>` and every query hands out a
//! sorted slice or a sorted, deduplicated `Vec<Coord>`. The window TABLE is
//! hashed, which is legal precisely because it is never iterated on a choice
//! path — only [`ThreatState::table_snapshot`] enumerates it, and that sorts.
//! The hasher is splitmix64 with written-down constants and no seed state: no
//! `RandomState`, no per-process entropy, nothing that could make two runs of
//! the same position differ (CLAUDE.md rule 4, docs/decisions.md D-32, D-254).
//!
//! # Failure
//!
//! Being told about a stone that contradicts what this state already holds is
//! not operator input — it means a caller's board and this state have drifted —
//! so it panics with [`THREAT_DESYNC`] rather than returning an error nobody
//! could handle (CLAUDE.md rule 3).

pub mod cover;
pub mod query;
pub mod state;

mod sets;
mod table;

pub use cover::{Cover, MinimalCover};
pub use query::{HitBudget, LiveCount, NearHot, StonesLeft, WinWitness};
pub use state::{THREAT_DESYNC, ThreatState};
pub use table::WindowMasks;
