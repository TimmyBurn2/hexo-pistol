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
