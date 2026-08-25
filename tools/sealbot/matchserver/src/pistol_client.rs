//! The pistol seat: drive the shipped binary over its own line protocol.
//!
//! Instrument mode or nothing: the handshake's `id mode` line is CHECKED to
//! say `instrument` before the first move request, because the determinism
//! law (CLAUDE.md rule 4) is what makes an anchor reproducible, and a play
//! mode seat would quietly break it. The budget is always `go nodes <n>`.
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
/// The only mode an anchor may seat.
const REQUIRED_MODE: &str = "instrument";

/// One configured pistol seat.
pub struct PistolClient {
    label: String,
    command: Vec<String>,
    cwd: std::path::PathBuf,
    nodes: u64,
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
        nodes: u64,
        timeout_seconds: f64,
        out_dir: &Path,
        prefix: &str,
    ) -> PistolClient {
        PistolClient {
            label,
            command,
            cwd: Path::new(cwd).to_path_buf(),
            nodes,
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
        let mut saw_mode = false;
        let mut saw_nodes_budget = false;
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
                    if mode != REQUIRED_MODE {
                        return Err(EngineFailure::Protocol {
                            why: format!(
                                "engine mode is {mode}, not {REQUIRED_MODE}: an anchor seats \
                                 instrument mode"
                            ),
                        });
                    }
                    saw_mode = true;
                }
                if let Some(budgets) = rest.strip_prefix("budgets ") {
                    if !budgets.split_whitespace().any(|word| word == "nodes") {
                        return Err(EngineFailure::Protocol {
                            why: format!("engine budgets do not include nodes: {budgets}"),
                        });
                    }
                    saw_nodes_budget = true;
                }
            }
        }
        if !saw_mode || !saw_nodes_budget {
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
        process.send(&format!("go nodes {}", self.nodes))?;
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
    rest.find(key)
        .and_then(|at| rest[at + key.len()..].split_whitespace().next())
        .and_then(|word| word.parse().ok())
}
