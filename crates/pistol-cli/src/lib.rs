//! `pistol-cli` — the line protocol, and the commands built on it.
//!
//! A library as well as a binary, because the protocol *is* a contract: it is
//! the `Engine` trait spelled as text (docs/decisions.md D-5) and the semantic
//! contract the future API layer will adapt (CLAUDE.md rule 11), so it is tested
//! in-process rather than only through a pipe. The binary is
//! [`serve`](serve::serve) with a stdin loop around it.
//!
//! - [`protocol`] — the verb layer: one input line, one call on the seam.
//! - [`budget_token`] — the `go` verb's budget grammar.
//! - [`count`] — one spelling per number, wherever this crate reads one.
//! - [`corpus`] — `corpus-extract`: human game records in, sha-pinned opening
//!   and bench fixtures out. It touches no engine and speaks no protocol; it is
//!   here because this crate holds the binaries and already owns the fixture
//!   forms (docs/decisions.md D-136).
//! - [`report`] — every line the engine writes, and exactly what is in each one.
//! - [`mod@serve`] — the read loop.
//! - [`fixtures`] — the sha-pinned tactical suite's format, and
//!   [`fixture_loader`] — the loader that reads it.
//! - [`selftest`] — the in-process determinism and tactical gate.
//! - [`sha256`] — the digest rule 7's fixture pins are computed with, and that
//!   `corpus-extract` writes into a fixture header.
//! - [`perft`] — the movegen count, for asking from a shell.
//!
//! This crate depends on pistol-core (for the stone and turn tokens, and for the
//! rules a driver validates its own game against) and on pistol-engine (for the
//! seam). It deliberately depends on neither pistol-search nor pistol-eval: what
//! this crate says to an engine, it says through the trait, and the engine
//! re-exports the reporting types it hands out.

pub mod budget_token;
pub mod corpus;
pub mod count;
pub mod fixture_loader;
pub mod fixtures;
pub mod perft;
pub mod protocol;
pub mod report;
pub mod selftest;
pub mod serve;
pub mod sha256;

pub use protocol::{Flow, Session};
pub use serve::serve;
