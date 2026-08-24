//! `pistol-arena` — the match runner that turns a change into a verdict.
//!
//! Hard Rule 6 makes SPRT over paired balanced openings the judge of every
//! search and eval change, so this crate is the judge every later work package
//! is tried by. Its failure mode is not slowness; it is returning a number that
//! is not a measurement, and every decision in it is argued against that.
//!
//! - [`config`] / [`validate`] — the pre-registered experiment, complete and
//!   explicit, with the wall-clock budget refused by name (CLAUDE.md rule 1).
//! - [`openings`] — the fixture WP-1.2a emitted, verified against its own
//!   in-band digest and refused if it repeats an opening up to a symmetry.
//! - [`channel`] / [`handshake`] / [`reap`] — one engine subprocess, whether a
//!   strength claim may come from it, and the two ways it can stop talking.
//! - [`identity`] — who each engine is by content, captured before the first
//!   game and re-verified at every spawn.
//! - [`seats`] — spawning the engines one game is played between, setting them
//!   up and taking them down: ONE sequence, called by the generation path and by
//!   the replay mode alike, so the two cannot drift (docs/decisions.md D-408).
//! - [`game`] / [`exchange`] — the referee, and one question-and-answer with one
//!   engine. Every turn is validated through pistol-core, which is the only
//!   judge of legality in this workspace (rule 2).
//! - [`record`] — one finished game as a value.
//! - [`transcript`] / [`replay`] / [`replay_report`] — a report read back as the
//!   run it describes, that run re-driven WARM through its own engines, and what
//!   the pass found. The instrument Criterion 1'' is taken with
//!   (docs/decisions.md D-409).
//! - [`schedule`] — the workers, and a stop that fires only at pair boundaries.
//! - [`dedupe`] — distinct games, keyed on the canonical form of the played
//!   sequence under the twelve lattice symmetries.
//! - [`sprt`] / [`score`] — the estimator, and the verdict read off it.
//! - [`report`] / [`conclusion`] / [`summary`] — the artifact and the human
//!   sentence.
//!
//! # What this crate is not
//!
//! It is not the Stage-5 harness: no book generation, no BayesElo, no
//! pentanomial *manager* beyond paired bookkeeping. And it holds no code
//! specific to any external engine — both sides are the pistol CLI speaking the
//! line protocol, and an external opponent is the bridge's job.
//!
//! # Artifacts
//!
//! Match logs are artifacts and are never committed (CLAUDE.md rule 8). Nothing
//! here writes inside the repository, and the report path is always given
//! explicitly on the command line.

pub mod channel;
pub mod conclusion;
pub mod config;
pub mod dedupe;
pub mod error;
pub mod exchange;
pub mod game;
pub mod handshake;
pub mod identity;
pub mod openings;
pub mod outpath;
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
mod validate;

pub use config::ArenaConfig;
pub use error::ArenaError;
pub use sprt::{Bounds, Sample, Unit, Verdict};
