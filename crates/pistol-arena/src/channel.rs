use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::ArenaError;
use crate::reap::{self, Death};

/// The longest line the arena will accept from an engine.
///
/// Generous: no answer this workspace writes comes near it, so it fires only on
/// input that is trying to make it fire.
pub const MAX_LINE_BYTES: usize = 1 << 20;

/// How many stderr lines are kept for a diagnosis.
///
/// A tail rather than everything: a child that panics in a loop must not be
/// able to fill the arena's memory with its own account of it.
pub const STDERR_TAIL_LINES: usize = 16;

/// What a read produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Received {
    /// One line, newline stripped.
    Line(String),
    /// The engine wrote more than [`MAX_LINE_BYTES`] without a newline. Not a
    /// line, and deliberately not treated as one.
    Overlong,
    /// The engine closed its side. Whether that forfeits or abandons the run is
    /// [`Channel::death`]'s answer, not this one's.
    Closed,
}

/// What the reader thread hands over.
#[derive(Debug, Clone, PartialEq, Eq)]
enum FromEngine {
    /// A complete line.
    Line(String),
    /// The cap was reached with no newline in sight.
    Overlong,
}

/// A live conversation with one engine subprocess.
pub struct Channel {
    label: String,
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<FromEngine>,
    stderr: Arc<Mutex<Vec<String>>>,
}

impl Channel {
    /// Start `binary --config config` and begin reading its answers.
    pub fn start(label: &str, binary: &Path, config: &Path) -> Result<Channel, ArenaError> {
        let mut child = Command::new(binary)
            .arg("--config")
            .arg(config)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|io| ArenaError::Spawn {
                engine: label.to_string(),
                binary: binary.display().to_string(),
                why: io.to_string(),
            })?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take().ok_or_else(|| ArenaError::Spawn {
            engine: label.to_string(),
            binary: binary.display().to_string(),
            why: String::from("no stdout pipe"),
        })?;
        let stderr = child.stderr.take().ok_or_else(|| ArenaError::Spawn {
            engine: label.to_string(),
            binary: binary.display().to_string(),
            why: String::from("no stderr pipe"),
        })?;

        let (sender, lines) = channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut bytes: Vec<u8> = Vec::new();
            loop {
                bytes.clear();
                // Bounded so one unterminated line cannot exhaust memory.
                let read = Read::by_ref(&mut reader)
                    .take(MAX_LINE_BYTES as u64)
                    .read_until(b'\n', &mut bytes);
                match read {
                    Ok(0) | Err(_) => return,
                    Ok(_) => {}
                }
                let complete = bytes.last() == Some(&b'\n');
                if !complete && bytes.len() >= MAX_LINE_BYTES {
                    // Over the cap with no newline: not a line. Reported as
                    // such and the thread stops, because everything after this
                    // point in the stream is the tail of something the arena
                    // already refused.
                    let _ = sender.send(FromEngine::Overlong);
                    return;
                }
                let text = String::from_utf8_lossy(&bytes);
                let line = text.trim_end_matches(['\n', '\r']).to_string();
                if sender.send(FromEngine::Line(line)).is_err() {
                    return;
                }
            }
        });

        let tail: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let collector = Arc::clone(&tail);
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                let Ok(mut held) = collector.lock() else {
                    return;
                };
                if held.len() == STDERR_TAIL_LINES {
                    held.remove(0);
                }
                held.push(line);
            }
        });

        Ok(Channel {
            label: label.to_string(),
            child,
            stdin,
            lines,
            stderr: tail,
        })
    }

    /// Which side this is.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Send one line. A broken pipe is a closed engine, not an I/O surprise.
    pub fn send(&mut self, line: &str) -> Result<(), Received> {
        let Some(stdin) = self.stdin.as_mut() else {
            return Err(Received::Closed);
        };
        if writeln!(stdin, "{line}").is_err() || stdin.flush().is_err() {
            return Err(Received::Closed);
        }
        Ok(())
    }

    /// Wait up to `timeout_ms` for one line.
    ///
    /// [`ArenaError::Hung`] on expiry — never a game result. `opening` and
    /// `turn` travel in so the refusal can say where the run stopped.
    pub fn receive(
        &mut self,
        timeout_ms: u64,
        opening: usize,
        turn: u32,
    ) -> Result<Received, ArenaError> {
        match self.lines.recv_timeout(Duration::from_millis(timeout_ms)) {
            Ok(FromEngine::Line(line)) => Ok(Received::Line(line)),
            Ok(FromEngine::Overlong) => Ok(Received::Overlong),
            Err(RecvTimeoutError::Disconnected) => Ok(Received::Closed),
            Err(RecvTimeoutError::Timeout) => Err(ArenaError::Hung {
                engine: self.label.clone(),
                opening,
                turn,
                timeout_ms,
            }),
        }
    }

    /// Anything the engine has ALREADY said that nobody asked for.
    ///
    /// The protocol is request and response: one `go`, a run of `info` lines,
    /// then exactly one `bestmove`. The channel underneath is a plain queue with
    /// no request identifier — the protocol has none to offer — so a line an
    /// engine volunteers is not merely noise: it is read as the answer to the
    /// NEXT question, asked from a different position. That was a live defect,
    /// and its consequence was the worst kind: a second `bestmove` for one `go`
    /// was replayed as the engine's move several turns later, and the game was
    /// recorded as an ordinary clean win with nothing anywhere in the report to
    /// say its move list is not what either engine intended.
    ///
    /// So an unsolicited line is a protocol violation and the referee forfeits
    /// on it. This is checked without waiting — it asks only what has already
    /// arrived — because a wait would be a wall-clock decision inside the
    /// referee, which is the thing D-159 forbids.
    ///
    /// The residual limit, stated rather than implied: a line that arrives late
    /// enough could still be mistaken for the next answer, and no framing this
    /// protocol offers can close that (D-2 pins the verb set, and there is no
    /// request identifier to match on). What is closed is every case where the
    /// engine has spoken by the time it is asked again — which is every engine
    /// that answers twice in one breath, the reachable case.
    pub fn unsolicited(&mut self) -> Option<Received> {
        match self.lines.try_recv() {
            Ok(FromEngine::Line(line)) => Some(Received::Line(line)),
            Ok(FromEngine::Overlong) => Some(Received::Overlong),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Received::Closed),
        }
    }

    /// How a child that closed its pipe died, waiting no longer than the run's
    /// own watchdog. See [`crate::reap`] for why the bound is load-bearing.
    pub fn death(&mut self, timeout_ms: u64) -> Death {
        self.stdin = None;
        reap::death(&mut self.child, timeout_ms)
    }

    /// The last few lines the engine wrote to stderr.
    pub fn stderr_tail(&self) -> Vec<String> {
        self.stderr
            .lock()
            .map(|held| held.clone())
            .unwrap_or_default()
    }

    /// Ask the engine to stop, then make sure it has.
    ///
    /// Best effort by design: this runs after a game is decided, so a child that
    /// ignores `quit` is killed rather than waited on. Nothing here can change a
    /// result.
    pub fn shutdown(&mut self) {
        let _ = self.send(pistol_cli::protocol::QUIT);
        self.stdin = None;
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        // A worker that abandoned a game must not leave a search running: the
        // next game's timings would be measured against it.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
