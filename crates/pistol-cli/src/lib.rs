//! `pistol-cli` — the line protocol, and the commands built on it.
//!
//! A library as well as a binary, because the protocol *is* a contract: it is
//! the `Engine` trait spelled as text (docs/decisions.md D-5) and what the
//! future API layer will adapt (CLAUDE.md rule 11), so it is tested in-process
//! rather than only through a pipe.
//!
//! It depends on pistol-core and pistol-engine and deliberately on neither
//! pistol-search nor pistol-eval: what this crate says to an engine, it says
//! through the trait.

pub mod budget_token;
pub mod corpus;
pub mod count;
pub mod fixture_loader;
pub mod fixtures;
pub mod flags;
pub mod perft;
pub mod protocol;
pub mod random_openings;
pub mod report;
pub mod selftest;
pub mod serve;
pub mod sha256;

pub use protocol::{Flow, Session};
pub use serve::serve;
