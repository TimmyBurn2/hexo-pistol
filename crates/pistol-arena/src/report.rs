//! The machine-readable report, and the line that says which kind it is.
//!
//! # Two blocks, because one required field cannot be worker-invariant
//!
//! Everything before the `# timing` marker is the VERDICT BLOCK, and one worker
//! and eight workers produce it byte for byte. Everything after is
//! machine- and schedule-dependent: engine `time_ms` changes the moment two
//! games contend for a core, and so does the count of in-flight games abandoned
//! at the stop. Rather than pretend otherwise, the report does what
//! `tools/determinism.sh` already does with `nps` and `time` — names the
//! non-comparable fields, segregates them, and excludes them by construction
//! (docs/decisions.md D-7, D-161).
//!
//! The claim is stated exactly: **a run that COMPLETES has a worker-invariant
//! verdict block. Whether a run completes is not worker-invariant**, because a
//! watchdog sized against one worker can fire at eight.
//!
//! # Two kinds, told apart by the first token
//!
//! `arena_report` carries a verdict. `arena_report_aborted` carries no
//! `verdict` line and no LLR at all: its games are a diagnostic and explicitly
//! not a sample, because which games finished before the abort depends on
//! timing. A different first-line token rather than a flag, so that no consumer
//! can read a verdict off a run that did not produce one (docs/decisions.md
//! D-160).
//!
//! # Format
//!
//! Line-oriented whitespace-delimited records in the fixture idiom this
//! workspace already reads and writes (docs/decisions.md D-147): lines end
//! `\n`, the file ends with exactly one, floats render at
//! [`FLOAT_DIGITS`]. Not JSON — this workspace has no serializer and D-139
//! ruled against growing one — and not TOML, because hand-emitting it means
//! hand-writing a quoting rule, which is the class of bug that turns a report
//! into a wrong answer. The one free-text field, an engine's verbatim refusal,
//! is the LAST field of its own record kind, so it needs no quoting at all.

use std::fmt::Write as _;

use crate::conclusion;
use crate::config::ArenaConfig;
use crate::error::ArenaError;
use crate::openings::Openings;
use crate::record::{Compute, GameRecord};
use crate::sprt::Bounds;

/// The first token of a report that carries a verdict.
pub const REPORT_KIND: &str = "arena_report";
/// The first token of a report from a run that was abandoned.
pub const ABORTED_KIND: &str = "arena_report_aborted";
/// The report format version.
pub const REPORT_SCHEMA: u32 = 1;
/// Where the verdict block ends and nothing comparable begins.
pub const TIMING_MARKER: &str = "# timing";

/// The fields that define the EXPERIMENT, hashed into the verdict block.
///
/// Not the config document's own digest, and the difference is the point. The
/// document also carries `n_workers` and `hang_timeout_ms`, which are run
/// mechanics rather than experiment parameters: two runs of the same experiment
/// at one worker and at eight are the same experiment, and a digest that moved
/// between them would put a scheduling knob inside the block those two runs
/// must agree on. Found by `two_worker_run_report_identical_to_single_worker`,
/// which is exactly what that test is for.
///
/// The document's own digest is still reported, in the timing block, because it
/// is provenance for the exact file that was used.
pub fn experiment_digest(written: &Written<'_>) -> String {
    let config = written.config;
    let (kind, value) = config.budget.report_tokens();
    let sprt = &config.sprt;
    let mut canonical = String::new();
    let _ = writeln!(canonical, "openings_body {}", written.openings.body_sha256);
    let _ = writeln!(canonical, "openings_take {}", config.run.openings_take);
    let _ = writeln!(canonical, "turn_cap {}", config.run.turn_cap);
    let _ = writeln!(canonical, "budget {kind} {value}");
    let _ = writeln!(
        canonical,
        "sprt {} {} {} {}",
        float(sprt.elo0),
        float(sprt.elo1),
        float(sprt.alpha),
        float(sprt.beta)
    );
    for (slot, engine, identity) in [
        ("a", &config.engine_a, &written.identities[0]),
        ("b", &config.engine_b, &written.identities[1]),
    ] {
        let _ = writeln!(
            canonical,
            "engine {slot} {} {} {}",
            engine.label, identity.binary_sha256, identity.config_sha256
        );
    }
    pistol_cli::sha256::sha256_hex(canonical.as_bytes())
}

/// Decimal places every float is rendered at.
///
/// Nine rather than six: a reader recomputing a verdict from the printed
/// numbers should not disagree with the run over a rounding. The comparison
/// itself is on the `f64`, and this is the rendering of it.
pub const FLOAT_DIGITS: usize = 9;

/// The identity of one engine, gathered before the first game.
#[derive(Debug, Clone)]
pub struct EngineIdentity {
    /// The handshake's `id` lines, verbatim.
    pub id_lines: Vec<String>,
    /// The digest of the binary that was actually run.
    pub binary_sha256: String,
    /// The digest of the config it was run with.
    pub config_sha256: String,
}

/// Everything a report is written from.
pub struct Written<'a> {
    /// The run's configuration.
    pub config: &'a ArenaConfig,
    /// The digest of that configuration's document.
    pub config_sha256: &'a str,
    /// The openings that were played from.
    pub openings: &'a Openings,
    /// Per engine, indexed `0` for A.
    pub identities: &'a [EngineIdentity; 2],
    /// The games of the verdict, in index order.
    pub records: &'a [GameRecord],
    /// Wall-clock milliseconds for the whole run.
    pub wall_ms: u64,
    /// Started games abandoned at the stop.
    pub discarded: usize,
    /// Why the run was abandoned, when it was.
    pub aborted: Option<&'a ArenaError>,
}

/// Render the report.
pub fn render(written: &Written<'_>) -> String {
    let mut out = String::new();
    let aborted = written.aborted.is_some();
    let kind = if aborted { ABORTED_KIND } else { REPORT_KIND };
    let _ = writeln!(out, "{kind} {REPORT_SCHEMA}");
    instrument(&mut out, written);
    conclusion::games(&mut out, written);
    if let Some(error) = written.aborted {
        let _ = writeln!(
            out,
            "# the games above are a diagnostic and not a sample: which of them finished before \
             the abort depends on timing"
        );
        let _ = writeln!(out, "aborted {} {error}", error.name());
    } else {
        conclusion::found(&mut out, written);
    }
    timing(&mut out, written);
    out
}

/// What was run, identified by content and not by path (docs/decisions.md D-147).
fn instrument(out: &mut String, written: &Written<'_>) {
    let config = written.config;
    let _ = writeln!(out, "arena_version {}", env!("CARGO_PKG_VERSION"));
    let _ = writeln!(out, "experiment_sha256 {}", experiment_digest(written));
    let _ = writeln!(out, "openings_file {}", config.run.openings_file.display());
    let _ = writeln!(out, "openings_body_sha256 {}", written.openings.body_sha256);
    let _ = writeln!(
        out,
        "openings_take {} of {}",
        config.run.openings_take, written.openings.total
    );
    let _ = writeln!(out, "opening_turns {}", written.openings.opening_turns);
    let _ = writeln!(out, "game_cap {}", config.run.openings_take * 2);
    let (kind, value) = config.budget.report_tokens();
    let _ = writeln!(out, "budget {kind} {value}");
    let _ = writeln!(out, "turn_cap {}", config.run.turn_cap);
    let sprt = &config.sprt;
    let _ = writeln!(
        out,
        "sprt elo0 {} elo1 {} alpha {} beta {}",
        float(sprt.elo0),
        float(sprt.elo1),
        float(sprt.alpha),
        float(sprt.beta)
    );
    let bounds = Bounds::of(sprt.alpha, sprt.beta);
    let _ = writeln!(
        out,
        "bounds h0 {} h1 {}",
        float(bounds.h0),
        float(bounds.h1)
    );
    for (slot, engine, identity) in [
        ("a", &config.engine_a, &written.identities[0]),
        ("b", &config.engine_b, &written.identities[1]),
    ] {
        let _ = writeln!(
            out,
            "engine {slot} label {} binary {} binary_sha256 {} config {} config_sha256 {}",
            engine.label,
            engine.binary.display(),
            identity.binary_sha256,
            engine.config.display(),
            identity.config_sha256
        );
        for line in &identity.id_lines {
            let _ = writeln!(out, "engine_id {slot} {line}");
        }
    }
}

/// Everything a second worker changes.
fn timing(out: &mut String, written: &Written<'_>) {
    let _ = writeln!(
        out,
        "{TIMING_MARKER} — machine- and schedule-dependent; excluded from every comparison"
    );
    let _ = writeln!(
        out,
        "timing n_workers {} wall_ms {} discarded_in_flight {} hang_timeout_ms {}",
        written.config.run.n_workers,
        written.wall_ms,
        written.discarded,
        written.config.run.hang_timeout_ms
    );
    // The document as it was on disk. Provenance, not identity: it also carries
    // the two knobs above, so it belongs on this side of the marker.
    let _ = writeln!(out, "timing config_sha256 {}", written.config_sha256);
    let mut totals = [Compute::default(); 2];
    for record in written.records {
        totals[0].absorb(record.compute[0]);
        totals[1].absorb(record.compute[1]);
    }
    for (slot, total) in [("a", totals[0]), ("b", totals[1])] {
        let _ = writeln!(
            out,
            "timing_engine {slot} time_ms {} searches {}",
            total.time_ms, total.searches
        );
    }
}

/// A float, rendered the one way this report renders floats.
pub(crate) fn float(value: f64) -> String {
    format!("{value:.FLOAT_DIGITS$}")
}

/// A value that may not exist. `none` rather than a substituted number, because
/// a degenerate sample has no LLR and inventing one is what this crate refuses
/// to do (docs/decisions.md D-156).
pub(crate) fn maybe(value: Option<f64>) -> String {
    match value {
        Some(value) => float(value),
        None => String::from("none"),
    }
}

/// The part of a report two worker counts must agree on, byte for byte.
pub fn verdict_block(report: &str) -> &str {
    // Anchored at a line start. Unanchored, any occurrence of the marker inside
    // an engine's VERBATIM refusal — which this format deliberately copies
    // through unquoted — would truncate the block early, and the two-worker
    // comparison would then be over a prefix chosen by the engine under test.
    let anchored = format!("\n{TIMING_MARKER}");
    match report.find(&anchored) {
        Some(at) => &report[..=at],
        None => report,
    }
}
