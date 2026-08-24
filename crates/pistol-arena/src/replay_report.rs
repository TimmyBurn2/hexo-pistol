//! What a warm-replay pass IS, as a value, and the document it writes.
//!
//! Separated from the driver that produces it for the reason `record.rs` is
//! separated from `game.rs`: the consumer of a replay is a statistics layer in
//! another language, and it reads ONE shape rather than three.
//!
//! # Two kinds, told apart by the first token
//!
//! `warm_replay` covers every game of its source report. `warm_replay_aborted`
//! does not, and carries no `divergences` line at all: a criterion computed over
//! SOME of a report's games is a criterion over a sample nobody registered, so
//! no consumer may read one off a pass that did not finish. A different first
//! token rather than a flag, exactly as `report.rs` does it (docs/decisions.md
//! D-160).
//!
//! # The document binds itself to its source
//!
//! `source_report_sha256` is the digest of the WHOLE report file the pass was
//! taken from. The checker recomputes it and refuses a pair of documents that
//! are not about each other — which is the one mistake a two-file criterion can
//! make silently, and the only referent that catches it is one neither file
//! computes about itself.
//!
//! # `at_turn` is ONE-BASED
//!
//! It is the turn number of the game, the same count `report.rs`'s `turns` field
//! and `game::play`'s own `state.turn()` carry. The prefix a consumer must
//! rebuild to probe that position is therefore the FIRST `at_turn - 1` recorded
//! moves. One spelling per number: the zero-based index is not also written.

use std::fmt::Write as _;

use pistol_core::Turn;

use crate::error::ArenaError;
use crate::record::ForfeitReason;
use crate::transcript::Transcript;

/// The first token of a pass that covered its whole source report.
pub const REPLAY_KIND: &str = "warm_replay";
/// The first token of a pass that did not.
pub const ABORTED_KIND: &str = "warm_replay_aborted";
/// The document format version.
pub const REPLAY_SCHEMA: u32 = 1;
/// Where the pass's own machine-dependent block starts.
pub const TIMING_MARKER: &str = "# timing";

/// What an engine answered where the record says something else.
#[derive(Debug, Clone)]
pub enum Answered {
    /// It answered a turn, and the turn is not the recorded one.
    Move(Turn),
    /// It stopped playing legally, where the report has a recorded move.
    Forfeit {
        /// Why.
        reason: ForfeitReason,
        /// The line that earned it, verbatim, when there was one.
        line: Option<String>,
    },
}

/// The first — and, because the replay halts, the only — disagreement in a game.
#[derive(Debug, Clone)]
pub struct Divergence {
    /// The turn of the game, one-based.
    pub at_turn: usize,
    /// The label the report credits with that turn's move.
    pub mover_label: String,
    /// Which engine that is, `0` for A.
    pub mover_engine: usize,
    /// What the report records.
    pub recorded: Turn,
    /// What the engine said instead.
    pub answered: Answered,
}

/// One game, replayed.
#[derive(Debug, Clone)]
pub struct GameReplay {
    /// Its index in the source report.
    pub index: usize,
    /// How many recorded turns the source holds for it.
    pub recorded_turns: usize,
    /// How many were fed before the replay ended. Short of `recorded_turns`
    /// exactly when the replay halted on a divergence.
    pub replayed_turns: usize,
    /// How many turns were compared against an engine's own answer. The book
    /// turns are nobody's search and are not comparable.
    pub compared_turns: usize,
    /// What each engine spent replaying, indexed `0` for A.
    pub nodes: [u64; 2],
    /// The disagreement, if there was one.
    pub divergence: Option<Divergence>,
}

/// Everything a pass produced.
pub struct Replayed {
    /// One per game of the source report, in index order. `None` where a game
    /// was never replayed because the pass was abandoned first.
    pub games: Vec<Option<GameReplay>>,
    /// Wall-clock milliseconds for the pass.
    pub wall_ms: u64,
    /// How many workers ran it.
    pub workers: usize,
}

impl Replayed {
    /// The games that were actually replayed.
    pub fn covered(&self) -> usize {
        self.games.iter().filter(|slot| slot.is_some()).count()
    }

    /// How many of them disagreed with the record.
    pub fn divergences(&self) -> usize {
        self.games
            .iter()
            .flatten()
            .filter(|game| game.divergence.is_some())
            .count()
    }
}

/// Render the document.
pub fn render(transcript: &Transcript, played: &Replayed, aborted: Option<&ArenaError>) -> String {
    let mut out = String::new();
    let complete = aborted.is_none() && played.covered() == played.games.len();
    let kind = if complete { REPLAY_KIND } else { ABORTED_KIND };
    let _ = writeln!(out, "{kind} {REPLAY_SCHEMA}");
    let _ = writeln!(out, "arena_version {}", env!("CARGO_PKG_VERSION"));
    let _ = writeln!(out, "source_report_sha256 {}", transcript.source_sha256);
    let _ = writeln!(
        out,
        "source_experiment_sha256 {}",
        transcript.experiment_sha256
    );
    let _ = writeln!(out, "budget nodes {}", transcript.budget_nodes);
    let _ = writeln!(out, "opening_turns {}", transcript.opening_turns);
    let _ = writeln!(out, "turn_cap {}", transcript.turn_cap);
    for (slot, section, identity) in [
        ("a", &transcript.engines[0], &transcript.identities[0]),
        ("b", &transcript.engines[1], &transcript.identities[1]),
    ] {
        // No path on this record. The source report already names the binary
        // and the config, and this document exists to be read BESIDE it.
        let _ = writeln!(
            out,
            "engine {slot} label {} binary_sha256 {} config_sha256 {} weights_sha256 {}",
            section.label, identity.binary_sha256, identity.config_sha256, identity.weights_sha256
        );
    }
    let _ = writeln!(out, "games {}", played.games.len());

    for game in played.games.iter().flatten() {
        let status = if game.divergence.is_some() {
            "divergence"
        } else {
            "clean"
        };
        let _ = writeln!(
            out,
            "replay {} recorded_turns {} replayed_turns {} compared_turns {} nodes_a {} nodes_b \
             {} status {status}",
            game.index,
            game.recorded_turns,
            game.replayed_turns,
            game.compared_turns,
            game.nodes[0],
            game.nodes[1],
        );
        if let Some(found) = &game.divergence {
            divergence(&mut out, game.index, found);
        }
    }

    if complete {
        let _ = writeln!(out, "divergences {}", played.divergences());
    } else {
        let _ = writeln!(
            out,
            "# the games above do not cover the source report: a criterion over some of a \
             report's games is a criterion over a sample nobody registered"
        );
        let _ = writeln!(
            out,
            "covered {} of {}",
            played.covered(),
            played.games.len()
        );
        if let Some(error) = aborted {
            let _ = writeln!(out, "aborted {} {error}", error.name());
        }
    }
    let _ = writeln!(
        out,
        "{TIMING_MARKER} — machine- and schedule-dependent; excluded from every comparison"
    );
    let _ = writeln!(
        out,
        "timing n_workers {} wall_ms {} hang_timeout_ms {}",
        played.workers, played.wall_ms, transcript.hang_timeout_ms
    );
    out
}

/// The two records one disagreement writes.
fn divergence(out: &mut String, index: usize, found: &Divergence) {
    let (answered, kind, reason) = match &found.answered {
        Answered::Move(turn) => (turn.to_string(), "move", String::from("none")),
        Answered::Forfeit { reason, .. } => {
            (String::from("none"), "forfeit", reason.token().to_string())
        }
    };
    let _ = writeln!(
        out,
        "divergence {index} at_turn {} mover {} mover_slot {} recorded {} answered {answered} \
         kind {kind} reason {reason}",
        found.at_turn,
        found.mover_label,
        if found.mover_engine == 0 { "a" } else { "b" },
        found.recorded,
    );
    if let Answered::Forfeit {
        line: Some(why), ..
    } = &found.answered
    {
        // Last field of its own record kind, so free text needs no quoting.
        let _ = writeln!(out, "divergence_line {index} {why}");
    }
}
