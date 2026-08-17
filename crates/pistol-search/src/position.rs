//! The position the search walks: a game and an evaluation, kept in step.
//!
//! `GameState` moves the stone and `Eval` accounts for it, and the two are only
//! correct together — an eval updated for a stone the state refused, or a stone
//! placed without telling the eval, is the drift docs/decisions.md D-41 names
//! the seam for. So they are moved from one place, here, and the search never
//! holds either of them alone.
//!
//! # Failure
//!
//! [`Position::place`] returns the rules' own refusal, because the search asks
//! about cells it believes are legal and would rather hear that it was wrong.
//! [`Position::undo`] cannot fail from a search: the search takes back what it
//! just placed, so a refusal there is a broken invariant and panics with
//! [`POSITION_DESYNC`].

use pistol_core::{Board, Color, Coord, CoreError, GameState, PlyOutcome};
use pistol_eval::Eval;

/// Named invariant: the game and the evaluation disagree about what is on the
/// board.
pub const POSITION_DESYNC: &str = "POSITION_DESYNC";

/// A game in progress and the evaluation of it.
pub struct Position {
    state: GameState,
    eval: Box<dyn Eval>,
    /// The stones this search put down, newest last.
    ///
    /// Not a second copy of the game: it is what the *evaluation* has to be told
    /// to take back, and the cell alone is not enough — the colour has left the
    /// board by the time the game hands the cell back. Reading it off the game
    /// instead would mean walking its history on every take-back, which is a
    /// linear cost per ply in a loop that runs millions of times.
    placed: Vec<(Coord, Color)>,
}

impl Position {
    /// A new game and an empty evaluation of it.
    pub fn new(eval: Box<dyn Eval>) -> Position {
        Position {
            state: GameState::new_game(),
            eval,
            placed: Vec::new(),
        }
    }

    /// Take up a position: unwind the evaluation to empty, then rebuild it over
    /// the new stones.
    ///
    /// Unwinding rather than rebuilding from nothing is what the trait promises
    /// — an unwound eval is indistinguishable from a fresh one, whatever order
    /// the stones came off in (docs/decisions.md D-61, D-62) — and it keeps this
    /// from needing a way to construct a backend it only knows as `dyn Eval`.
    pub fn reset_to(&mut self, state: &GameState) {
        let stones: Vec<(Coord, Color)> = self.state.board().stones().collect();
        for (at, color) in stones {
            self.eval.undo(at, color);
        }
        self.state = state.clone();
        for (at, color) in self.state.board().stones() {
            self.eval.apply(at, color);
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
    /// The stone comes straight back off, so the position is unchanged.
    pub fn static_score_after(&mut self, at: Coord) -> i32 {
        let mover = self.state.to_move();
        self.eval.apply(at, mover);
        let score = self.eval.value(mover);
        self.eval.undo(at, mover);
        score
    }

    /// Place the next stone of the turn, accounting for it in the evaluation.
    ///
    /// A refused stone leaves the evaluation untouched, so a caller that handles
    /// the refusal is standing on the position it started from.
    pub fn place(&mut self, at: Coord) -> Result<PlyOutcome, CoreError> {
        let mover = self.state.to_move();
        let outcome = self.state.place(at)?;
        self.eval.apply(at, mover);
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
        let (at, color) = self.placed.pop().unwrap_or_else(|| {
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
        self.eval.undo(at, color);
    }
}
