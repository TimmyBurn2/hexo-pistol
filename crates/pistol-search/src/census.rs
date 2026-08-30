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
    /// Turns from the search root. 0 is the root's own firing.
    pub turns_from_root: u32,
    /// Live windows holding four or more of the MOVER's stones.
    pub mover_hot: u32,
    /// The same for the opponent.
    pub opponent_hot: u32,
    /// Live windows holding exactly five of the MOVER's stones — one stone
    /// from six.
    pub mover_win_in_one_ply: u32,
    /// The same for the opponent.
    pub opponent_win_in_one_ply: u32,
    /// Live windows holding exactly three of the MOVER's stones.
    pub mover_live_three: u32,
    /// The same for the opponent.
    pub opponent_live_three: u32,
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
