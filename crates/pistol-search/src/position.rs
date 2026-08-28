use pistol_core::{Board, Coord, CoreError, GameState, Player, PlyOutcome};
use pistol_eval::Eval;
use pistol_solver::ThreatState;

/// Named invariant: the game and the evaluation disagree about what is on the
/// board.
pub const POSITION_DESYNC: &str = "POSITION_DESYNC";

/// A game in progress, the evaluation of it, and — under `Staged` — the threat
/// state.
pub struct Position {
    state: GameState,
    eval: Box<dyn Eval>,
    /// `Some` exactly when this position was built to track it
    /// (`CandidatePolicy::Staged`); `None` under `Radius`, where nothing reads
    /// it.
    threats: Option<ThreatState>,
    /// The stones this search put down, newest last.
    ///
    /// Not a second copy of the game: it is what the *evaluation* has to be told
    /// to take back, and the cell alone is not enough — the player has left the
    /// board by the time the game hands the cell back. Reading it off the game
    /// instead would mean walking its history on every take-back, which is a
    /// linear cost per ply in a loop that runs millions of times.
    placed: Vec<(Coord, Player)>,
}

impl Position {
    /// A new game, an empty evaluation of it, and — if `tracks_threats` — an
    /// empty threat state.
    ///
    /// `tracks_threats` is the caller's `CandidatePolicy::Staged` test, made
    /// once at construction rather than read from a policy this type does not
    /// hold: [`crate::search::Searcher::new`] is the one caller and it already
    /// knows its own policy.
    pub fn new(eval: Box<dyn Eval>, tracks_threats: bool) -> Position {
        Position {
            state: GameState::new_game(),
            eval,
            threats: tracks_threats.then(ThreatState::new),
            placed: Vec::new(),
        }
    }

    /// Take up a position: unwind the evaluation (and the threat state, if
    /// tracked) to empty, then rebuild both over the new stones.
    ///
    /// Unwinding rather than rebuilding from nothing is what the trait promises
    /// — an unwound eval is indistinguishable from a fresh one, whatever order
    /// the stones came off in (docs/decisions.md D-61, D-62) — and it keeps this
    /// from needing a way to construct a backend it only knows as `dyn Eval`.
    /// The threat state is simply replaced: `ThreatState::new` is O(1) and its
    /// own `apply` is what rebuilds it, in the same loop as the eval — O(stones
    /// × 18) once per search, per `U2_node_protocol.md` §2.1.
    pub fn reset_to(&mut self, state: &GameState) {
        let stones: Vec<(Coord, Player)> = self.state.board().stones().collect();
        for (at, player) in stones {
            self.eval.undo(at, player);
        }
        if self.threats.is_some() {
            self.threats = Some(ThreatState::new());
        }
        self.state = state.clone();
        for (at, player) in self.state.board().stones() {
            self.eval.apply(at, player);
            if let Some(threats) = &mut self.threats {
                threats.apply(at, player);
            }
        }
        self.placed.clear();
    }

    /// The game.
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// The stones.
    pub fn board(&self) -> &Board {
        self.state.board()
    }

    /// What the position is worth to the side to move.
    pub fn value(&self) -> i32 {
        self.eval.value(self.state.to_move())
    }

    /// What the position would be worth to the mover with a stone of theirs on
    /// `at` — the static score move ordering sorts by.
    ///
    /// This touches the evaluation and not the game: ordering asks about cells
    /// the candidate policy has already established are legal, and it does not
    /// care whether one of them wins, only what the pattern tables make of it.
    /// The stone is HYPOTHETICAL and is never applied: [`pistol_eval::Eval::delta`]
    /// answers what the D-76 apply/value/undo roundtrip used to answer here,
    /// leaving the eval untouched — the mechanism change D-192's profile
    /// licensed, with the numbers pinned identical by the delta oracle test
    /// and the search-identity gate (docs/decisions.md D-110, D-214).
    pub fn static_score_after(&mut self, at: Coord) -> i32 {
        let mover = self.state.to_move();
        self.eval.delta(at, mover)
    }

    /// Place the next stone of the turn, accounting for it in the evaluation.
    ///
    /// A refused stone leaves the evaluation untouched, so a caller that handles
    /// the refusal is standing on the position it started from.
    pub fn place(&mut self, at: Coord) -> Result<PlyOutcome, CoreError> {
        let mover = self.state.to_move();
        let outcome = self.state.place(at)?;
        self.eval.apply(at, mover);
        if let Some(threats) = &mut self.threats {
            threats.apply(at, mover);
        }
        self.placed.push((at, mover));
        Ok(outcome)
    }

    /// Take back the last stone this search placed.
    ///
    /// # Panics
    ///
    /// With [`POSITION_DESYNC`] if there is nothing this search placed to take
    /// back, or if the game hands back a different stone than the one recorded.
    pub fn undo(&mut self) {
        let (at, player) = self.placed.pop().unwrap_or_else(|| {
            panic!(
                "pistol-search invariant {POSITION_DESYNC}: nothing to take back, though the \
                 search only ever takes back what it placed"
            )
        });
        let taken = self.state.undo().unwrap_or_else(|error| {
            panic!("pistol-search invariant {POSITION_DESYNC}: taking back {at}: {error}")
        });
        assert!(
            taken == at,
            "pistol-search invariant {POSITION_DESYNC}: this search placed {at} last and the \
             game took back {taken}"
        );
        self.eval.undo(at, player);
        if let Some(threats) = &mut self.threats {
            threats.undo(at, player);
        }
    }

    /// The three things [`crate::staged::staged_candidates`] needs, handed out
    /// together (`U2_node_protocol.md` §5.35: "one accessor, so the three
    /// cannot be taken apart at a call site and drift").
    ///
    /// # Panics
    ///
    /// If this position was not built to track threats (`tracks_threats` was
    /// `false` at [`Position::new`]) — a caller reaching for the staged
    /// generator under `CandidatePolicy::Radius` is a policy-dispatch bug, not
    /// operator input.
    pub(crate) fn staged_context(&mut self) -> (&GameState, &ThreatState, &mut dyn Eval) {
        let threats = self.threats.as_ref().unwrap_or_else(|| {
            panic!(
                "pistol-search invariant {POSITION_DESYNC}: staged_context called on a position \
                 that was not built to track threats"
            )
        });
        (&self.state, threats, self.eval.as_mut())
    }
}
