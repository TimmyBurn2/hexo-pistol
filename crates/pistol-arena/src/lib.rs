//! `pistol-arena` — the match runner that turns a change into a verdict.
//!
//! Hard rule 6 makes SPRT over paired balanced openings the judge of every
//! search and eval change, so this crate is the judge every later work package
//! is tried by. Its failure mode is not slowness; it is returning a number that
//! is not a measurement, and every decision in it is argued against that.
//!
//! It is not the Stage-5 harness, and it holds no code specific to any external
//! engine — both sides are the pistol CLI speaking the line protocol. Match
//! logs are artifacts and are never committed (CLAUDE.md rule 8).

pub mod capture;
pub mod capture_file;
pub mod census_file;
pub mod channel;
pub mod conclusion;
pub mod config;
pub mod dedupe;
pub mod error;
pub mod exchange;
pub mod game;
pub mod handshake;
pub mod identity;
pub mod label_cache;
pub mod labels;
pub mod labels_file;
pub mod openings;
pub mod outpath;
pub mod passes;
pub mod reap;
pub mod record;
pub mod replay;
pub mod replay_report;
pub mod report;
pub mod schedule;
pub mod score;
pub mod seats;
pub mod sprt;
pub mod summary;
pub mod transcript;
pub mod usage;
mod validate;

pub use config::ArenaConfig;
pub use error::ArenaError;
pub use sprt::{Bounds, Sample, Unit, Verdict};
