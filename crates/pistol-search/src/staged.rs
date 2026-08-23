//! Threat-first staged pair generation: the node protocol for
//! [`crate::params::CandidatePolicy::Staged`] (docs/decisions.md D-310, D-352;
//! `docs/wp15b_impl_prompt.md` §1, §3, §4; `docs/experiments/U2_node_protocol.md`
//! §5).
//!
//! `PROTO-NODE` steps 1-3 and step 4's tiers, per `U2_node_protocol.md` §5.1-§5.4:
//! win-now (Tier F), then the overload check (`LAW-OVERLOAD`, realised as
//! `blocking_covers`/`M5-E`), then the filtered or batched row's cells. D-scope
//! is stages F and T only (docs/decisions.md D-310); Tier Q's widening schedule
//! is deferred (`WPQ_seed.md`, D-315) and this module never runs it.
//!
//! `pub`, not `pub(crate)`: [`staged_candidates`] is the generator's one entry
//! point, made public so the differential gate's expensive half can drive it
//! from an integration test (`U2_node_protocol.md` §5.35, ADR item 17). Putting
//! a `pistol-solver` type in this crate's public API is a permanent surface
//! commitment named there, not decided here.
//!
//! # THE TIER-Q GAP THIS MODULE CLOSES, AND WHY IT IS OPEN DEBT AND NOT SETTLED
//! DESIGN
//!
//! `U3_tier_t.md` §U3-Z leaves OPEN whether the shipped D-scope surface keeps
//! `quiet_top_k`/`widen_schedule` at all, naming three branches: keep both keys
//! validated-and-inert, narrow to "Tier F ∪ Tier T with no quiet tier at all",
//! **"or something else"**. Tier F ∪ Tier T with no quiet tier at all is not a
//! safe choice: at the game's earliest plies no window anywhere has reached a
//! live count, so both tiers are PROVABLY EMPTY, and an empty candidate set at
//! the root is not a leaf — [`crate::search::Searcher::search`] panics with
//! `NO_MOVE_FROM_A_COMPLETED_ITERATION` reading an empty principal variation
//! back out of a node that returned a static value instead of a move. That is
//! the "or something else" branch this module takes: on a BATCHED row whose
//! Tier T is empty, [`staged_candidates`] falls back to the plain
//! `quiet_radius` ball (uncapped by `quiet_top_k`, which is stage Q's own knob
//! and not this net's) — the same ball [`crate::candidates::candidate_cells`]'s
//! `Staged` arm already answers with, so the net changes nothing about what a
//! caller may assume "the quiet ball" means. This is flagged here as **OPEN,
//! architect debt**, not asserted as settled design: it is IMPL choosing the
//! one branch that keeps the search from crashing, not a matrix-selected
//! option.
//!
//! # RULE9-JUSTIFICATION: one node protocol, over the one dispatch that
//! realises it (CLAUDE.md rule 9).
//!
//! `staged_candidates` and its four helpers (`tier_f`, `filtered`, `batched`,
//! `tier_t_union`/`tier_t_side`) are the SAME argument `U2_node_protocol.md`
//! §5.1-§5.4 states as one sequence of steps: which row a node takes decides
//! which helper runs, and the row decision itself (steps 1-3) is one match
//! with one query each. Splitting the tiers into separate files would pass
//! `params`/`threats`/`us`/`left` back and forth between them and put the
//! soundness argument for each row's boundary (Tier F empty on every BATCHED
//! row, the FILTERED union and nothing below it) in a different file from the
//! code it constrains. It grows again only if a fifth row or a fourth tier
//! arrives.

use pistol_core::{Coord, GameState, Player};
use pistol_eval::Eval;
use pistol_solver::{Cover, HitBudget, LiveCount, MinimalCover, StonesLeft, ThreatState};

use crate::candidates::within_radius;
use crate::params::StagedParams;

/// Named invariant: [`staged_candidates`] was asked about a position the rules
/// have already decided, where no turn's stones-owed count applies.
///
/// Unreachable through `pvs::visit`'s own recursion — `place` returns
/// `PlyOutcome::Win` and the parent scores without recursing into a decided
/// child, and `Searcher::check_root` refuses a decided root — so this is a
/// caller-order bug and not operator input (CLAUDE.md rule 3,
/// `U2_node_protocol.md` §5.2).
pub const OVERLOAD_ON_A_DECIDED_POSITION: &str = "OVERLOAD_ON_A_DECIDED_POSITION";

/// Which of the node protocol's rows a node took.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedRow {
    /// `can_win_this_turn` is `Some`: the emitted set is exactly the win-now
    /// class, and it is the whole set (`U2_node_protocol.md` §5.3's WIN-NOW
    /// row).
    WinNow,
    /// `None`, and `blocking_covers` answers `Minimal`: the emitted set is the
    /// cover union and nothing below it (the FILTERED row).
    Filtered,
    /// `None`, and `blocking_covers` answers `NothingToBlock`: Tier F
    /// (provably empty on this row) ∪ Tier T, or the quiet-ball safety net
    /// when Tier T is itself empty (see this module's doc).
    Batched,
    /// `None`, `blocking_covers` answers `Impossible`, and the node IS a PV
    /// node or the root: the same cells [`StagedRow::Batched`] would emit —
    /// the position IS lost, but a PV node must return the line that proves
    /// its score, so generation proceeds rather than returning early
    /// (`U2_node_protocol.md` §5.3's BATCHED-lost row). A distinct variant
    /// from [`StagedRow::Batched`], even though cell generation is identical,
    /// because the stage-share counters WP-1.6 reads need the
    /// `Cover::Impossible` rate kept apart from the `NothingToBlock` one
    /// (`U2_node_protocol.md` §U2-M item 2).
    BatchedLost,
    /// `None`, `blocking_covers` answers `Impossible`, and the node is NOT a
    /// PV node: `LAW-OVERLOAD`'s early return. `out` is left empty — the
    /// caller returns `-mate_in(turns_from_root + 2)` without expanding a
    /// child (`U2_node_protocol.md` §5.2's overload return; the `!is_pv` gate
    /// is `pvs::visit`'s own, not this function's, since only the caller knows
    /// `turns_from_root`).
    OverloadReturn,
}

/// The candidate cells [`staged_candidates`] emits, in search order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StagedSet {
    /// The cells, in the order the search should try them: Tier F (if any),
    /// never delta-ranked, ascending `(q, r)`; then Tier T, delta-ranked and
    /// stably sorted; then (BATCHED rows only, and only when Tier T is empty)
    /// the quiet-ball safety net, also delta-ranked.
    pub cells: Vec<Coord>,
    /// How many leading cells of [`StagedSet::cells`] are Tier F — the FORCED
    /// prefix. `crate::ordering::order` is never called under `Staged`
    /// (`U2_node_protocol.md` §5.4): the caller may promote the table's move
    /// only within `cells[forced..]`, never across this boundary
    /// ([`StagedSet::promote_table_move`]).
    pub forced: usize,
    /// Whether a BATCHED or BATCHED-lost row just filled found Tier T empty
    /// and used the quiet-ball safety net instead (this module's doc). `false`
    /// on the WIN-NOW and FILTERED rows, where it is never consulted. Read by
    /// `crate::info::StageCounters::record_quiet_safety_net`'s one caller.
    pub used_quiet_safety_net: bool,
}

impl StagedSet {
    fn clear(&mut self) {
        self.cells.clear();
        self.forced = 0;
        self.used_quiet_safety_net = false;
    }

    /// Promote `table_move` to the front of the UNFORCED range
    /// (`cells[forced..]`) if it is a candidate there — never across the
    /// Tier-F/Tier-T boundary (`U2_node_protocol.md` §5.4: "the table's move
    /// is promoted only within the tier it belongs to — never across a tier
    /// boundary, so the forced prefix stays a prefix"). On the WIN-NOW and
    /// FILTERED rows `forced == cells.len()`, so the unforced range is empty
    /// and this is a no-op by construction, not by a special case.
    pub(crate) fn promote_table_move(&mut self, table_move: Option<Coord>) {
        let Some(best) = table_move else {
            return;
        };
        let unforced = &mut self.cells[self.forced..];
        if let Some(found) = unforced.iter().position(|&at| at == best) {
            // Rotate rather than swap, the same reason `ordering::order` gives:
            // the cells behind it keep the order they arrived in.
            unforced[..=found].rotate_right(1);
        }
    }
}

/// `PROTO-NODE` steps 1-4: which row this node takes, and — for every row but
/// [`StagedRow::OverloadReturn`] — the cells it emits into `out`.
///
/// `is_pv` decides the one branch step 2's own text leaves to the caller
/// (`U2_node_protocol.md` §5.3's BATCHED-lost row: `Cover::Impossible` at a PV
/// node or the root generates Tier F ∪ Tier T rather than returning early) — a
/// plain `bool` the caller already has, not a new data dependency beyond the
/// three things §5.35 names this entry point must reach (the board, the threat
/// state, and a delta ranking).
///
/// `out` is cleared and refilled, never appended — the convention every buffer
/// this crate hands a caller follows.
///
/// # Panics
///
/// With [`OVERLOAD_ON_A_DECIDED_POSITION`] on a decided `state`.
pub fn staged_candidates(
    state: &GameState,
    threats: &ThreatState,
    eval: &mut dyn Eval,
    is_pv: bool,
    params: StagedParams,
    out: &mut StagedSet,
) -> StagedRow {
    out.clear();
    let us = state.to_move();
    let left = StonesLeft::from_state(state).unwrap_or_else(|| {
        panic!(
            "pistol-search invariant {OVERLOAD_ON_A_DECIDED_POSITION}: staged_candidates was \
             called on a decided position, where no turn's stones-owed count applies"
        )
    });

    // Step 1 (§5.1): win-now, realised as generation.
    if threats.can_win_this_turn(us, left).is_some() {
        tier_f_win_now(threats, us, left, out);
        return StagedRow::WinNow;
    }

    // Steps 2-3 (§5.2): one match, one query each — M5-E's realisation.
    match threats.blocking_covers(us, HitBudget::from(left)) {
        Cover::NothingToBlock => {
            batched(state.board(), threats, eval, us, params, out);
            StagedRow::Batched
        }
        Cover::Minimal(covers) => {
            filtered(&covers, out);
            StagedRow::Filtered
        }
        Cover::Impossible if !is_pv => StagedRow::OverloadReturn,
        Cover::Impossible => {
            batched(state.board(), threats, eval, us, params, out);
            StagedRow::BatchedLost
        }
    }
}

/// Tier F alone, on the WIN-NOW row: `U2_node_protocol.md` §5.3 proves it is
/// either the whole candidate set or empty, and this is the "whole set" case.
fn tier_f_win_now(threats: &ThreatState, us: Player, left: StonesLeft, out: &mut StagedSet) {
    tier_f(threats, us, left, &mut out.cells);
    out.forced = out.cells.len();
}

/// Tier F, `U2_node_protocol.md` §5.1: every size-one plan, at either budget,
/// plus — at [`StonesLeft::Two`] only — both empties of every own hot window
/// holding exactly four stones (`WinWitness::Pair`'s class in full).
///
/// The pair class is the union of ALL hot windows' empties
/// ([`ThreatState::threat_cells`]), not only the count-four ones: a
/// count-five hot window's one empty is already a win-in-one-ply cell — the
/// two classes overlap by construction at count five (`docs/decisions.md`
/// D-267) — so folding it into the union a second time changes nothing.
/// Merging the two queries' output and deduplicating is therefore exactly the
/// union §5.1 specifies.
fn tier_f(threats: &ThreatState, us: Player, left: StonesLeft, cells: &mut Vec<Coord>) {
    threats.win_in_one_ply_cells(us, cells);
    if left == StonesLeft::Two {
        let mut pair_cells = Vec::new();
        threats.threat_cells(us, &mut pair_cells);
        cells.extend(pair_cells);
        cells.sort_unstable();
        cells.dedup();
    }
}

/// The FILTERED row: the cover union alone, and nothing below it
/// (`U2_node_protocol.md` §5.3: "the union of cells over the inclusion-minimal
/// covers, and nothing below it"). `MinimalCover::cells` is not deduplicated
/// across covers on its own, so the union is sorted and deduplicated here —
/// mirroring `pistol_solver::Cover::cells`'s own construction, which this
/// function cannot call directly because the match in [`staged_candidates`]
/// already holds the `Vec<MinimalCover>` and not the enclosing `Cover`.
///
/// Not Tier F: `can_win_this_turn` is `None` on every row that reaches this
/// function, which `U2_node_protocol.md` §5.3 proves makes Tier F empty here.
/// The whole emitted set is therefore forced, but it is the survival set the
/// overload check just proved sufficient, never the win-now class — the two
/// are never conflated.
fn filtered(covers: &[MinimalCover], out: &mut StagedSet) {
    let mut cells: Vec<Coord> = covers.iter().flat_map(|cover| cover.cells()).collect();
    cells.sort_unstable();
    cells.dedup();
    out.cells = cells;
    out.forced = out.cells.len();
}

/// The BATCHED and BATCHED-lost rows: Tier T, delta-ranked — with the
/// quiet-ball safety net (this module's doc) when Tier T is itself empty.
///
/// Tier F is provably empty on every row this function is called from
/// (`U2_node_protocol.md` §5.3): `can_win_this_turn` is `None`, which at
/// `StonesLeft::Two` forbids both a win-in-one-ply cell and a hot window at
/// exactly four, and at `One` forbids the former while Tier F withholds the
/// pair class outright. `out.forced` stays `0`.
fn batched(
    board: &pistol_core::Board,
    threats: &ThreatState,
    eval: &mut dyn Eval,
    us: Player,
    params: StagedParams,
    out: &mut StagedSet,
) {
    let mut tier_t = tier_t_union(threats, us, params);
    if tier_t.is_empty() {
        // The safety net. See the module doc: at the game's earliest plies no
        // window anywhere has reached a live count, so Tier T is provably
        // empty too, and this is the branch that keeps the search from
        // reporting no move at all. The same ball `candidate_cells`'s `Staged`
        // arm answers with, uncapped — `quiet_top_k` is stage Q's own knob and
        // this D-scope does not arm stage Q.
        tier_t = within_radius(board, params.quiet_radius);
        out.used_quiet_safety_net = true;
    }
    delta_rank(&mut tier_t, eval, us);
    out.cells = tier_t;
    out.forced = 0;
}

/// Tier T's whole set: the union of both sides' `LAW-SUPPORT`-qualified cells
/// (`U3_tier_t.md` §10) — our own at `params.tier_t_own_count`, the opponent's
/// at `params.tier_t_opponent_count`.
fn tier_t_union(threats: &ThreatState, us: Player, params: StagedParams) -> Vec<Coord> {
    let mut cells = Vec::new();
    tier_t_side(threats, us, params.tier_t_own_count, &mut cells);
    let mut opponent = Vec::new();
    tier_t_side(
        threats,
        us.opponent(),
        params.tier_t_opponent_count,
        &mut opponent,
    );
    cells.extend(opponent);
    cells.sort_unstable();
    cells.dedup();
    cells
}

/// One side's Tier-T qualification at the threshold reading
/// (`U3_tier_t.md` §10, the union `LiveCount` alone cannot express):
///
/// ```text
/// threshold 2  ->  live_cells_at_count(Two) ∪ live_cells_at_count(Three) ∪ threat_cells
/// threshold 3  ->                             live_cells_at_count(Three) ∪ threat_cells
/// ```
///
/// `threshold` is validated to `{2, 3}` at `Searcher::new`, so the `<= 2` test
/// below is the whole of the branch — a third value never reaches this
/// function.
fn tier_t_side(threats: &ThreatState, side: Player, threshold: u8, out: &mut Vec<Coord>) {
    out.clear();
    let mut scratch = Vec::new();
    if threshold <= 2 {
        threats.live_cells_at_count(side, LiveCount::Two, &mut scratch);
        out.extend_from_slice(&scratch);
    }
    threats.live_cells_at_count(side, LiveCount::Three, &mut scratch);
    out.extend_from_slice(&scratch);
    threats.threat_cells(side, &mut scratch);
    out.extend_from_slice(&scratch);
    out.sort_unstable();
    out.dedup();
}

/// Score `cells` by [`Eval::delta`] for `mover` and sort descending, stably —
/// the same tie-break `crate::ordering::order` uses (docs/decisions.md D-5,
/// D-7): a stable sort by score leaves equal-scoring cells in the ascending
/// coordinate order they arrived in.
fn delta_rank(cells: &mut Vec<Coord>, eval: &mut dyn Eval, mover: Player) {
    let mut scored: Vec<(i32, Coord)> = cells
        .iter()
        .map(|&at| (eval.delta(at, mover), at))
        .collect();
    scored.sort_by_key(|&(score, _)| std::cmp::Reverse(score));
    cells.clear();
    cells.extend(scored.into_iter().map(|(_, at)| at));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `StagedSet::promote_table_move` is `pub(crate)`, reachable only from
    /// inside this crate — this is the one place a test can drive it directly
    /// rather than through the whole `pvs::visit` recursion.
    #[test]
    fn the_table_move_is_promoted_only_within_the_unforced_range() {
        let mut set = StagedSet {
            cells: vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)],
            forced: 1,
            used_quiet_safety_net: false,
        };
        let before = set.cells.clone();

        // A table move that is not a candidate at all: no-op.
        set.promote_table_move(Some(Coord::new(99, 99)));
        assert_eq!(set.cells, before, "a foreign table move changes nothing");

        // A table move that IS the forced cell itself: it lives at index 0,
        // strictly before `forced`, so it is not in the unforced range and
        // must not be touched — promoting a forced cell would let a Tier-T
        // table move masquerade as Tier F's own answer.
        set.promote_table_move(Some(before[0]));
        assert_eq!(
            set.cells, before,
            "a table move inside the forced prefix is never promoted"
        );

        // A table move inside the unforced range, not already at its front:
        // promoted, and the rest of that range keeps its order.
        set.promote_table_move(Some(before[2]));
        assert_eq!(
            set.cells,
            vec![before[0], before[2], before[1]],
            "the unforced range's own table move is promoted to its front, Tier F untouched"
        );
    }

    #[test]
    fn an_empty_unforced_range_survives_a_promotion_attempt() {
        // The WIN-NOW/FILTERED shape: forced == cells.len(), so the unforced
        // slice is empty. Promoting into it must not panic on an empty slice.
        let mut set = StagedSet {
            cells: vec![Coord::new(0, 0)],
            forced: 1,
            used_quiet_safety_net: false,
        };
        set.promote_table_move(Some(Coord::new(0, 0)));
        assert_eq!(set.cells, vec![Coord::new(0, 0)]);
    }
}
