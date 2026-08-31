use pistol_core::Turn;

/// Which seat won, or that nobody did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    /// The first player won.
    P1Win,
    /// The second player won.
    P2Win,
    /// The turn cap ended it. An evaluation horizon, never a game rule.
    Capped,
}

impl GameResult {
    /// The report's word.
    pub const fn token(self) -> &'static str {
        match self {
            GameResult::P1Win => "p1_win",
            GameResult::P2Win => "p2_win",
            GameResult::Capped => "capped",
        }
    }

    /// The result a report's word names, or `None` for a word this build does
    /// not write.
    ///
    /// The inverse of [`GameResult::token`], so a reader cannot come to accept a
    /// spelling the writer never produces.
    pub fn from_token(word: &str) -> Option<GameResult> {
        match word {
            "p1_win" => Some(GameResult::P1Win),
            "p2_win" => Some(GameResult::P2Win),
            "capped" => Some(GameResult::Capped),
            _ => None,
        }
    }
}

/// How the game ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// By the rules, or at the cap.
    Normal,
    /// One side stopped playing legally, and lost for it.
    Forfeit(ForfeitReason),
}

/// Why a side forfeited. A closed set: these are report tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForfeitReason {
    /// The turn it answered with is not legal in the position it was given.
    IllegalTurn,
    /// It refused a line, answered something that is not the protocol, or
    /// answered nothing recognizable.
    ///
    /// A refusal to `position` and a refusal to `go` are one token on purpose:
    /// both arrive as an `error` line in the same stream and the protocol does
    /// not label which verb a `Protocol` error came from, so telling them apart
    /// would be a guess. The verbatim line is recorded instead.
    ProtocolError,
    /// It exited, with a code, before answering.
    EngineExited,
    /// Its `bestmove` line does not carry a turn token.
    BadBestmove,
}

impl ForfeitReason {
    /// The report's word.
    pub const fn token(self) -> &'static str {
        match self {
            ForfeitReason::IllegalTurn => "illegal_turn",
            ForfeitReason::ProtocolError => "protocol_error",
            ForfeitReason::EngineExited => "engine_exited",
            ForfeitReason::BadBestmove => "bad_bestmove",
        }
    }
}

/// What one engine spent on one game.
///
/// `nodes` and `max_depth` are reproducible and belong to the verdict block;
/// `time_ms` measures the machine and belongs to the timing block
/// (docs/decisions.md D-7, D-161).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Compute {
    /// Nodes summed over the game's searches.
    pub nodes: u64,
    /// Wall-clock milliseconds summed over the game's searches.
    pub time_ms: u64,
    /// The deepest completed depth, in turns, any of them reached.
    pub max_depth: u32,
    /// How many searches were asked for.
    pub searches: u32,
}

impl Compute {
    /// Fold one `info totals` line's numbers in.
    pub fn add(&mut self, nodes: u64, time_ms: u64, depth_turns: u32) {
        self.nodes = self.nodes.saturating_add(nodes);
        self.time_ms = self.time_ms.saturating_add(time_ms);
        self.max_depth = self.max_depth.max(depth_turns);
        self.searches += 1;
    }

    /// Fold another game's totals in, for a per-run line.
    pub fn absorb(&mut self, other: Compute) {
        self.nodes = self.nodes.saturating_add(other.nodes);
        self.time_ms = self.time_ms.saturating_add(other.time_ms);
        self.max_depth = self.max_depth.max(other.max_depth);
        self.searches += other.searches;
    }
}

/// One finished game.
#[derive(Debug, Clone, PartialEq)]
pub struct GameRecord {
    /// Its place in the run, which is also its place in the report.
    pub index: usize,
    /// Which opening it started from.
    pub opening: usize,
    /// Whether engine A held the first seat. The other engine held the second.
    pub a_is_p1: bool,
    /// Who won, or that nobody did.
    pub result: GameResult,
    /// How it ended.
    pub end: End,
    /// Which engine forfeited, as an index into the two engines: `0` is A.
    pub forfeit_by: Option<usize>,
    /// The line the offender wrote, verbatim, when it wrote one.
    pub refusal: Option<String>,
    /// The whole game, opening included — the canonical encoding of what was
    /// played, and what the distinct-game key is computed from.
    pub moves: Vec<Turn>,
    /// Per engine, indexed the same way as `forfeit_by`.
    pub compute: [Compute; 2],
}

impl GameRecord {
    /// How many turns were played, opening included.
    pub fn turns(&self) -> usize {
        self.moves.len()
    }

    /// Engine A's score for this game: 1 a win, 0.5 capped, 0 a loss.
    pub fn score_a(&self) -> f64 {
        match self.result {
            GameResult::Capped => 0.5,
            GameResult::P1Win => {
                if self.a_is_p1 {
                    1.0
                } else {
                    0.0
                }
            }
            GameResult::P2Win => {
                if self.a_is_p1 {
                    0.0
                } else {
                    1.0
                }
            }
        }
    }

    /// Whether a forfeit ended this game.
    pub fn is_forfeit(&self) -> bool {
        matches!(self.end, End::Forfeit(_))
    }

    /// Whether the game was decided by the rules rather than by the horizon.
    ///
    /// A capped game has no first-player winner, which is why the run's
    /// first-player rate is over these and not over every game.
    pub fn is_decided(&self) -> bool {
        !matches!(self.result, GameResult::Capped)
    }
}
