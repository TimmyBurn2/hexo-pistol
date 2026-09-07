//! The sealbot seat: drive `sealbot_shim.py` over its JSON-lines contract.
//!
//! The shim owns everything sealbot-specific (its python, its module paths);
//! this client only speaks the request/reply shape:
//!
//! ```text
//! preamble `sealbot_shim: ready`, once, before any request
//! request  {"setup": [[q,r], ...], "moves": [[q,r], ...], "time_limit": s}
//! reply    {"moves": [[q,r], ...], "engine_time_ms": n}
//! ```
//!
//! The preamble is read in `new_game`, BEFORE the clock starts, so the shim's
//! interpreter start and extension import are not charged to the first answer
//! of a game — the defect docs/decisions.md D-699 records at 26 ms per game.
//! `engine_time_ms` is the engine's own elapsed time for the answer and is
//! REQUIRED: without it nothing separates this seat's search from its
//! overhead, which is the state D-699 stops on.
//!
//! `setup` is the server-played opening (the origin cross); `moves` is every
//! stone after it, in true play order. The stones come back in sealbot's own
//! play order and are submitted to the referee exactly so.

use std::path::{Path, PathBuf};
use std::time::Instant;

use pistol_core::{Coord, Player};
use serde_json::{Value, json};

use crate::client::{EngineClient, EngineFailure, EngineReply, LineProcess};
use crate::deadline;

/// The line the shim writes once its bot is constructed, on stdout and stderr.
const READY: &str = "sealbot_shim: ready";

/// One configured sealbot seat.
pub struct SealbotClient {
    label: String,
    command: Vec<String>,
    cwd: PathBuf,
    time_limit_seconds: f64,
    timeout_seconds: f64,
    out_dir: PathBuf,
    prefix: String,
    process: Option<LineProcess>,
    game_no: u32,
    /// Leading stones the SERVER played, which `request` reports as `setup`.
    setup_plies: usize,
}

impl SealbotClient {
    /// Build the seat from the validated config, the run's output directory
    /// and this seat's file prefix.
    pub fn new(
        label: String,
        command: Vec<String>,
        cwd: &str,
        time_limit_seconds: f64,
        timeout_seconds: f64,
        out_dir: &Path,
        prefix: &str,
        setup_plies: usize,
    ) -> SealbotClient {
        SealbotClient {
            label,
            command,
            cwd: Path::new(cwd).to_path_buf(),
            time_limit_seconds,
            timeout_seconds,
            out_dir: out_dir.to_path_buf(),
            prefix: prefix.to_string(),
            process: None,
            game_no: 0,
            setup_plies,
        }
    }

    /// The stderr file for the current game's process.
    fn stderr_path(&self) -> PathBuf {
        self.out_dir
            .join(format!("g{:03}_{}.stderr", self.game_no, self.prefix))
    }

    /// One request for the position `plies` describes.
    ///
    /// `setup_plies` is how many leading stones the SERVER played — one for the
    /// platform's origin stone, five for a `k_stones = 5` book opening. The
    /// split matters because `setup` and `moves` are different claims about who
    /// played what, and a book opening reported as the opponent's own moves is
    /// a false record even where a shim happens to replay both the same way.
    fn request(
        plies: &[(Coord, Player)],
        setup_plies: usize,
        time_limit: f64,
    ) -> Result<String, EngineFailure> {
        if plies.len() < setup_plies {
            return Err(EngineFailure::Protocol {
                why: format!(
                    "sealbot asked to move on {} stones with a {setup_plies}-stone server \
                     opening: the server always plays the opening first",
                    plies.len()
                ),
            });
        }
        let cells = |slice: &[(Coord, Player)]| -> Vec<(i32, i32)> {
            slice
                .iter()
                .map(|(at, _)| (i32::from(at.q), i32::from(at.r)))
                .collect()
        };
        let setup = cells(&plies[..setup_plies]);
        let moves = cells(&plies[setup_plies..]);
        serde_json::to_string(&json!({ "setup": setup, "moves": moves, "time_limit": time_limit }))
            .map_err(|error| EngineFailure::Io {
                why: format!("serialising the request: {error}"),
            })
    }
    /// Read the shim's readiness line, before anything is timed.
    ///
    /// The deadline is `turn_timeout_seconds`, which is what `PistolClient`'s
    /// own handshake already uses — a start-up that outlasts the seat's answer
    /// budget is the same failure as an answer that does, and it needs no
    /// second knob (CLAUDE.md hard rule 1).
    fn await_ready(&mut self) -> Result<(), EngineFailure> {
        let timeout = self.timeout_seconds;
        let process = self.process.as_mut().expect("spawned");
        let line = process.read_line(deadline(timeout))?;
        if line.trim() != READY {
            return Err(EngineFailure::Protocol {
                why: format!(
                    "expected `{READY}` before the first request; got: {}",
                    &line[..line.len().min(200)]
                ),
            });
        }
        Ok(())
    }
}

impl EngineClient for SealbotClient {
    fn label(&self) -> &str {
        &self.label
    }

    fn new_game(&mut self, game_no: u32) -> Result<(), EngineFailure> {
        if let Some(mut process) = self.process.take() {
            process.close_stdin();
            process.wait();
        }
        self.game_no = game_no;
        let process = LineProcess::spawn(&self.command, &self.cwd, &self.stderr_path())?;
        self.process = Some(process);
        self.await_ready()
    }

    fn pick_turn(
        &mut self,
        plies: &[(Coord, Player)],
        _owed: u32,
    ) -> Result<EngineReply, EngineFailure> {
        let started = Instant::now();
        let process = self.process.as_mut().ok_or_else(|| EngineFailure::Io {
            why: "no process: new_game was not called".to_string(),
        })?;
        let deadline = deadline(self.timeout_seconds);
        let request = Self::request(plies, self.setup_plies, self.time_limit_seconds)?;
        process.send(&request)?;
        let line = match process.read_line(deadline) {
            Ok(line) => line,
            Err(failure) => {
                process.kill();
                return Err(failure);
            }
        };
        let reply: Value =
            serde_json::from_str(&line).map_err(|error| EngineFailure::Protocol {
                why: format!(
                    "reply is not JSON ({error}): {}",
                    &line[..line.len().min(200)]
                ),
            })?;
        let stones_value = reply["moves"]
            .as_array()
            .ok_or_else(|| EngineFailure::Protocol {
                why: format!("reply has no moves array: {}", &line[..line.len().min(200)]),
            })?;
        let mut stones = Vec::with_capacity(stones_value.len());
        for stone in stones_value {
            let pair = stone
                .as_array()
                .filter(|pair| pair.len() == 2)
                .ok_or_else(|| EngineFailure::Protocol {
                    why: format!("a stone is not a [q, r] pair: {stone}"),
                })?;
            let q = pair[0].as_i64().ok_or_else(|| EngineFailure::Protocol {
                why: format!("a stone's q is not an integer: {stone}"),
            })?;
            let r = pair[1].as_i64().ok_or_else(|| EngineFailure::Protocol {
                why: format!("a stone's r is not an integer: {stone}"),
            })?;
            let (q, r) = (
                i16::try_from(q).map_err(|_| EngineFailure::Protocol {
                    why: format!("a stone's q does not fit the board: {stone}"),
                })?,
                i16::try_from(r).map_err(|_| EngineFailure::Protocol {
                    why: format!("a stone's r does not fit the board: {stone}"),
                })?,
            );
            stones.push(Coord::new(q, r));
        }
        // REQUIRED, not optional: a shim that stopped reporting it would put
        // this seat back in the state D-699 stops on — a null column nobody
        // notices — so its absence is a named refusal (hard rule 3). Whole
        // milliseconds: a JSON float would make `as_u64` answer None and fire
        // this refusal for the wrong reason.
        let engine_time_ms =
            reply["engine_time_ms"]
                .as_u64()
                .ok_or_else(|| EngineFailure::Protocol {
                    why: format!(
                        "reply has no whole-millisecond `engine_time_ms`: {}",
                        &line[..line.len().min(200)]
                    ),
                })?;
        Ok(EngineReply {
            stones,
            nodes: None,
            engine_time_ms: Some(engine_time_ms),
            wall_ms: started.elapsed().as_millis() as u64,
            raw: line,
        })
    }

    fn finish_game(&mut self) {
        if let Some(mut process) = self.process.take() {
            process.close_stdin();
            process.wait();
        }
    }
}
