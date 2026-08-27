//! `pistol-matchserver` — the local HeXO match platform.
//!
//! One config document in, one match out, everything on disk under an
//! artifacts directory: per-game transcripts, per-game engine stderr, and a
//! report (JSON + text). The referee is pistol-core; the engines are
//! subprocesses; nothing between them is guessed.
//!
//! Usage: `pistol-matchserver <config.toml>`
//! Exit: 0 the match ran and was written; 2 anything was refused.

mod client;
mod config;
mod pistol_client;
mod referee;
mod report;
mod sealbot_client;
mod transcript;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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

    let mut a = build_client(&config.engine_a, &out_dir, "engine_a")?;
    let mut b = build_client(&config.engine_b, &out_dir, "engine_b")?;

    let mut summaries = Vec::with_capacity(config.games as usize);
    for game in 1..=config.games {
        let a_is_p1 = game % 2 == 1;
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
        let summary = run_game(game, a_is_p1, config.turn_cap, &mut *a, &mut *b);
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
/// Build the client a config's kind names. The extension seam: a new engine
/// is a new arm here and a new module, nothing else moves.
fn build_client(
    engine: &EngineSpec,
    out_dir: &Path,
    prefix: &str,
) -> Result<Box<dyn EngineClient>, String> {
    match engine.kind {
        EngineKind::Pistol => Ok(Box::new(PistolClient::new(
            engine.label.clone(),
            engine.command.clone(),
            &engine.cwd,
            engine.nodes.ok_or("pistol seat without nodes")?,
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
        ))),
    }
}