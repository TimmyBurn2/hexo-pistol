//! `pistol-core` — the one source of game truth.
//!
//! This crate will own, and no other crate may re-implement (CLAUDE.md rule 2):
//!
//! - the hex lattice: axial coordinates `(q, r)`, three line axes, six
//!   neighbours per cell, unbounded board;
//! - win detection: six or more contiguous own stones along one axis, overlines
//!   included, decided the instant a single placed stone completes a line;
//! - the turn structure: one stone on turn 1, two stones on every later turn,
//!   with the second stone of a winning turn never played;
//! - legal placement: the radius-8 legal region, as a named constant. It is a
//!   game rule, never a search knob, and it is never compared with the search
//!   candidate radius that lives in the config;
//! - pair-move generation and zobrist keys (lazy per-cell keys, with side to
//!   move and intra-turn phase in the key).
//!
//! Dependencies: `std` only, permanently.
//!
//! WP-01 is workspace scaffold, config, errors and CI; it writes no game logic.
//! The rules and their pinning tests land together in the rules work package —
//! a rules constant without its pinning test is worse than no constant at all
//! (docs/decisions.md D-28).
