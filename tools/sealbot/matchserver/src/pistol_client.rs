//! The pistol seat: drive the shipped binary over its own line protocol.
//!
//! THE MODE IS PINNED TO THE BUDGET, in both directions: the handshake's
//! `id mode` line is CHECKED before the first move request, and which mode it
//! must say is decided by the budget the seat carries. A node budget seats
//! `instrument`, because the determinism law (CLAUDE.md rule 4) is what makes
//! that anchor reproducible; a `movetime` budget seats `play`, because
//! instrument mode refuses a wall-clock budget by name (docs/decisions.md
//! D-22). A seat that accepted either mode would let a config silently measure
//! the other engine.
//!
//! # Why the pair order is recovered by replay
//!
//! `bestmove` spells a pair canonically (smaller cell first), but which stone
//! goes down first is a legality question pistol-core answers when the turn
//! is made: a pair is legal iff SOME ordering is (D-6), and D-52 constructs
//! pairs legal in only one order. The referee applies submitted stones
//! strictly in submitted order (platform semantics), so this client replays
//! the game on a shadow `GameState`, calls `make_turn` there, and submits the
//! stones in the order pistol-core itself plays them.

use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use pistol_core::{Coord, GameState, Player, Turn};

use crate::budget::PistolBudget;
use crate::client::{EngineClient, EngineFailure, EngineReply, LineProcess};
use crate::deadline;

/// The reply that ends the pistol handshake.
const HANDSHAKE_OK: &str = "pistolok";
/// The word every identity line starts with.
const ID_PREFIX: &str = "id ";
/// The word every refusal starts with.
const ERROR_PREFIX: &str = "error";
/// The word the answer starts with.
const BESTMOVE_PREFIX: &str = "bestmove ";
/// The marker that distinguishes the closing report from a per-depth one.
const TOTALS_MARKER: &str = " totals ";
/// One configured pistol seat.
pub struct PistolClient {
    label: String,
    command: Vec<String>,
    cwd: std::path::PathBuf,
    budget: PistolBudget,
    timeout_seconds: f64,
    out_dir: std::path::PathBuf,
    prefix: String,
    process: Option<LineProcess>,
    /// The game the current process belongs to, for stderr paths.
    game_no: u32,
}

impl PistolClient {
    /// Build the seat from the validated config, the run's output directory
    /// and this seat's file prefix.
    pub fn new(
        label: String,
        command: Vec<String>,
        cwd: &str,
        budget: PistolBudget,
        timeout_seconds: f64,
        out_dir: &Path,
        prefix: &str,
    ) -> PistolClient {
        PistolClient {
            label,
            command,
            cwd: Path::new(cwd).to_path_buf(),
            budget,
            timeout_seconds,
            out_dir: out_dir.to_path_buf(),
            prefix: prefix.to_string(),
            process: None,
            game_no: 0,
        }
    }

    /// The process, spawned for this game if it is not up.
    fn ensure_spawned(&mut self) -> Result<(), EngineFailure> {
        if self.process.is_none() {
            let stderr_path = self
                .out_dir
                .join(format!("g{:03}_{}.stderr", self.game_no, self.prefix));
            let process = LineProcess::spawn(&self.command, &self.cwd, &stderr_path)?;
            self.process = Some(process);
            self.handshake()?;
        }
        Ok(())
    }

    /// Read the handshake and pin the mode.
    fn handshake(&mut self) -> Result<(), EngineFailure> {
        let process = self.process.as_mut().expect("spawned");
        let deadline = deadline(self.timeout_seconds);
        process.send("pistol")?;
        let required_mode = self.budget.required_mode();
        let required_budget = self.budget.required_budget_word();
        let mut saw_mode = false;
        let mut saw_budget = false;
        loop {
            let line = process.read_line(deadline)?;
            if line == HANDSHAKE_OK {
                break;
            }
            if line.starts_with(ERROR_PREFIX) {
                return Err(EngineFailure::Protocol {
                    why: format!("handshake refused: {line}"),
                });
            }
            if let Some(rest) = line.strip_prefix(ID_PREFIX) {
                if let Some(mode) = rest.strip_prefix("mode ") {
                    if mode != required_mode {
                        return Err(EngineFailure::Protocol {
                            why: format!(
                                "engine mode is {mode}, not {required_mode}: this seat's \
                                 budget seats {required_mode} mode"
                            ),
                        });
                    }
                    saw_mode = true;
                }
                if let Some(budgets) = rest.strip_prefix("budgets ") {
                    if !budgets
                        .split_whitespace()
                        .any(|word| word == required_budget)
                    {
                        return Err(EngineFailure::Protocol {
                            why: format!(
                                "engine budgets do not include {required_budget}: {budgets}"
                            ),
                        });
                    }
                    saw_budget = true;
                }
            }
        }
        if !saw_mode || !saw_budget {
            return Err(EngineFailure::Protocol {
                why: "handshake lacked the mode or budgets identity line".to_string(),
            });
        }
        Ok(())
    }

    /// `position start moves <turns>` from the ply list, canonically spelled.
    fn position_line(plies: &[(Coord, Player)]) -> Result<String, String> {
        if plies.is_empty() {
            return Ok("position start".to_string());
        }
        // Turn 1 is one stone and every later turn two (rule 3), so a ply
        // list at a turn boundary is 1 + 2k long. An even-length list is not
        // a position any game is ever AT — refused by name rather than
        // silently truncating the dangling half-turn (CLAUDE.md rule 3).
        if plies.len().is_multiple_of(2) {
            return Err(format!(
                "the ply stream has {} stones: turn 1 is one stone and every later turn \
                 two, so a turn boundary is an odd count",
                plies.len()
            ));
        }
        let coords: Vec<Coord> = plies.iter().map(|(at, _)| *at).collect();
        let mut line = String::from("position start moves");
        line.push(' ');
        line.push_str(&coords[0].to_string());
        for pair in coords[1..].chunks_exact(2) {
            let turn =
                Turn::pair(pair[0], pair[1]).map_err(|error| error.to_string())?;
            line.push(' ');
            line.push_str(&turn.to_string());
        }
        Ok(line)
    }

    /// The stones of a bestmove turn, in pistol-core's own play order.
    fn play_order(plies: &[(Coord, Player)], turn: Turn) -> Result<Vec<Coord>, String> {
        let coords: Vec<Coord> = plies.iter().map(|(at, _)| *at).collect();
        let mut state = GameState::from_plies(&coords).map_err(|error| error.to_string())?;
        match state.make_turn(turn) {
            Ok(_) => {
                // undo takes stones back last-first, exactly as many as the
                // turn placed: one for a Single, two for a Pair.
                let placed = if turn.second().is_some() { 2 } else { 1 };
                let mut stones = Vec::with_capacity(placed);
                for _ in 0..placed {
                    stones.push(state.undo().map_err(|error| error.to_string())?);
                }
                stones.reverse();
                Ok(stones)
            }
            // The engine's own move was refused by its own rules; submit the
            // canonical spelling and let the referee judge it loudly.
            Err(_) => Ok(vec![turn.first(), turn.second().unwrap_or(turn.first())]),
        }
    }
}

impl EngineClient for PistolClient {
    fn label(&self) -> &str {
        &self.label
    }

    fn new_game(&mut self, game_no: u32) -> Result<(), EngineFailure> {
        // One process per game: crash containment and a clean table.
        if let Some(mut process) = self.process.take() {
            process.close_stdin();
            process.wait();
        }
        self.game_no = game_no;
        self.ensure_spawned()?;
        self.process
            .as_mut()
            .expect("just spawned")
            .send("newgame")
    }

    fn pick_turn(
        &mut self,
        plies: &[(Coord, Player)],
        _owed: u32,
    ) -> Result<EngineReply, EngineFailure> {
        let started = Instant::now();
        self.ensure_spawned()?;
        let process = self.process.as_mut().expect("just spawned");
        let deadline = deadline(self.timeout_seconds);
        let position = Self::position_line(plies).map_err(|why| EngineFailure::Protocol {
            why: format!("building the position line: {why}"),
        })?;
        process.send(&position)?;
        process.send(&self.budget.go_line())?;
        let mut nodes = None;
        let mut engine_time_ms = None;
        let token = loop {
            let line = process.read_line(deadline)?;
            if line.starts_with(ERROR_PREFIX) {
                return Err(EngineFailure::Protocol { why: line });
            }
            if let Some(rest) = line.strip_prefix("info") {
                if rest.contains(TOTALS_MARKER) {
                    nodes = field_after(rest, "nodes ");
                    engine_time_ms = field_after(rest, "time ");
                }
                continue;
            }
            if let Some(best) = line.strip_prefix(BESTMOVE_PREFIX) {
                break best.trim().to_string();
            }
        };
        let turn = Turn::from_str(&token).map_err(|error| EngineFailure::Protocol {
            why: format!("bestmove {token}: {error}"),
        })?;
        let stones = Self::play_order(plies, turn).map_err(|why| EngineFailure::Protocol {
            why: format!("recovering play order: {why}"),
        })?;
        Ok(EngineReply {
            stones,
            nodes,
            engine_time_ms,
            wall_ms: started.elapsed().as_millis() as u64,
            raw: format!("bestmove {token}"),
        })
    }

    fn finish_game(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.send("quit");
            process.close_stdin();
            process.wait();
        }
    }
}

/// The integer field that follows `key` in an info tail, if it is there.
fn field_after(rest: &str, key: &str) -> Option<u64> {
    word_bounded(rest, key)
        .and_then(|at| rest[at + key.len()..].split_whitespace().next())
        .and_then(|word| word.parse().ok())
}

/// The WORD-BOUNDARY find (WP-1.8b §3's fix): a plain `find` would match
/// `key` inside a longer field name — `nodes ` inside `solver_nodes 300`,
/// the exact hazard the widened engine's ON seats print — and read the
/// wrong counter as the node total. The match must start a field: at the
/// line's head or after a space.
fn word_bounded(rest: &str, key: &str) -> Option<usize> {
    debug_assert!(!key.is_empty(), "an empty key would underflow the range");
    (0..rest.len().saturating_sub(key.len() - 1))
        .find(|&at| rest[at..].starts_with(key) && (at == 0 || rest[..at].ends_with(' ')))
}
