use std::str::FromStr;

use pistol_engine::{Engine, EngineError, ParsePositionError, PositionSpec};

use crate::budget_token::{budget_tokens, parse_budget};
use crate::report::{HANDSHAKE_OK, bestmove_line, error_line, id_line, info_line, totals_line};

/// The identity handshake.
pub const HANDSHAKE: &str = "pistol";
/// Start a new game.
pub const NEW_GAME: &str = "newgame";
/// Stand on a stated position.
pub const POSITION: &str = "position";
/// Search, and answer with a move.
pub const GO: &str = "go";
/// Stop reading.
pub const QUIT: &str = "quit";

/// Every verb, for a rejection that has to name them.
pub const VERBS: [&str; 5] = [HANDSHAKE, NEW_GAME, POSITION, GO, QUIT];

/// The name this engine answers to.
pub const ENGINE_NAME: &str = "pistol";
/// The version of the protocol itself, which is not the version of the build.
pub const PROTOCOL_VERSION: &str = "v0";

/// Whether the driver should read another line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    /// Keep reading.
    Continue,
    /// `quit` was given.
    Quit,
}

/// A protocol conversation with one engine.
pub struct Session<'e> {
    engine: &'e mut dyn Engine,
    identity: Vec<String>,
}

impl<'e> Session<'e> {
    /// Talk to this engine.
    pub fn new(engine: &'e mut dyn Engine) -> Session<'e> {
        Session {
            engine,
            identity: Vec::new(),
        }
    }

    /// Add lines to the handshake, after the ones every engine answers with.
    ///
    /// This is how the run becomes reproducible from its own transcript: a
    /// strength claim ships the instrument it was measured with (CLAUDE.md
    /// rule 6), and an arena log that cannot recover the config path, the
    /// candidate radius and the table size cannot be re-run. The lines are given
    /// by whoever built the engine, because the config is theirs and the
    /// `Engine` seam deliberately does not hand one out.
    ///
    /// Each line is written verbatim after `{ID_PREFIX} `; a caller that puts a
    /// newline in one would split a line, so they are folded like every other
    /// answer.
    pub fn identify(mut self, lines: Vec<String>) -> Session<'e> {
        self.identity = lines;
        self
    }

    /// The engine, for a driver that wants to check the game it is running
    /// against the rules' own state (CLAUDE.md rule 2).
    pub fn engine(&self) -> &dyn Engine {
        &*self.engine
    }

    /// Handle one input line, writing every output line to `out`.
    ///
    /// Never fails: a refusal is an output line, because the engine stays alive
    /// (docs/decisions.md D-5).
    pub fn line(&mut self, line: &str, out: &mut dyn FnMut(&str)) -> Flow {
        match self.dispatch(line, out) {
            Ok(flow) => flow,
            Err(error) => {
                out(&error_line(&error));
                Flow::Continue
            }
        }
    }

    /// One line, with the refusal still a value.
    fn dispatch(&mut self, line: &str, out: &mut dyn FnMut(&str)) -> Result<Flow, EngineError> {
        let line = line.trim();
        let (verb, rest) = split_verb(line);
        match verb {
            HANDSHAKE => {
                no_arguments(line, verb, rest)?;
                self.handshake(out);
                Ok(Flow::Continue)
            }
            NEW_GAME => {
                no_arguments(line, verb, rest)?;
                self.engine.new_game();
                Ok(Flow::Continue)
            }
            POSITION => {
                self.position(line, rest)?;
                Ok(Flow::Continue)
            }
            GO => {
                self.go(line, rest, out)?;
                Ok(Flow::Continue)
            }
            QUIT => {
                no_arguments(line, verb, rest)?;
                Ok(Flow::Quit)
            }
            "" => Err(protocol(
                line,
                format!("an empty line is not a verb; the verbs are {}", verb_list()),
            )),
            other => Err(protocol(
                line,
                format!(
                    "unknown verb `{}`; the verbs are {}",
                    quoted(other),
                    verb_list()
                ),
            )),
        }
    }

    /// Who this engine is, and what it will accept.
    ///
    /// The set and the order are fixed — `name`, `version`, `protocol`, `mode`,
    /// `budgets`, then whatever [`Session::identify`] added — and nothing here is
    /// measured or timed, so two builds of the same commit in the same mode
    /// answer the handshake identically.
    fn handshake(&self, out: &mut dyn FnMut(&str)) {
        let mode = self.engine.mode();
        out(&id_line(&format!("name {ENGINE_NAME}")));
        out(&id_line(&format!("version {}", env!("CARGO_PKG_VERSION"))));
        out(&id_line(&format!("protocol {PROTOCOL_VERSION}")));
        out(&id_line(&format!("mode {}", mode.token())));
        out(&id_line(&format!(
            "budgets {}",
            budget_tokens(mode).join(" ")
        )));
        for line in &self.identity {
            out(&id_line(line));
        }
        out(HANDSHAKE_OK);
    }

    /// `position start …` / `position set …`.
    fn position(&mut self, line: &str, rest: &str) -> Result<(), EngineError> {
        let spec = PositionSpec::from_str(rest)
            .map_err(|error: ParsePositionError| protocol(line, error.why))?;
        self.engine.set_position(&spec)
    }

    /// `go <budget> <amount>`.
    ///
    /// The per-depth reports are written as they arrive, and the last `info`
    /// line before `bestmove` is the outcome's own: the last completed depth's
    /// line and score, with the **totals** for the whole search. An interrupted
    /// iteration is discarded as an answer but not as work, and per-side compute
    /// is a reporting requirement (CLAUDE.md rule 6, docs/decisions.md D-80), so
    /// a driver that accounts for compute reads that line and not the one before
    /// it.
    fn go(&mut self, line: &str, rest: &str, out: &mut dyn FnMut(&str)) -> Result<(), EngineError> {
        let budget = parse_budget(line, rest)?;
        let outcome = self
            .engine
            .go_reporting(budget, &mut |info| out(&info_line(info)))?;
        out(&totals_line(&outcome.info));
        out(&bestmove_line(outcome.best));
        Ok(())
    }
}

/// The verb and whatever followed it.
fn split_verb(line: &str) -> (&str, &str) {
    match line.split_once(char::is_whitespace) {
        Some((verb, rest)) => (verb, rest.trim()),
        None => (line, ""),
    }
}

/// Refuse arguments to a verb that takes none.
fn no_arguments(line: &str, verb: &str, rest: &str) -> Result<(), EngineError> {
    if rest.is_empty() {
        return Ok(());
    }
    Err(protocol(
        line,
        format!(
            "`{verb}` takes no arguments, and `{}` follows it",
            quoted(rest)
        ),
    ))
}

/// The verbs, for a message that lists them.
fn verb_list() -> String {
    VERBS.join(", ")
}

/// How much of a rejected line is quoted back in the refusal.
///
/// A refusal names the line it refused, and a driver that sent a megabyte of
/// nonsense should not get a megabyte of it back: the answer has to stay one
/// readable line in a log. The cap is on characters rather than bytes so the
/// quote is never cut through the middle of one.
const QUOTED_CHARS: usize = 120;

/// A line that could not be understood.
pub(crate) fn protocol(line: &str, why: impl Into<String>) -> EngineError {
    EngineError::Protocol {
        line: quoted(line),
        why: why.into(),
    }
}

/// A fragment of input as a refusal quotes it: whole if it is short, elided if it
/// is not. Every message that names something the caller typed goes through this.
pub(crate) fn quoted(line: &str) -> String {
    if line.chars().count() <= QUOTED_CHARS {
        return line.to_string();
    }
    let head: String = line.chars().take(QUOTED_CHARS).collect();
    format!("{head}…")
}
