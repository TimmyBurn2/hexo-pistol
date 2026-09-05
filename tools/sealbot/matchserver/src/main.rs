//! `pistol-matchserver` — the local HeXO match platform.
//!
//! One config document in, one match out, everything on disk under an
//! artifacts directory: per-game transcripts, per-game engine stderr, and a
//! report (JSON + text). The referee is pistol-core; the engines are
//! subprocesses; nothing between them is guessed.
//!
//! Usage: `pistol-matchserver <config.toml>`
//! Exit: 0 the match ran and was written; 2 anything was refused.

mod budget;
mod client;
mod config;
mod openings;
mod pistol_client;
mod referee;
mod report;
mod sealbot_client;
mod sha256;
mod transcript;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use budget::PistolBudget;
use client::EngineClient;
use config::{EngineKind, EngineSpec};
use pistol_client::PistolClient;
use referee::run_game;
use sealbot_client::SealbotClient;

/// Usage, because a tool without one is a guess.
const USAGE: &str = "usage: pistol-matchserver <config.toml>";

fn main() -> std::process::ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    if owned.len() != 1 || owned[0].starts_with('-') {
        eprintln!("{USAGE}");
        return std::process::ExitCode::from(2);
    }
    match run(&owned[0]) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("pistol-matchserver: {why}");
            std::process::ExitCode::from(2)
        }
    }
}

/// A read deadline `seconds` from now.
fn deadline(seconds: f64) -> Instant {
    Instant::now()
        .checked_add(Duration::from_secs_f64(seconds.max(0.001)))
        .unwrap_or_else(Instant::now)
}

/// Load the config, build the seats, run the games, write everything.
fn run(config_path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(config_path)
        .map_err(|error| format!("read {config_path}: {error}"))?;
    let config = config::load(&text)?;
    let out_dir = PathBuf::from(&config.output_dir);
    std::fs::create_dir_all(&out_dir)
        .map_err(|error| format!("mkdir {}: {error}", out_dir.display()))?;
    if out_dir.join("report.json").exists() {
        return Err(format!(
            "{} already holds a report: an anchor run never overwrites one",
            out_dir.display()
        ));
    }

    // The book is loaded BEFORE either process is spawned, so a bad document
    // refuses at config load rather than after some games are written.
    let book = match config.openings.kind {
        config::OpeningsKind::PlatformStandard => None,
        config::OpeningsKind::Book => {
            let file = config.openings.file.as_deref().expect("checked at load");
            let take = config.openings.take.expect("checked at load");
            let skip = config.openings.skip.expect("checked at load");
            let loaded = openings::load(Path::new(file), take, skip, config.turn_cap)
                .map_err(|why| format!("openings: {why}"))?;
            eprintln!(
                "matchserver: book {} — {} of {} openings, {} turns each, body_sha256 {}",
                file,
                loaded.taken.len(),
                loaded.total,
                loaded.opening_turns,
                loaded.body_sha256
            );
            Some(loaded)
        }
    };
    // How many leading stones the SERVER plays, which the sealbot seat reports
    // as `setup` rather than as its opponent's moves.
    let setup_plies = match &book {
        None => 1,
        Some(loaded) => loaded.taken[0].plies.len(),
    };

    let mut a = build_client(&config.engine_a, &out_dir, "engine_a", setup_plies)?;
    let mut b = build_client(&config.engine_b, &out_dir, "engine_b", setup_plies)?;

    let mut summaries = Vec::with_capacity(config.games as usize);
    for game in 1..=config.games {
        let a_is_p1 = game % 2 == 1;
        // PAIRED: games 2k-1 and 2k are the SAME opening from opposite seats,
        // so an opening's own first-player advantage cancels within the pair
        // instead of being attributed to whichever engine drew it.
        let opening = book.as_ref().map(|loaded| {
            let pair = (game as usize - 1) / 2;
            loaded.taken[pair].plies.as_slice()
        });
        eprintln!(
            "matchserver: game {}/{} ({} as p1)",
            game,
            config.games,
            if a_is_p1 {
                &config.engine_a.label
            } else {
                &config.engine_b.label
            }
        );
        let summary = run_game(game, a_is_p1, config.turn_cap, opening, &mut *a, &mut *b);
        let path = transcript::write_game(&out_dir, &summary)?;
        eprintln!(
            "matchserver: game {} done ({}), transcript {}",
            game,
            summary.kind(),
            path.display()
        );
        summaries.push(summary);
    }

    let built = report::MatchReport::assemble(
        config.games,
        config.turn_cap,
        &label_of(&config.engine_a),
        &label_of(&config.engine_b),
        match &book {
            None => 1,
            Some(loaded) => {
                let mut keys: Vec<&str> = loaded
                    .taken
                    .iter()
                    .map(|opening| opening.position_tail.as_str())
                    .collect();
                keys.sort_unstable();
                keys.dedup();
                keys.len()
            }
        },
        summaries,
    );
    let report_json = out_dir.join("report.json");
    std::fs::write(&report_json, serde_json::to_string_pretty(&built.to_json()).map_err(
        |error| format!("serialise report: {error}"),
    )?)
    .map_err(|error| format!("write {}: {error}", report_json.display()))?;
    let report_txt = out_dir.join("report.txt");
    std::fs::write(&report_txt, built.to_text())
        .map_err(|error| format!("write {}: {error}", report_txt.display()))?;
    eprintln!(
        "matchserver: report {}",
        out_dir.join("report.txt").display()
    );
    Ok(())
}

/// One seat's label for the report.
fn label_of(engine: &EngineSpec) -> String {
    engine.label.clone()
}
/// The budget a validated pistol seat carries.
///
/// `config::load` has already refused a seat that names neither budget or
/// both, so the arms below are the two documents that get this far; the last
/// one is not a fallback but the impossible case said out loud (CLAUDE.md
/// rule 3).
fn pistol_budget(engine: &EngineSpec) -> Result<PistolBudget, String> {
    match (engine.nodes, engine.movetime_ms) {
        (Some(nodes), None) => Ok(PistolBudget::Nodes(nodes)),
        (None, Some(ms)) => Ok(PistolBudget::MovetimeMs(ms)),
        _ => Err("pistol seat reached the client with no single budget".to_string()),
    }
}

/// Build the client a config's kind names. The extension seam: a new engine
/// is a new arm here and a new module, nothing else moves.
fn build_client(
    engine: &EngineSpec,
    out_dir: &Path,
    prefix: &str,
    setup_plies: usize,
) -> Result<Box<dyn EngineClient>, String> {
    match engine.kind {
        EngineKind::Pistol => Ok(Box::new(PistolClient::new(
            engine.label.clone(),
            engine.command.clone(),
            &engine.cwd,
            pistol_budget(engine)?,
            engine.turn_timeout_seconds,
            out_dir,
            prefix,
        ))),
        EngineKind::Sealbot => Ok(Box::new(SealbotClient::new(
            engine.label.clone(),
            engine.command.clone(),
            &engine.cwd,
            engine.time_limit_seconds
                .ok_or("sealbot seat without time_limit_seconds")?,
            engine.turn_timeout_seconds,
            out_dir,
            prefix,
            setup_plies,
        ))),
    }
}