//! The engine-side contract: one trait, two implementations, one seam.
//!
//! The referee knows nothing about protocols; a client knows nothing about
//! the rules. A new engine is a new client module and a new config `kind`,
//! and the referee does not change (tools/sealbot/README.md, "Shape").

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

use pistol_core::{Coord, Player};

/// One engine's answer to a move request.
#[derive(Debug, Clone)]
pub struct EngineReply {
    /// The stones it plays, in submitted order.
    pub stones: Vec<Coord>,
    /// The engine's own node count, when it reports one.
    pub nodes: Option<u64>,
    /// The engine's own wall time in ms, when it reports it.
    pub engine_time_ms: Option<u64>,
    /// The wall time the whole request took, measured here.
    pub wall_ms: u64,
    /// The essential reply line, recorded verbatim in the transcript.
    pub raw: String,
}

/// Why an engine forfeited a game without the referee judging a stone.
#[derive(Debug, Clone)]
pub enum EngineFailure {
    /// No answer within the configured wall cap.
    Timeout,
    /// The process died or spoke bytes that are not a line.
    Crashed { detail: String },
    /// It answered, but the answer is not a valid reply.
    Protocol { why: String },
    /// Talking to it failed.
    Io { why: String },
}

impl EngineFailure {
    /// One line for the transcript and the report.
    pub fn describe(&self) -> String {
        match self {
            EngineFailure::Timeout => "engine timeout".to_string(),
            EngineFailure::Crashed { detail } => format!("engine process died: {detail}"),
            EngineFailure::Protocol { why } => format!("engine protocol failure: {why}"),
            EngineFailure::Io { why } => format!("engine io failure: {why}"),
        }
    }
}

/// What the referee asks of an engine, and what it gets back.
pub trait EngineClient {
    /// The seat's label, from the config.
    fn label(&self) -> &str;
    /// Prepare for game `game_no` (1-based): spawn a fresh process.
    fn new_game(&mut self, game_no: u32) -> Result<(), EngineFailure>;
    /// Ask for the next turn, given every stone so far in true play order.
    fn pick_turn(
        &mut self,
        plies: &[(Coord, Player)],
        owed: u32,
    ) -> Result<EngineReply, EngineFailure>;
    /// End of game: shut the process down.
    fn finish_game(&mut self);
}

/// One line from the child's stdout: `None` is end of stream.
type StreamItem = std::io::Result<Option<String>>;

/// A spawned engine process speaking lines, with a per-read deadline.
///
/// stdout is drained by a thread into a channel so a read can time out
/// without blocking forever; stderr is drained to a file so a crash's last
/// words survive into the artifacts directory. A process that misses its
/// deadline is killed by the reader's caller, loudly.
pub struct LineProcess {
    child: Child,
    lines: Receiver<StreamItem>,
}

impl LineProcess {
    /// Spawn `command` in `cwd`, stderr to `stderr_path`.
    pub fn spawn(
        command: &[String],
        cwd: &std::path::Path,
        stderr_path: &std::path::Path,
    ) -> Result<LineProcess, EngineFailure> {
        let mut child = Command::new(&command[0])
            .args(&command[1..])
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| EngineFailure::Io {
                why: format!("spawn {:?}: {error}", command[0]),
            })?;
        let stderr = child.stderr.take().ok_or_else(|| EngineFailure::Io {
            why: "no stderr".to_string(),
        })?;
        let stdout = child.stdout.take().ok_or_else(|| EngineFailure::Io {
            why: "no stdout".to_string(),
        })?;
        let stderr_file = std::fs::File::create(stderr_path).map_err(|error| {
            EngineFailure::Io {
                why: format!("create {}: {error}", stderr_path.display()),
            }
        })?;
        thread::spawn(move || {
            let mut sink = std::io::BufWriter::new(stderr_file);
            let mut reader = BufReader::new(stderr);
            let mut bytes = Vec::new();
            loop {
                bytes.clear();
                match reader.read_until(b'\n', &mut bytes) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let _ = sink.write_all(&bytes);
                    }
                }
            }
            let _ = sink.flush();
        });
        let (sender, receiver): (Sender<StreamItem>, _) = channel();
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut bytes = Vec::new();
            loop {
                bytes.clear();
                match reader.read_until(b'\n', &mut bytes) {
                    Ok(0) => {
                        let _ = sender.send(Ok(None));
                        break;
                    }
                    Ok(_) => {
                        let line = String::from_utf8(bytes.clone()).map_err(|error| {
                            std::io::Error::other(format!("not UTF-8: {error}"))
                        });
                        if sender.send(line.map(Some)).is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send(Err(error));
                        break;
                    }
                }
            }
        });
        Ok(LineProcess {
            child,
            lines: receiver,
        })
    }

    /// The child's stdin, for writing.
    pub fn stdin(&mut self) -> Result<&mut ChildStdin, EngineFailure> {
        self.child.stdin.as_mut().ok_or_else(|| EngineFailure::Io {
            why: "stdin closed".to_string(),
        })
    }

    /// Send one line.
    pub fn send(&mut self, line: &str) -> Result<(), EngineFailure> {
        let stdin = self.stdin()?;
        stdin
            .write_all(line.as_bytes())
            .and_then(|_| stdin.write_all(b"\n"))
            .and_then(|_| stdin.flush())
            .map_err(|error| EngineFailure::Io {
                why: format!("write: {error}"),
            })
    }

    /// Close stdin, signalling a well-behaved child to exit.
    pub fn close_stdin(&mut self) {
        self.child.stdin = None;
    }

    /// Kill the process. Used on timeout: a process that missed its deadline
    /// does not get to keep the seat.
    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }

    /// Wait for the process to finish.
    pub fn wait(&mut self) {
        let _ = self.child.wait();
    }

    /// Read one line, or fail by `deadline`.
    pub fn read_line(&mut self, deadline: Instant) -> Result<String, EngineFailure> {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO);
        match self.lines.recv_timeout(remaining) {
            Ok(Ok(Some(line))) => Ok(line.trim_end_matches(['\n', '\r']).to_string()),
            Ok(Ok(None)) => Err(EngineFailure::Crashed {
                detail: "stdout closed".to_string(),
            }),
            Ok(Err(error)) => Err(EngineFailure::Crashed {
                detail: format!("stdout read: {error}"),
            }),
            Err(RecvTimeoutError::Timeout) => Err(EngineFailure::Timeout),
            Err(RecvTimeoutError::Disconnected) => Err(EngineFailure::Crashed {
                detail: "stdout reader gone".to_string(),
            }),
        }
    }
}
