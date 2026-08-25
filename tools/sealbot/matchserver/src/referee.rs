//! The referee: one game, driven over pistol-core, judged by pistol-core.
//!
//! No rule lives here (CLAUDE.md rule 2). What lives here is the platform's
//! JUDGING POLICY, spelled as the official server behaves (tools/sealbot/
//! README.md, "The platform's game rules"): the server plays the opening
//! stone itself, asks each engine at every turn boundary, applies submitted
//! stones strictly in submitted order, ends the game the instant a stone
//! completes a line, forfeits the mover on any refusal or incomplete turn,
//! and caps the game at the configured horizon.

use pistol_core::{Coord, GameState, Phase, Player, PlyOutcome};

use crate::client::EngineClient;

/// The opening the platform's server auto-plays: one cross at the origin,
/// p1's turn-1 stone (the htttx `setup` packet's standard board).
const OPENING: Coord = Coord::new(0, 0);

/// How one turn went, for the transcript and the report.
#[derive(Debug, Clone, PartialEq)]
pub enum TurnOutcome {
    /// The turn was applied; the game continues.
    Continue,
    /// A stone completed a line; the game is over.
    Win {
        winner: Player,
        turn: u32,
        /// True when the completing stone was the first of a two-stone turn:
        /// rule 4's truncation, the second stone never applied.
        first_stone_win: bool,
    },
    /// `place` refused a stone: illegal move, forfeit.
    Illegal { stone: Coord, why: String },
    /// The submitted turn stopped short of the stones owed: forfeit.
    Incomplete { submitted: usize, owed: u32 },
    /// The engine did not answer: forfeit.
    EngineFailure { detail: String },
}

/// How a game ended.
#[derive(Debug, Clone)]
pub enum GameResult {
    /// Six (or more) in a line.
    Win {
        winner: Player,
        turn: u32,
        first_stone_win: bool,
    },
    /// The evaluation horizon: no decision within the cap.
    Capped { turn: u32 },
    /// One side lost by forfeit (illegal move, incomplete turn, or engine
    /// failure) — counted separately, never as a win by line.
    Forfeit { loser: Player, why: String },
}

/// One turn as the transcript records it.
#[derive(Debug, Clone)]
pub struct TurnRecord {
    pub turn: u32,
    pub mover: Player,
    pub engine: String,
    pub stones: Vec<Coord>,
    pub nodes: Option<u64>,
    pub engine_time_ms: Option<u64>,
    pub wall_ms: u64,
    pub raw: String,
    pub outcome: TurnOutcome,
}

/// One game as the report records it.
pub struct GameSummary {
    pub game: u32,
    /// Whether engine A played p1 this game (seats alternate per game).
    pub a_is_p1: bool,
    pub result: GameResult,
    pub turns: Vec<TurnRecord>,
    /// Engine A's summed node count, when it reports one.
    pub a_nodes: Option<u64>,
    /// Engine A's summed wall time, ms.
    pub a_wall_ms: u64,
    /// Engine B's summed wall time, ms.
    pub b_wall_ms: u64,
}

impl GameSummary {
    /// A short result token for JSON.
    pub fn kind(&self) -> &'static str {
        match &self.result {
            GameResult::Win { .. } => "win",
            GameResult::Capped { .. } => "capped",
            GameResult::Forfeit { .. } => "forfeit",
        }
    }
}

/// Run one game and judge it. Engines are handed in already told nothing;
/// they are told `new_game` here, and finished here.
pub fn run_game(
    game_no: u32,
    a_is_p1: bool,
    turn_cap: u32,
    a: &mut dyn EngineClient,
    b: &mut dyn EngineClient,
) -> GameSummary {
    let mut state = GameState::new_game();
    // The server's opening: p1's turn-1 stone at the origin. The only stone
    // the referee ever plays itself, exactly as the platform's server does.
    match state.place(OPENING) {
        Ok(PlyOutcome::TurnComplete) => {}
        other => panic!("matchserver invariant: the opening stone did not complete turn 1: {other:?}"),
    }
    let mut plies: Vec<(Coord, Player)> = vec![(OPENING, Player::P1)];

    let _ = a.new_game(game_no);
    let _ = b.new_game(game_no);

    let mut turns: Vec<TurnRecord> = Vec::new();
    let mut a_nodes: Option<u64> = Some(0);
    let mut a_wall_ms = 0u64;
    let mut b_wall_ms = 0u64;

    let result = loop {
        if state.turn() > turn_cap {
            break GameResult::Capped { turn: turn_cap };
        }
        let mover = state.to_move();
        let owed = state.stones_owed();
        let mover_is_a = (mover == Player::P1) == a_is_p1;
        let (label, reply, is_a) = if mover_is_a {
            let label = a.label().to_string();
            (label, a.pick_turn(&plies, owed), true)
        } else {
            let label = b.label().to_string();
            (label, b.pick_turn(&plies, owed), false)
        };

        let reply = match reply {
            Ok(reply) => reply,
            Err(failure) => {
                turns.push(TurnRecord {
                    turn: state.turn(),
                    mover,
                    engine: label,
                    stones: Vec::new(),
                    nodes: None,
                    engine_time_ms: None,
                    wall_ms: 0,
                    raw: String::new(),
                    outcome: TurnOutcome::EngineFailure {
                        detail: failure.describe(),
                    },
                });
                break GameResult::Forfeit {
                    loser: mover,
                    why: failure.describe(),
                };
            }
        };
        if is_a {
            a_wall_ms += reply.wall_ms;
            a_nodes = match (a_nodes, reply.nodes) {
                (Some(total), Some(delta)) => Some(total + delta),
                _ => None,
            };
        } else {
            b_wall_ms += reply.wall_ms;
        }

        let mut outcome = apply_turn(&mut state, &mut plies, mover, owed, &reply.stones);
        if outcome == TurnOutcome::Continue && state.phase() != Phase::First {
            outcome = TurnOutcome::Incomplete {
                submitted: reply.stones.len(),
                owed,
            };
        }
        let finished: Option<GameResult> = match &outcome {
            TurnOutcome::Continue => None,
            TurnOutcome::Win {
                winner,
                turn,
                first_stone_win,
            } => Some(GameResult::Win {
                winner: *winner,
                turn: *turn,
                first_stone_win: *first_stone_win,
            }),
            TurnOutcome::Illegal { stone, why } => Some(GameResult::Forfeit {
                loser: mover,
                why: format!("illegal move {stone}: {why}"),
            }),
            TurnOutcome::Incomplete { submitted, owed } => Some(GameResult::Forfeit {
                loser: mover,
                why: format!("incomplete turn: {submitted} of {owed} stones, no win"),
            }),
            TurnOutcome::EngineFailure { .. } => {
                unreachable!("an engine failure is handled before its stones are judged")
            }
        };
        turns.push(TurnRecord {
            turn: record_turn_of(&state, &outcome),
            mover,
            engine: label,
            stones: reply.stones.clone(),
            nodes: reply.nodes,
            engine_time_ms: reply.engine_time_ms,
            wall_ms: reply.wall_ms,
            raw: reply.raw,
            outcome: outcome.clone(),
        });
        if let Some(result) = finished {
            break result;
        }
    };

    a.finish_game();
    b.finish_game();

    GameSummary {
        game: game_no,
        a_is_p1,
        result,
        turns,
        a_nodes,
        a_wall_ms,
        b_wall_ms,
    }
}

/// Apply submitted stones in submitted order; pistol-core judges each one.
fn apply_turn(
    state: &mut GameState,
    plies: &mut Vec<(Coord, Player)>,
    mover: Player,
    owed: u32,
    stones: &[Coord],
) -> TurnOutcome {
    let mut outcome = TurnOutcome::Continue;
    for (index, stone) in stones.iter().enumerate() {
        if index >= 2 || (owed == 1 && index >= 1) {
            // More stones than a turn is shaped for: the platform's move is
            // one or two pieces; a third is a malformed move.
            outcome = TurnOutcome::Illegal {
                stone: *stone,
                why: format!("a turn owes {owed} stones, more were submitted"),
            };
            break;
        }
        match state.place(*stone) {
            Err(error) => {
                outcome = TurnOutcome::Illegal {
                    stone: *stone,
                    why: error.to_string(),
                };
                break;
            }
            Ok(PlyOutcome::Win { winner, turn }) => {
                outcome = TurnOutcome::Win {
                    winner,
                    turn,
                    first_stone_win: owed == 2 && index == 0,
                };
                break;
            }
            Ok(_) => plies.push((*stone, mover)),
        }
    }
    outcome
}

/// The turn a record belongs to. A win carries its own turn number (frozen
/// at the deciding stone); otherwise a COMPLETED turn is the one before the
/// state advanced, and a FAILED attempt is the in-progress one, which is the
/// state's own turn whenever stones are still owed.
fn record_turn_of(state: &GameState, outcome: &TurnOutcome) -> u32 {
    if let TurnOutcome::Win { turn, .. } = outcome {
        return *turn;
    }
    match outcome {
        TurnOutcome::Continue => state.turn().saturating_sub(1),
        _ => state.turn(),
    }
}
