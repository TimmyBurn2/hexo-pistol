/// The two identity columns of one firing, both derived from ONE state by ONE
/// function.
///
/// **IT IS A TYPE RATHER THAN TWO EXPRESSIONS AT EACH FIRING SITE, AND THE
/// REASON IS A MEASUREMENT.** The columns differ only in WHICH derivation lands
/// in WHICH field, and a review demonstrated that exchanging them at a call site
/// — the identity column carrying `GameState::key`, the option the design's F2
/// rules out — left **seventy tests green**, including the one written to catch
/// it. Deriving both here removes the DUPLICATED derivation and not the
/// exchange, and the test that kills an exchange reads an IN-TREE row's
/// identity against a referent replayed outside the search
/// (`docs/experiments/wp20b_B1_remedy.md`).
///
/// **THE SECOND HALF OF THAT ARGUMENT NO LONGER HOLDS AND IS WITHDRAWN.** It
/// used to say "each firing site still names the two fields when it builds its
/// row, so each is attacked by a registered mutant of its own" — true when the
/// root and the in-tree site were two constructions. Audit row A-05 was that
/// they were two, and `TriggerColumns::at` made them one, so there is one
/// site to attack and one mutant reaches it (D-675's sibling ruling). What
/// defends the exchange is now the referent test alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CensusKeys {
    /// The position up to transposition AND symmetry — the identity D-537's
    /// *disjoint positions* denominator is counted over.
    pub key: pistol_core::Key128,
    /// The same position under `GameState::key`: transpositions folded,
    /// symmetries NOT. Carried for the comparison
    /// `docs/experiments/wp20b_design.md` §9 registers, never for counting.
    pub key_pos: pistol_core::Key128,
}

impl CensusKeys {
    /// Both columns of the position `state` stands on.
    ///
    /// The one place either is computed. A caller cannot get one without the
    /// other and cannot choose which is which.
    pub fn at(state: &pistol_core::GameState) -> CensusKeys {
        let stones: Vec<(pistol_core::Coord, pistol_core::Player)> =
            state.board().stones().collect();
        CensusKeys {
            key: pistol_core::canonical_key(&stones),
            key_pos: state.key(),
        }
    }
}

/// One trigger firing, described by the O(1) facts a per-node detector could
/// read at it, plus what the solver then answered.
///
/// The census this feeds exists to rank an option field by MEASUREMENT rather
/// than by argument: every candidate narrowing of the trigger is a predicate
/// over these columns, so one instrumented run ranks the whole field on the
/// only axis that matters — what fraction of the present trigger's firings a
/// row keeps, and what fraction of the PROOFS it keeps with them (the premise
/// memo's §3.6, D-516).
///
/// **It is an observation and never an input.** Nothing in the search reads
/// one back, so recording them cannot move a move (CLAUDE.md rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerObservation {
    /// The position this firing happened at, as the identity that makes two
    /// firings on ONE position count as one: transpositions and symmetries
    /// both folded (`pistol_core::canonical_key`). It is what
    /// docs/decisions.md D-537's *disjoint positions* denominator is counted
    /// over, and the reason the census carries a key at all.
    pub key: pistol_core::Key128,
    /// The same firing's position under `GameState::key` — transpositions
    /// folded, symmetries NOT.
    ///
    /// Carried beside [`TriggerObservation::key`] because the symmetry fold's
    /// only measurement in this project is ZERO: the pilot corpus's three keys
    /// agreed at 347 distinct positions over 742 records (docs/decisions.md
    /// D-560), so at the ROOT population symmetry merged nothing. The census
    /// population is IN-TREE, where a search generates symmetric
    /// transpositions by construction, and whether the fold's yield there is
    /// materially above zero is settled by counting distinct values of these
    /// two columns over one run rather than argued
    /// (docs/experiments/wp20b_design.md §2's strongest surviving attack, §9).
    ///
    /// Free at the firing: the state carries it incrementally, and reading it
    /// is one XOR. It is NOT on the wire — §4's field order is pinned by a
    /// report test — and reaches a reader through
    /// `crates/pistol-search/examples/trigger_census.rs`, the instrument every
    /// census measurement in this arc was taken with.
    pub key_pos: pistol_core::Key128,
    /// What the position looked like at the decision.
    pub columns: TriggerColumns,
    /// Visits the attacker direction spent, and whether it proved.
    pub attacker: TriggerAnswer,
    /// The same for the defender direction. `None` when the attacker proved
    /// and the defender was therefore never asked.
    pub defender: Option<TriggerAnswer>,
}

/// What one direction of one firing cost and answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerAnswer {
    /// Visits this invocation spent.
    pub visits: u64,
    /// Whether it returned a proof. The expensive invocations are the ones
    /// that do not.
    pub proved: bool,
}

/// The columns a per-node detector could read AT the firing, before either
/// invocation answers.
///
/// Separate from [`TriggerObservation`] because they are read at a different
/// MOMENT: these describe the decision, the answers describe its outcome, and
/// carrying them as one tuple through the call sites is what made the seventh
/// column indistinguishable from the sixth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerColumns {
    /// Turns from the search root. 0 is the root's own firing.
    pub turns_from_root: u32,
    /// Live windows holding four or more of the MOVER's stones.
    pub mover_hot: u32,
    /// The same for the opponent.
    pub opponent_hot: u32,
    /// Live windows holding exactly five of the MOVER's stones.
    pub mover_win_in_one_ply: u32,
    /// The same for the opponent.
    pub opponent_win_in_one_ply: u32,
    /// Live windows holding exactly three of the MOVER's stones.
    pub mover_live_three: u32,
    /// The same for the opponent.
    pub opponent_live_three: u32,
    /// Whether the mover can answer the opponent's hot windows this turn.
    pub cover: CoverClass,
}

impl TriggerColumns {
    /// Every column of one firing, from ONE state and ONE threat view.
    ///
    /// **ONE CONSTRUCTOR, BECAUSE THERE WERE TWO** (audit row A-05): the root's
    /// site and the in-tree site read the same four queries into the same eight
    /// fields in the same order, and a column added to one and not the other
    /// split the census with every gate green. The root's own site exists for a
    /// borrow reason and not a semantic one, so what it varies is
    /// `turns_from_root` and nothing else.
    ///
    /// # Panics
    ///
    /// [`crate::staged::OVERLOAD_ON_A_DECIDED_POSITION`] if the trigger fired
    /// on a decided position, which `cover` cannot be asked about. `site` is
    /// the only thing that separates the two firings in that message.
    pub(crate) fn at(
        state: &pistol_core::GameState,
        threats: &pistol_solver::ThreatState,
        turns_from_root: u32,
        site: &'static str,
    ) -> TriggerColumns {
        let mover = state.to_move();
        let opponent = mover.opponent();
        let counts = |side| {
            (
                threats.hot_windows(side).len() as u32,
                threats.win_in_one_ply_windows(side).len() as u32,
                threats
                    .live_windows_at_count(side, pistol_solver::LiveCount::Three)
                    .len() as u32,
            )
        };
        let (mover_hot, mover_win_in_one_ply, mover_live_three) = counts(mover);
        let (opponent_hot, opponent_win_in_one_ply, opponent_live_three) = counts(opponent);
        // The one column that costs more than a slice length. It is paid ONLY
        // under a census — a run that collects none never reaches this
        // function — so the cost the matrix's row (b) owes a bench is neither
        // paid nor measured here.
        // `site` EARNS ITS PARAMETER HERE AND NOWHERE ELSE. Merging the two
        // constructions took the in-tree wording, so the root's panic lost the
        // word "root" and a crash report could no longer tell the two firings
        // apart from the message. One constructor, still two named sites.
        let left = pistol_solver::StonesLeft::from_state(state).unwrap_or_else(|| {
            panic!(
                "pistol-search invariant {}: the {site} trigger fired on a decided position",
                crate::staged::OVERLOAD_ON_A_DECIDED_POSITION
            )
        });
        let cover = match threats.blocking_covers(mover, pistol_solver::HitBudget::from(left)) {
            pistol_solver::Cover::NothingToBlock => CoverClass::NothingToBlock,
            pistol_solver::Cover::Impossible => CoverClass::Impossible,
            pistol_solver::Cover::Minimal(covers) => CoverClass::Minimal(covers.len()),
        };
        TriggerColumns {
            turns_from_root,
            mover_hot,
            opponent_hot,
            mover_win_in_one_ply,
            opponent_win_in_one_ply,
            mover_live_three,
            opponent_live_three,
            cover,
        }
    }
}

/// Push one firing's row, if a census was asked for.
///
/// **ONE PUSH, BECAUSE THERE WERE TWO** (audit row A-05): the root's and the
/// in-tree one built the same `TriggerObservation` field for field behind the
/// same "no site, or no census, so nothing to record" guard. A field added to
/// one and not the other split the census as silently as a column did.
pub(crate) fn push(
    census: &mut Option<Vec<TriggerObservation>>,
    site: Option<(CensusKeys, TriggerColumns)>,
    attacker: TriggerAnswer,
    defender: Option<TriggerAnswer>,
) {
    let (Some((keys, columns)), Some(rows)) = (site, census.as_mut()) else {
        return;
    };
    rows.push(TriggerObservation {
        key: keys.key,
        key_pos: keys.key_pos,
        columns,
        attacker,
        defender,
    });
}

/// What the mover can do about the opponent's hot windows this turn.
///
/// [`pistol_solver::Cover`] carries the covering CELLS; a census row wants the
/// class and the count, because a detector reading this column decides on
/// whether a cover exists and not on where it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverClass {
    /// The opponent holds no hot window. Reachable here because a firing needs
    /// only ONE side hot, and the mover-hot-only firings are exactly this.
    NothingToBlock,
    /// No cover within the turn's stones exists: the threat is unanswerable,
    /// which is row (b)'s whole mechanism.
    Impossible,
    /// A cover exists, with this many inclusion-minimal ones.
    Minimal(usize),
}

impl CoverClass {
    /// The token the census prints, so the reader and the writer share one
    /// spelling rather than two that drift.
    pub fn token(self) -> &'static str {
        match self {
            CoverClass::NothingToBlock => "none",
            CoverClass::Impossible => "impossible",
            CoverClass::Minimal(_) => "minimal",
        }
    }

    /// How many inclusion-minimal covers there are; zero for the two classes
    /// that hold none.
    pub fn count(self) -> usize {
        match self {
            CoverClass::NothingToBlock | CoverClass::Impossible => 0,
            CoverClass::Minimal(count) => count,
        }
    }
}
