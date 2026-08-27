//! The lines the engine writes, and exactly what is in each one.
//!
//! Every field name and every token here is part of the protocol contract
//! (docs/decisions.md D-5), so they are written once, in this module, and both
//! the engine loop and the tests read them from here.
//!
//! # What two runs must agree on
//!
//! Every field of an `info` line except [`NPS_FIELD`] and [`TIME_FIELD`]. Those
//! two measure the machine rather than the search, which is why the
//! cross-process determinism gate strips them and compares the rest — the move,
//! the node count, the score, the depth and the whole principal variation
//! (CLAUDE.md rule 4, docs/decisions.md D-7).

use pistol_core::Turn;
use pistol_engine::{EngineError, ScoreKind, SearchInfo, classify};

/// The reply that ends the `pistol` handshake.
pub const HANDSHAKE_OK: &str = "pistolok";
/// The word every identity line starts with.
pub const ID_PREFIX: &str = "id";
/// The word every per-depth report starts with.
pub const INFO_PREFIX: &str = "info";
/// The word the answer starts with.
pub const BESTMOVE_PREFIX: &str = "bestmove";
/// The word every refusal starts with.
pub const ERROR_PREFIX: &str = "error";

/// Nodes per second: a measurement of the machine, not of the search.
pub const NPS_FIELD: &str = "nps";
/// Wall-clock milliseconds: likewise.
pub const TIME_FIELD: &str = "time";

/// The marker that distinguishes the closing report from a per-depth one.
///
/// The last `info` line before `bestmove` carries the last completed depth's
/// line and score with the **totals** for the whole search, so it repeats a depth
/// already reported with a different node count (docs/decisions.md D-80). Without
/// a marker those two lines differ only in numbers, and a driver keying on field
/// names could not tell which one to bill the search to; per-side compute is a
/// reporting requirement (CLAUDE.md rule 6), so the distinction is in the
/// grammar rather than in the reader's memory of what came before.
pub const TOTALS_MARKER: &str = "totals";

/// One per-depth report.
///
/// The key set is exactly this ordered list, and a driver keys on the names
/// rather than on positions: `[totals] depth_turns seldepth nodes nps time
/// hashfull score pv`. `score` is one of `cp <n>`, `mate <turns>` or `-mate
/// <turns>`; `pv` comes last because it is variable-length, and it is never
/// empty — a completed iteration always has a move (docs/decisions.md D-78, and
/// pistol-search's `NO_MOVE_FROM_A_COMPLETED_ITERATION`).
pub fn info_line(info: &SearchInfo) -> String {
    render_info(info, false)
}

/// The closing report: the same fields, marked [`TOTALS_MARKER`].
pub fn totals_line(info: &SearchInfo) -> String {
    render_info(info, true)
}

/// One `info` line, with or without the totals marker.
fn render_info(info: &SearchInfo, totals: bool) -> String {
    let marker = if totals {
        format!(" {TOTALS_MARKER}")
    } else {
        String::new()
    };
    // STRICTLY AFTER `nodes`, and ONLY when the solver ran (design
    // wp18b §3 and the dispatch's own "prints search nodes and solver
    // nodes separately every turn"): gate-off searches print neither, so
    // every committed config's line is byte-identical to the pre-wiring
    // engine's (D-88's pinned order stands); the order is load-bearing
    // for the one substring parser in the tree (`tools/sealbot`),
    // word-boundary-fixed at this WP — a report test pins both.
    let solver_field = if info.solver_nodes > 0 {
        format!(
            " search_nodes {} solver_nodes {}",
            info.search_nodes, info.solver_nodes
        )
    } else {
        String::new()
    };
    let mut line = format!(
        "{INFO_PREFIX}{marker} depth_turns {} seldepth {} nodes {}{solver_field} {NPS_FIELD} {} \
         {TIME_FIELD} {} hashfull {} score {} pv",
        info.depth_turns,
        info.seldepth_turns,
        info.nodes,
        info.nps,
        info.time_ms,
        info.hashfull_permille,
        score_token(info.score),
    );
    for turn in &info.pv {
        line.push(' ');
        line.push_str(&turn.to_string());
    }
    line
}

/// One identity line: `id <rest>`, folded to one line.
pub fn id_line(rest: &str) -> String {
    format!("{ID_PREFIX} {}", one_line(rest))
}

/// The move the engine would play, as a turn token: `"q,r"` or `"q,r/q,r"`
/// canonically ordered (docs/decisions.md D-5, D-49).
pub fn bestmove_line(best: Turn) -> String {
    format!("{BESTMOVE_PREFIX} {best}")
}

/// The longest refusal this engine writes.
///
/// A refusal names the input it refused, and that input came from whoever is on
/// the other end of the pipe: a ten-megabyte line must not be answered with ten
/// megabytes. The cap is the backstop rather than the first defence — the verb,
/// the budget word and the quoted line are each shortened where they are read —
/// but a message can be assembled from a token this module never sees (a parser's
/// own explanation, for instance), so the answer is bounded once, here, where
/// every refusal passes.
///
/// Generous by design: no explanation this workspace writes comes close, so the
/// cap fires only on input that is trying to make it fire.
pub const MAX_REFUSAL_CHARS: usize = 1024;

/// A refusal: the named error, and why.
///
/// One line, always, bounded, and the engine stays alive afterwards (CLAUDE.md
/// rule 3, docs/decisions.md D-5). A newline inside the explanation would split
/// one refusal into two lines and desynchronize a driver, so it is folded rather
/// than trusted not to be there; a control byte is escaped for the same reason
/// the rest of a quoted line is.
pub fn error_line(error: &EngineError) -> String {
    let line = format!(
        "{ERROR_PREFIX} {}: {}",
        error.name(),
        one_line(&error.detail())
    );
    if line.chars().count() <= MAX_REFUSAL_CHARS {
        return line;
    }
    let head: String = line.chars().take(MAX_REFUSAL_CHARS).collect();
    format!("{head}…")
}

/// A score, as the protocol spells it.
///
/// `mate <turns>` counts every turn from the root, both sides', so a win for the
/// side to move is always an odd distance and a loss an even one; `-mate
/// <turns>` is that loss (docs/decisions.md D-3, D-72). `cp <n>` is the static
/// evaluation in the integer units pistol-eval works in — there is no pawn on
/// this board to be a hundredth of, and inventing a conversion factor would
/// make the number less honest, not more familiar.
pub fn score_token(score: i32) -> String {
    match classify(score) {
        ScoreKind::Eval(value) => format!("cp {value}"),
        ScoreKind::MateIn(turns) => format!("mate {turns}"),
        ScoreKind::MatedIn(turns) => format!("-mate {turns}"),
    }
}

/// Whether [`one_line`] would leave this text exactly as it is.
///
/// The folding below is LOSSY and not reversible: a control character becomes
/// `?`, and `?` is a character a path may legitimately contain. That is the
/// right trade for a refusal's free-form explanation, which is prose and only
/// has to reach the driver on one line. It is the wrong one for an identity
/// VALUE, which somebody re-runs the engine from — `id config inst?v0.toml`
/// names a document that does not exist, and `tools/baseline_snapshot.sh`
/// writes that line into a record beside its own RAW copy of the same path, so
/// the two disagree for a reason that has nothing to do with the config
/// (docs/decisions.md D-198, D-324).
///
/// So this predicate is exported rather than the escape: a caller whose value
/// must survive verbatim asks BEFORE handing it over and refuses by name
/// (CLAUDE.md rule 3), and because the question and the fold read the one
/// expression here, a guard cannot come to guard something other than what is
/// written.
pub fn travels_verbatim(text: &str) -> bool {
    !text.chars().any(char::is_control)
}

/// Collapse anything multi-line into one line, and escape what would otherwise
/// travel down the pipe as a control byte.
fn one_line(text: &str) -> String {
    if travels_verbatim(text) {
        return text.to_string();
    }
    let folded = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("; ");
    folded
        .chars()
        .map(|c| if c.is_control() { '?' } else { c })
        .collect()
}
