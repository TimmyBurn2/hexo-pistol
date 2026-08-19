//! `pistol` — the engine binary.
//!
//! Three commands, one build, one engine-construction path. That last part is
//! the point of keeping them in one binary rather than three: the determinism
//! gate compares two runs of *this* program, and a `selftest` that built its
//! engine differently from the protocol command would be certifying something
//! else (docs/decisions.md D-7).
//!
//! Every path here fails loudly and says which document or which flag it
//! refused (CLAUDE.md rule 3). There is no default config path, no built-in
//! configuration, and no command that runs with less than it needs.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pistol_cli::count::plain_count;
use pistol_cli::fixture_loader;
use pistol_cli::perft;
use pistol_cli::selftest;
use pistol_cli::{Session, serve};
use pistol_engine::{Config, Pistol};

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
pistol — a classical search engine for hex-lattice Connect(6,2,1)

usage:
  pistol --config <path>                      speak the line protocol on stdin
  pistol perft --depth <turns> [--plies \"<q,r> …\"]
  pistol selftest --fixtures <path>
  pistol --help

  --config      an engine config. Always explicit: there is no default path and
                no built-in configuration (CLAUDE.md rule 1). Paths *inside* a
                config — the eval weights file — resolve against the working
                directory, so run this from the directory the config was written
                for.
  --plies       the position, as the stones played in order. Absent means the
                empty board.
  --fixtures    a sha-pinned tactical fixture. It names the config it was
                pre-registered against, so `selftest` takes no --config.

  A `movetime N` budget is accepted in play mode and is a CEILING: the answer
  arrives within N + epsilon, where epsilon is the config's
  play.movetime_epsilon_ms, advertised on the handshake. Before deepening the
  engine secures a bounded fallback answer, so every iteration may be
  interrupted; under an abort the report's depth_turns counts only COMPLETED
  depths, and may be 0 (docs/decisions.md D-95 superseded, WP-1.4 series).
  Wall-clock results are never evidence for a strength claim (CLAUDE.md rule 6).

  `score mate T` counts EVERY turn from the root, both sides', not the winner's
  own turns: an odd T is a win for the side to move at the root, an even T a loss,
  and `-mate T` is that loss from the mated side (docs/decisions.md D-3, D-72,
  D-98). A driver that assumes the other convention halves every distance it reads.

  `bench` is not implemented. A benchmark ships with the first perf-sensitive
  change, its pre-registered hotspot and its abort threshold, not before
  (CLAUDE.md rule 5, docs/decisions.md D-14).

exit: 0 success, 1 a gate failed, 2 usage, or a document this build refuses.
";

/// Exit code for a gate that ran and did not hold.
const GATE_FAILED: u8 = 1;
/// Exit code for anything this program refused before doing work.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match dispatch(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("pistol: {why}");
            ExitCode::from(REFUSED)
        }
    }
}

/// Pick the command, or say why not.
fn dispatch(words: &[&str]) -> Result<ExitCode, String> {
    match words {
        [] => Err(format!("no command given\n\n{USAGE}")),
        ["--help" | "-h"] => {
            print!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        ["perft", rest @ ..] => perft_command(rest),
        ["selftest", rest @ ..] => selftest_command(rest),
        rest => protocol_command(rest),
    }
}

/// Speak the protocol on stdin and stdout.
fn protocol_command(words: &[&str]) -> Result<ExitCode, String> {
    let flags = flags(words)?;
    let path = PathBuf::from(one(&flags, "--config")?);
    only(&flags, &["--config"])?;

    let config = Config::load(&path).map_err(|error| error.to_string())?;
    let weights_file = config.eval.weights_file.clone();
    let mut identity = identity_lines(&path, &config);
    // The engine is built FIRST, so a missing or corrupt weights file dies
    // with pistol-eval's named `eval.weights_file` error, and the identity
    // digest below is the SECOND read of a file the eval just loaded
    // (docs/decisions.md D-198; REVIEW-impl caught this order reversed).
    let mut engine = Pistol::from_config(config).map_err(|error| error.to_string())?;
    identity.push(format!("weights_sha256 {}", weights_digest(&weights_file)?));
    let mut session = Session::new(&mut engine).identify(identity);

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serve(&mut session, &mut input, &mut output).map_err(|io| format!("i/o: {io}"))?;
    output.flush().map_err(|io| format!("i/o: {io}"))?;
    Ok(ExitCode::SUCCESS)
}

/// The handshake lines that make a transcript reproducible: which document, and
/// the values from it that decide what the search does (CLAUDE.md rule 6).
/// The caller appends `weights_sha256` after the engine is built (see
/// [`weights_digest`] for why the order matters).
///
/// Play mode additionally advertises `movetime_epsilon_ms` — the ceiling
/// contract a driver sizes its clamp against (WP-1.4). Instrument mode does
/// NOT: it refuses movetime by name, and its handshake is pinned byte-for-byte
/// against the pre-WP-1.4 revision.
fn identity_lines(path: &Path, config: &Config) -> Vec<String> {
    let pistol_engine::config::CandidatePolicy::Radius { radius } = config.search.candidate_policy;
    let mut lines = vec![
        format!("config {}", path.display()),
        format!("eval {}", config.eval.backend.token()),
        format!("tt_bytes {}", config.search.tt_bytes),
        format!("candidate_policy radius {radius}"),
    ];
    if config.engine.mode == pistol_engine::EngineMode::Play {
        lines.push(format!(
            "movetime_epsilon_ms {}",
            config.play.movetime_epsilon_ms
        ));
    }
    lines
}

/// The eval weight table, identified by CONTENT.
///
/// Two configs that differ only in the weights file used to produce
/// byte-identical arena identities while `nelo_pair` moved by 98 points — the
/// provenance hole WP-1.3 recorded (docs/decisions.md D-188, D-198). The digest
/// travels in the handshake because the engine is the process that actually
/// loads the file, resolved against ITS working directory; a referee digesting
/// the path on its own could attest bytes this process never read.
///
/// A limit, stated rather than hidden: this is a SECOND read of the file,
/// after the eval loaded it, not a digest of the bytes the eval parsed — the
/// loader would have to digest what it read, and it sits below this
/// workspace's one hashing implementation (docs/decisions.md D-198 records
/// why that is declined for now). A swap landing between the eval's read and
/// this one surfaces as a named `IdentityDrift` abort at the arena's next
/// spawn, never as a silently wrong number.
fn weights_digest(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|io| {
        format!(
            "eval.weights_file `{}`: cannot read for the identity digest: {io}",
            path.display()
        )
    })?;
    Ok(pistol_cli::sha256::sha256_hex(&bytes))
}

/// Count turn sequences from a stated position.
fn perft_command(words: &[&str]) -> Result<ExitCode, String> {
    let flags = flags(words)?;
    let stated = one(&flags, "--depth")?;
    let depth: u32 = plain_count(stated)
        .map_err(|why| format!("--depth takes a depth in turns: {why} (given `{stated}`)"))?;
    let plies = perft::parse_plies(optional(&flags, "--plies")?.unwrap_or(""))?;
    only(&flags, &["--depth", "--plies"])?;

    let turns = perft::count(&plies, depth).map_err(|error| error.to_string())?;
    println!("perft depth {depth} turns {turns}");
    Ok(ExitCode::SUCCESS)
}

/// Run the sha-pinned suite: determinism, and the pre-registered threshold.
fn selftest_command(words: &[&str]) -> Result<ExitCode, String> {
    let flags = flags(words)?;
    let path = PathBuf::from(one(&flags, "--fixtures")?);
    only(&flags, &["--fixtures"])?;

    let suite = fixture_loader::load(&path).map_err(|error| error.to_string())?;
    let mut configs = Vec::new();
    for named in suite.configs() {
        let config = Config::load(&named).map_err(|error| error.to_string())?;
        configs.push((named, config));
    }
    println!(
        "selftest: {} cases from {} (required {}), configs: {}",
        suite.cases.len(),
        path.display(),
        suite.required,
        configs
            .iter()
            .map(|(named, _)| named.display().to_string())
            .collect::<Vec<String>>()
            .join(", "),
    );
    let report = selftest::run(&configs, &suite).map_err(|error| error.to_string())?;
    print!("{report}");
    println!();
    if report.holds() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(GATE_FAILED))
}

/// `--name value` pairs, in the order given.
///
/// Hand-rolled rather than taken from a dependency: three commands with five
/// flags between them do not justify one, and the workspace's dependency list is
/// something a reader can hold in their head.
fn flags<'a>(words: &[&'a str]) -> Result<Vec<(&'a str, &'a str)>, String> {
    let mut pairs = Vec::new();
    let mut rest = words;
    while let [name, tail @ ..] = rest {
        if !name.starts_with("--") {
            return Err(format!("expected a flag, got `{name}`\n\n{USAGE}"));
        }
        let [value, tail @ ..] = tail else {
            return Err(format!("`{name}` needs a value"));
        };
        if value.starts_with("--") {
            return Err(format!("`{name}` needs a value, got the flag `{value}`"));
        }
        pairs.push((*name, *value));
        rest = tail;
    }
    Ok(pairs)
}

/// The one value of a flag that must appear exactly once.
fn one<'a>(pairs: &[(&'a str, &'a str)], name: &str) -> Result<&'a str, String> {
    let mut found = pairs.iter().filter(|(flag, _)| *flag == name);
    let Some((_, value)) = found.next() else {
        return Err(format!("`{name}` is required\n\n{USAGE}"));
    };
    if found.next().is_some() {
        return Err(format!("`{name}` is given more than once"));
    }
    Ok(value)
}

/// The value of a flag that may appear once.
fn optional<'a>(pairs: &[(&'a str, &'a str)], name: &str) -> Result<Option<&'a str>, String> {
    if pairs.iter().any(|(flag, _)| *flag == name) {
        return one(pairs, name).map(Some);
    }
    Ok(None)
}

/// Refuse a flag this command does not have. An ignored flag is an instruction
/// that silently did nothing (CLAUDE.md rule 3).
fn only(pairs: &[(&str, &str)], allowed: &[&str]) -> Result<(), String> {
    for (flag, _) in pairs {
        if !allowed.contains(flag) {
            return Err(format!(
                "unknown flag `{flag}`; this command takes {}\n\n{USAGE}",
                allowed.join(", ")
            ));
        }
    }
    Ok(())
}
