use std::collections::BTreeSet;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use pistol_core::{Coord, GameState, NEIGHBOUR_DIRECTIONS, PlyOutcome, Turn, legal_placements};
use pistol_engine::config::EngineMode;
use pistol_engine::{
    Budget, Engine, EngineError, PositionSpec, Provenance, SearchInfo, SearchOutcome,
    SolverCallCounters, StageCounters,
};

/// What this instrument does wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Behave {
    /// Nothing. Grows its own cluster from the lexicographically smallest
    /// cells, every time.
    Honest,
    /// Nothing wrong either, but a DIFFERENT deterministic engine: it grows its
    /// cluster from the largest cells instead. Two stubs that differ only here
    /// win different openings, which is the only way a test can produce a
    /// sample with pair-to-pair variation and therefore an actual SPRT
    /// crossing — a self-match never can, because every pair scores alike.
    HonestLast,
    /// Answers with a turn that is not legal in the position it was given.
    Illegal,
    /// Answers with a line that is not the protocol.
    Garbage,
    /// Answers `bestmove` with something that is not a turn token.
    BadBestmove,
    /// Never answers `go`.
    Hang,
    /// Exits, with a code, instead of answering.
    Exit,
    /// Claims a protocol version this arena does not speak.
    BadProtocol,
    /// Claims to be in play mode.
    PlayMode,
    /// Plays honestly, but appends a comment line to its OWN config file on
    /// the first `go` it is asked — the deterministic reproducer for a
    /// document edited under a live run, which the arena must catch at the
    /// next spawn's identity re-verification (docs/decisions.md D-199).
    EditOwnConfig,
    /// Plays honestly, but REFUSES any `position` it is given before it has
    /// been sent `newgame`.
    ///
    /// `seats::with_seats` sends `newgame` on every fresh spawn, and on a fresh
    /// process that send is a functional no-op — so no honest engine can tell
    /// whether it happened, and no test built from one could pin it. A REVIEW
    /// of the extraction measured exactly that: deleting the send left the
    /// whole workspace green (docs/decisions.md D-413). This behaviour is the
    /// witness that closes it, and it is a deliberate deviation from a real
    /// engine rather than a claim about one.
    DemandsNewGame,
}

impl Behave {
    fn parse(word: &str) -> Option<Behave> {
        Some(match word {
            "honest" => Behave::Honest,
            "honest_last" => Behave::HonestLast,
            "illegal" => Behave::Illegal,
            "garbage" => Behave::Garbage,
            "bad_bestmove" => Behave::BadBestmove,
            "hang" => Behave::Hang,
            "exit" => Behave::Exit,
            "bad_protocol" => Behave::BadProtocol,
            "play_mode" => Behave::PlayMode,
            "edit_own_config" => Behave::EditOwnConfig,
            "demands_newgame" => Behave::DemandsNewGame,
            _ => return None,
        })
    }

    /// Every spelling, for a refusal that has to list them.
    const ALL: &'static str = "honest, honest_last, illegal, garbage, bad_bestmove, hang, \
                               exit, bad_protocol, play_mode, edit_own_config, demands_newgame";
}

/// The exit code the `exit` behaviour uses. Distinct from this program's own
/// refusal codes so a test can tell "the engine quit" from "the stub refused".
const ENGINE_EXIT_CODE: u8 = 3;

/// An engine that plays the first legal turn it is offered.
struct FirstLegal {
    state: GameState,
    behave: Behave,
}

impl Engine for FirstLegal {
    fn mode(&self) -> EngineMode {
        match self.behave {
            Behave::PlayMode => EngineMode::Play,
            _ => EngineMode::Instrument,
        }
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn new_game(&mut self) {
        self.state = GameState::new_game();
    }

    fn set_position(&mut self, spec: &PositionSpec) -> Result<(), EngineError> {
        self.state = spec.replay()?;
        Ok(())
    }

    fn go_reporting(
        &mut self,
        _budget: Budget,
        report: &mut dyn FnMut(&SearchInfo),
    ) -> Result<SearchOutcome, EngineError> {
        let best = self.greedy()?;
        // Non-zero nodes on purpose: a test that asserts per-side compute was
        // recorded must be able to fail against a driver that records nothing.
        let info = SearchInfo {
            depth_turns: 1,
            seldepth_turns: 1,
            nodes: u64::from(best.stone_count()).max(1),
            search_nodes: u64::from(best.stone_count()).max(1),
            solver_nodes: 0,
            solver_refusals: 0,
            solver_calls: SolverCallCounters::default(),
            nps: 1,
            time_ms: 0,
            pv: vec![best],
            score: 0,
            hashfull_permille: 0,
            stages: StageCounters::default(),
        };
        report(&info);
        Ok(SearchOutcome {
            best,
            info,
            provenance: Provenance::CompletedDepth,
        })
    }
}

impl FirstLegal {
    /// The cheapest move that is not a pass: grow the mover's own cluster.
    ///
    /// Deliberately not `generate_turns`, and not because of taste — that
    /// enumerates the whole radius-8 legal region pairwise, which is the
    /// candidate count SQUARED and is minutes per move at seven stones. A stub
    /// that took minutes per move could not be in a test suite at all.
    ///
    /// Growing the cluster rather than taking the lexicographically smallest
    /// legal cells matters for coverage: the smallest legal cells sit at the
    /// far corner of the region and never form a line, so an arena driven by
    /// that stub would only ever see capped games and the win path would go
    /// untested.
    fn greedy(&self) -> Result<Turn, EngineError> {
        let mover = self.state.to_move();
        let mut wanted: BTreeSet<Coord> = BTreeSet::new();
        for (cell, player) in self.state.played() {
            if player != mover {
                continue;
            }
            for step in NEIGHBOUR_DIRECTIONS {
                let next = Coord::new(cell.q + step.q, cell.r + step.r);
                if self.state.board().get(next).is_none()
                    && self.state.board().in_legal_region(next)
                {
                    wanted.insert(next);
                }
            }
        }
        let mut cells: Vec<Coord> = wanted.into_iter().collect();
        if self.behave == Behave::HonestLast {
            cells.reverse();
        }
        if cells.len() < 2 {
            // No room beside its own stones: fall back to the legal region,
            // which is a placement list rather than a pair enumeration and so
            // is affordable.
            cells = legal_placements(self.state.board());
        }
        let first = *cells
            .first()
            .ok_or_else(|| EngineError::illegal_position(String::from("no legal placement")))?;
        if self.state.stones_owed() == 1 {
            return Ok(Turn::single(first));
        }
        // Rule 4: if the first stone completes a line the turn is ONE stone and
        // a pair would be refused. Asked by placing it, not by guessing.
        let mut probe = self.state.clone();
        let completed = matches!(probe.place(first), Ok(PlyOutcome::Win { .. }));
        if completed {
            return Ok(Turn::single(first));
        }
        let second = *cells
            .get(1)
            .ok_or_else(|| EngineError::illegal_position(String::from("only one legal cell")))?;
        Turn::pair(first, second).map_err(|error| EngineError::illegal_position(error.to_string()))
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match run(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("arena-stub-engine: {why}");
            ExitCode::from(2)
        }
    }
}

fn run(words: &[String]) -> Result<ExitCode, String> {
    let path = match words {
        [flag, path] if flag == "--config" => path.clone(),
        _ => {
            return Err(format!(
                "usage: arena-stub-engine --config <path>\n\nThis is a TEST INSTRUMENT, not an \
                 engine. Its config states `behave <mode>` on one line, and a config that names \
                 no mode is refused rather than defaulted (one of: {}).",
                Behave::ALL
            ));
        }
    };
    let text = std::fs::read_to_string(&path).map_err(|io| format!("cannot read {path}: {io}"))?;
    let word = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .find_map(|line| line.strip_prefix("behave "))
        .ok_or_else(|| {
            format!(
                "{path} states no `behave <mode>` line; one of: {}",
                Behave::ALL
            )
        })?;
    let behave = Behave::parse(word.trim())
        .ok_or_else(|| format!("`{word}` is not a behaviour; one of: {}", Behave::ALL))?;

    eprintln!("arena-stub-engine: TEST INSTRUMENT, behaviour `{word}` — not a real engine");

    let mut engine = FirstLegal {
        state: GameState::new_game(),
        behave,
    };
    serve(&mut engine, behave, &path, &text)
}

/// The read loop, with the one deviation layered over the real session.
fn serve(
    engine: &mut FirstLegal,
    behave: Behave,
    config_path: &str,
    config_text: &str,
) -> Result<ExitCode, String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    // The stub identifies its "weights" as its own behaviour file, by content:
    // the arena REQUIRES a `weights_sha256` field of every engine it will run
    // (docs/decisions.md D-198), and the behaviour file is the one document
    // that decides what this instrument does.
    let weights_line = format!(
        "weights_sha256 {}",
        pistol_cli::sha256::sha256_hex(config_text.as_bytes())
    );
    let mut session = pistol_cli::Session::new(engine).identify(vec![weights_line]);
    let mut config_edited = false;
    let mut told_new_game = false;
    for line in stdin.lock().lines() {
        let line = line.map_err(|io| format!("stdin: {io}"))?;
        if behave == Behave::DemandsNewGame {
            let asked = line.trim_start();
            if asked.starts_with(pistol_cli::protocol::NEW_GAME) {
                told_new_game = true;
            } else if asked.starts_with(pistol_cli::protocol::POSITION) && !told_new_game {
                // An `error` line is what the arena forfeits on, so a spawn that
                // was never sent `newgame` shows up as a forfeit with a named
                // reason rather than as a silently different game.
                writeln!(
                    out,
                    "{} NoNewGame: this instrument was given a position before it was told \
                     `{}`",
                    pistol_cli::report::ERROR_PREFIX,
                    pistol_cli::protocol::NEW_GAME
                )
                .map_err(io_error)?;
                out.flush().map_err(io_error)?;
                continue;
            }
        }
        let asking_to_move = line.trim_start().starts_with(pistol_cli::protocol::GO);
        if asking_to_move && behave == Behave::EditOwnConfig && !config_edited {
            // The edit keeps the document valid — a comment — so an arena that
            // MISSED the drift would play on cleanly, which is exactly the
            // silent continuation the test must be able to observe.
            config_edited = true;
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(config_path)
                .map_err(|io| format!("cannot reopen {config_path}: {io}"))?;
            writeln!(file, "# edited under a live run")
                .map_err(|io| format!("cannot edit {config_path}: {io}"))?;
        }
        if asking_to_move {
            match behave {
                Behave::Hang => {
                    // Answer nothing, ever. The arena's watchdog is what ends
                    // this, and it must end the RUN rather than the game.
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(3600));
                    }
                }
                Behave::Exit => return Ok(ExitCode::from(ENGINE_EXIT_CODE)),
                Behave::Garbage => {
                    writeln!(out, "this is not a protocol line").map_err(io_error)?;
                    out.flush().map_err(io_error)?;
                    continue;
                }
                Behave::BadBestmove => {
                    writeln!(out, "{} not-a-turn", pistol_cli::report::BESTMOVE_PREFIX)
                        .map_err(io_error)?;
                    out.flush().map_err(io_error)?;
                    continue;
                }
                Behave::Illegal => {
                    // The origin is occupied from turn 1 onward, so this is
                    // always illegal in any position the arena can be in.
                    writeln!(
                        out,
                        "{} {}",
                        pistol_cli::report::BESTMOVE_PREFIX,
                        Turn::single(pistol_core::Coord::new(0, 0))
                    )
                    .map_err(io_error)?;
                    out.flush().map_err(io_error)?;
                    continue;
                }
                _ => {}
            }
        }
        let mut answers: Vec<String> = Vec::new();
        let flow = session.line(&line, &mut |answer| answers.push(answer.to_string()));
        for answer in answers {
            writeln!(out, "{}", deviate(&answer, behave)).map_err(io_error)?;
        }
        out.flush().map_err(io_error)?;
        if flow == pistol_cli::Flow::Quit {
            break;
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// The handshake deviations, which are line rewrites rather than control flow.
fn deviate(answer: &str, behave: Behave) -> String {
    match behave {
        Behave::BadProtocol if answer.starts_with("id protocol ") => {
            String::from("id protocol v-not-this-one")
        }
        _ => answer.to_string(),
    }
}

fn io_error(error: io::Error) -> String {
    format!("stdout: {error}")
}
