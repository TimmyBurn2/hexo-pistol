//! The sealbot seat: drive `sealbot_shim.py` over its JSON-lines contract.
//!
//! The shim owns everything sealbot-specific (its python, its module paths);
//! this client only speaks the request/reply shape:
//!
//! ```text
//! request  {"setup": [[q,r], ...], "moves": [[q,r], ...], "time_limit": s}
//! reply    {"moves": [[q,r], ...]}
//! ```
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
        }
    }

    /// The stderr file for the current game's process.
    fn stderr_path(&self) -> PathBuf {
        self.out_dir
            .join(format!("g{:03}_{}.stderr", self.game_no, self.prefix))
    }

    /// One request for the position `plies` describes.
    fn request(plies: &[(Coord, Player)], time_limit: f64) -> Result<String, EngineFailure> {
        let setup: Vec<(i32, i32)> = match plies.split_first() {
            None => {
                return Err(EngineFailure::Protocol {
                    why: "sealbot asked to move on an empty board: the server always plays \
                          the opening first"
                        .to_string(),
                })
            }
            Some(((first, _), _)) => vec![(i32::from(first.q), i32::from(first.r))],
        };
        let moves: Vec<(i32, i32)> = plies[1..]
            .iter()
            .map(|(at, _)| (i32::from(at.q), i32::from(at.r)))
            .collect();
        serde_json::to_string(&json!({ "setup": setup, "moves": moves, "time_limit": time_limit }))
            .map_err(|error| EngineFailure::Io {
                why: format!("serialising the request: {error}"),
            })
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
        Ok(())
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
        let request = Self::request(plies, self.time_limit_seconds)?;
        process.send(&request)?;
        let line = match process.read_line(deadline) {
            Ok(line) => line,
            Err(failure) => {
                process.kill();
                return Err(failure);
            }
        };
        let reply: Value = serde_json::from_str(&line).map_err(|error| {
            EngineFailure::Protocol {
                why: format!("reply is not JSON ({error}): {}", &line[..line.len().min(200)]),
            }
        })?;
        let stones_value = reply["moves"].as_array().ok_or_else(|| EngineFailure::Protocol {
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
        Ok(EngineReply {
            stones,
            nodes: None,
            engine_time_ms: None,
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
