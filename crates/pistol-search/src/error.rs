use std::fmt;

use pistol_core::Player;

use crate::params::CandidatePolicy;

/// Every way the search refuses to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    /// A search parameter is outside the range this build can honour. `key` is
    /// the dotted config path that names it.
    Params {
        /// Dotted config path of the offending parameter.
        key: &'static str,
        /// What is wrong with it.
        why: String,
    },
    /// The root position is in the middle of a turn. The search counts in turns
    /// and reports a principal variation of turns, so it starts at a turn
    /// boundary (docs/decisions.md D-50, D-71).
    TurnInProgress {
        /// The turn that is half played.
        turn: u32,
    },
    /// The root position is already decided. There is no move to search for.
    GameDecided {
        /// The side that completed a line.
        winner: Player,
        /// The turn it completed on.
        turn: u32,
    },
    /// A depth was asked for that this build's ply horizon cannot hold.
    DepthOutOfRange {
        /// The depth asked for, in turns.
        turns: u32,
        /// The deepest this build searches, in turns.
        max: u32,
    },
    /// The candidate policy offered no cell at the root, though the position is
    /// ongoing and the rules therefore admit a move. Narrowing the search until
    /// there is nothing left to play is a configuration error, and the engine
    /// says so rather than picking a move the policy excluded.
    ///
    /// No radius of at least one can provoke this, by the argument
    /// `pistol_core::turn` gives for there being no stalemate: the cell one step
    /// past the stone with the largest `q` is empty, is within one of that
    /// stone, and is therefore in every policy's reach. It is stated anyway
    /// because "cannot happen" is a claim about today's policies, and the next
    /// one to arrive would otherwise fail by returning no move.
    ///
    /// **Policy-agnostic by construction** (U2-Z item 8, `U3_tier_t.md` §U3-T's
    /// `no_candidates_under_staged_is_refused_by_a_policy_agnostic_error`): a
    /// `CandidatePolicy::Staged` has three radius-shaped numbers of its own
    /// (`quiet_radius` and the two `tier_t_*_count` thresholds), and a variant
    /// carrying a bare `radius: u32` field would either lie about which of
    /// them came up empty or force an invented one. The whole policy is
    /// carried instead, and [`fmt::Display`] reads it.
    NoCandidates {
        /// The turn the mover owes a stone on.
        turn: u32,
        /// The candidate policy that came up empty.
        policy: CandidatePolicy,
    },
}

impl SearchError {
    /// Build a [`SearchError::Params`] from anything string-shaped.
    pub fn params(key: &'static str, why: impl Into<String>) -> Self {
        SearchError::Params {
            key,
            why: why.into(),
        }
    }
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::Params { key, why } => write!(f, "search parameter `{key}`: {why}"),
            SearchError::TurnInProgress { turn } => {
                write!(
                    f,
                    "turn {turn} is half played: the search starts at a turn boundary"
                )
            }
            SearchError::GameDecided { winner, turn } => {
                write!(
                    f,
                    "{winner} won on turn {turn}: there is no move to search for"
                )
            }
            SearchError::DepthOutOfRange { turns, max } => {
                write!(
                    f,
                    "depth {turns} turns is past this build's horizon of {max}"
                )
            }
            SearchError::NoCandidates { turn, policy } => write!(
                f,
                "the {policy:?} candidate policy offers no cell on turn {turn}, \
                 though the rules admit a move"
            ),
        }
    }
}

impl std::error::Error for SearchError {}
