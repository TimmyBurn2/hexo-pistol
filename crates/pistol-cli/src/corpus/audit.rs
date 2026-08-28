use std::collections::BTreeMap;
use std::fmt;

use pistol_core::win::winning_run;
use pistol_core::{Board, FIRST_TURN, Player, WIN_LEN, stones_in_turn};

use super::record::Record;

// RULE9-JUSTIFICATION: the per-game replay, the aggregation it feeds and the
// rendered block are one instrument — the reviews found the aggregate and the
// renderer unbound while the per-game flags were bound, which is exactly the
// seam a split would put a file boundary through.

/// The move floor `source_filter` claims: "rated, >=20 moves, decisive by
/// six-in-a-row" (`dataset_metadata.json`).
pub const CLAIMED_MIN_MOVES: usize = 20;

/// What one game does or does not satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameAudit {
    /// How many stones the record holds.
    pub moves: usize,
    /// Whether the LAST stone completes a run of at least [`pistol_core::WIN_LEN`].
    ///
    /// The last stone specifically, not any stone: rule 4 ends a game the
    /// instant a placed stone completes a line, so a decisive record's final
    /// stone is the one that did it.
    pub decisive: bool,
    /// Whether any earlier stone already completed a run, which would mean the
    /// record continued past a finished game.
    pub decided_early: bool,
    /// Whether the record carries a rating for BOTH sides.
    ///
    /// Half of this is structurally unobservable: `record::parse` refuses a
    /// record with no `elo` key at all (`ELO_KEY_REQUIRED`), so a missing key
    /// can never reach here. What it does measure is the `null` case, which the
    /// reader accepts and which a corpus can exercise (REVIEW-impl m6).
    pub rated: bool,
    /// Whether the record's `winner` is the owner of the stone that won.
    ///
    /// `winner` is parsed and range-checked by the reader and was then never
    /// read: a record whose play contradicts its own outcome field passed the
    /// audit while contradicting itself (WP-P1d RED-TEAM F1).
    pub winner_agrees: bool,
    /// The length of the run the last stone completed, where it completed one.
    ///
    /// Reported because `source_filter` says "six-in-a-row" while game rule 2
    /// scores overlines as wins too: a corpus ending on runs of 7+ is evidence
    /// the SOURCE platform scores them the same way, which is independent
    /// corroboration of rule 2 against an exact-six variant (RED-TEAM F7).
    pub final_run: Option<u32>,
    /// Whether the record could not be replayed at all — a repeated cell.
    ///
    /// Kept apart from every rules-level verdict: wrong-shape input gets a
    /// named category, not a claim that the game was not decisive (rule 3).
    pub malformed: bool,
}

impl GameAudit {
    /// Whether this game satisfies the move floor.
    pub fn meets_floor(self) -> bool {
        self.moves >= CLAIMED_MIN_MOVES
    }
}

/// Replay one game and answer the auditable conjuncts.
///
/// Win detection is [`winning_run`] and the stones owed by a turn are
/// [`stones_in_turn`], both pistol-core's. This module does walk the turn
/// structure and assign ownership itself, the same `owed.min(remaining)` shape
/// `super::replay::group_turns` walks — stated plainly because an earlier
/// version of this comment claimed it computed neither, which was a stronger
/// claim than the code (WP-P1d REVIEW-impl m3). What it must never do is
/// re-implement geometry or win detection, and it does not.
pub fn audit_game(record: &Record) -> GameAudit {
    let rated = record.elo.iter().all(Option::is_some);
    let malformed = GameAudit {
        moves: record.moves.len(),
        decisive: false,
        decided_early: false,
        rated,
        winner_agrees: false,
        final_run: None,
        malformed: true,
    };

    let mut board = Board::empty();
    let mut decided_early = false;
    let mut final_run = None;
    let mut last_owner = Player::P1;

    let mut index = 0usize;
    let mut turn = FIRST_TURN;
    let mut owner = Player::P1;
    let last_index = record.moves.len().saturating_sub(1);
    while index < record.moves.len() {
        let owed = (stones_in_turn(turn) as usize).min(record.moves.len() - index);
        for offset in 0..owed {
            let at = index + offset;
            let stone = record.moves[at];
            if board.apply(stone, owner).is_err() {
                return malformed;
            }
            // By INDEX, not by comparing coordinates: the coordinate test is
            // correct only while `Board::apply` refuses a repeat, which is an
            // invariant that is not visible at this line (RED-TEAM F3).
            if at == last_index {
                last_owner = owner;
                final_run = winning_run(&board, stone).map(|run| run.len);
            } else if winning_run(&board, stone).is_some() {
                decided_early = true;
            }
        }
        index += owed;
        turn += 1;
        owner = owner.opponent();
    }

    GameAudit {
        moves: record.moves.len(),
        decisive: final_run.is_some_and(|len| len >= WIN_LEN),
        decided_early,
        rated,
        winner_agrees: final_run.is_some() && last_owner == record.winner,
        final_run,
        malformed: false,
    }
}

/// The audit over a whole corpus.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Audit {
    lengths: BTreeMap<usize, usize>,
    /// Games below the claimed move floor, by corpus index.
    pub short: Vec<usize>,
    /// Games whose last stone completes no run, by corpus index.
    pub indecisive: Vec<usize>,
    /// Games that continued past a completed run, by corpus index.
    pub decided_early: Vec<usize>,
    /// Games whose recorded `winner` is not the owner of the winning stone.
    pub winner_disagrees: Vec<usize>,
    /// Games that could not be replayed at all, by corpus index.
    pub malformed: Vec<usize>,
    /// How many decisive games ended on a run longer than [`WIN_LEN`].
    pub overlines: usize,
    /// How many games carry a rating for both sides.
    pub rated: usize,
    /// How many games were audited.
    pub total: usize,
}

impl Audit {
    /// Audit every game, in corpus order.
    pub fn of(records: &[Record]) -> Audit {
        let mut audit = Audit::default();
        for (index, record) in records.iter().enumerate() {
            let one = audit_game(record);
            audit.total += 1;
            *audit.lengths.entry(one.moves).or_insert(0) += 1;
            if !one.meets_floor() {
                audit.short.push(index);
            }
            // MA1: a record that ran PAST its win still contains the six, so
            // it satisfies `decisive by six-in-a-row`; its defect is rule-4
            // conformance and is counted in `decided_early`. Only a game with
            // no completed run anywhere fails the conjunct.
            if !one.decisive && !one.decided_early && !one.malformed {
                audit.indecisive.push(index);
            }
            if one.decided_early {
                audit.decided_early.push(index);
            }
            if one.rated {
                audit.rated += 1;
            }
            if one.malformed {
                audit.malformed.push(index);
            }
            if !one.winner_agrees && !one.malformed {
                audit.winner_disagrees.push(index);
            }
            if one.final_run.is_some_and(|len| len > WIN_LEN) {
                audit.overlines += 1;
            }
        }
        audit
    }

    /// The shortest game, or `None` over an empty corpus.
    pub fn min(&self) -> Option<usize> {
        self.lengths.keys().next().copied()
    }

    /// The longest game.
    pub fn max(&self) -> Option<usize> {
        self.lengths.keys().next_back().copied()
    }

    /// The median move count, by the lower-middle convention on an even count.
    pub fn median(&self) -> Option<usize> {
        if self.total == 0 {
            return None;
        }
        let want = (self.total - 1) / 2;
        let mut seen = 0usize;
        for (&length, &count) in &self.lengths {
            seen += count;
            if seen > want {
                return Some(length);
            }
        }
        None
    }

    /// Whether both auditable conjuncts of `source_filter` hold.
    ///
    /// A malformed record is not a verdict on the filter and is reported in its
    /// own category, but it still denies the audit: a corpus holding one is not
    /// a corpus this tool has read (rule 3).
    pub fn filter_holds(&self) -> bool {
        self.short.is_empty() && self.indecisive.is_empty() && self.malformed.is_empty()
    }

    /// Games that satisfy the `decisive by six-in-a-row` conjunct.
    ///
    /// A record that ran PAST its win is decisive — the six is there — and what
    /// is wrong with it is rule-4 conformance, which [`Audit::decided_early`]
    /// reports on its own. Folding it into the conjunct would announce that the
    /// dataset's metadata is false on the evidence of a different defect, and a
    /// false D-456 STOP is the expensive direction (WP-P1d REVIEW-impl MA1).
    pub fn decisive_games(&self) -> usize {
        self.total - self.indecisive.len()
    }

    /// Move-count deciles, so the shape of the distribution is reported and not
    /// only its ends: a floor of exactly 20 and a floor of 47 say different
    /// things about how a filter was applied.
    pub fn deciles(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for step in 0..=10usize {
            let want = (self.total.saturating_sub(1) * step) / 10;
            let mut seen = 0usize;
            for (&length, &count) in &self.lengths {
                seen += count;
                if seen > want {
                    out.push((step * 10, length));
                    break;
                }
            }
        }
        out
    }
}

impl fmt::Display for Audit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  games audited           {}", self.total)?;
        let (Some(min), Some(max)) = (self.min(), self.max()) else {
            return write!(f, "  no game was audited");
        };
        // `median` is taken from the same `let-else` as the ends rather than
        // defaulted: a code-side default here would print `median 0` confidently
        // instead of failing (RED-TEAM F4).
        let Some(median) = self.median() else {
            return write!(f, "  no game was audited");
        };
        writeln!(f, "  move count  min {min}  median {median}  max {max}")?;
        writeln!(f, "  percentile  move count")?;
        for (percentile, length) in self.deciles() {
            writeln!(f, "  {percentile:<11} {length}")?;
        }
        writeln!(
            f,
            "  >= {CLAIMED_MIN_MOVES} moves           {} of {} ({} short)",
            self.total - self.short.len(),
            self.total,
            self.short.len()
        )?;
        writeln!(
            f,
            "  last stone completes a run  {} of {} ({} not)",
            self.total - self.indecisive.len(),
            self.total,
            self.indecisive.len()
        )?;
        writeln!(
            f,
            "  continued past a win        {}",
            self.decided_early.len()
        )?;
        writeln!(
            f,
            "  ended on an overline (7+)   {}  (game rule 2 scores these as wins)",
            self.overlines
        )?;
        writeln!(
            f,
            "  `winner` disagrees with play {}",
            self.winner_disagrees.len()
        )?;
        writeln!(f, "  malformed (repeated cell)   {}", self.malformed.len())?;
        write!(
            f,
            "  rating present both sides   {} of {}",
            self.rated, self.total
        )
    }
}
