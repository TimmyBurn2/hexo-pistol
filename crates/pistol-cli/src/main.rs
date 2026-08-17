//! `pistol-cli` — the binaries.
//!
//! Reserved purpose: the line-protocol engine (whose I/O mirrors the `Engine`
//! trait one to one), plus `perft`, `bench` and `selftest`. None of them exist
//! yet; WP-01 is workspace scaffold, config, errors and CI, and writes no
//! protocol.
//!
//! Until then this binary refuses work loudly rather than pretending to accept
//! a command it does not implement (CLAUDE.md rule 3).

use std::process::ExitCode;

/// Subcommands the map calls for, none of which exist yet.
const PENDING_COMMANDS: [&str; 4] = ["engine (line protocol)", "perft", "bench", "selftest"];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        eprintln!(
            "pistol-cli: no command is implemented yet; refusing `{}`",
            args.join(" ")
        );
        eprintln!(
            "pistol-cli: pending commands: {}",
            PENDING_COMMANDS.join(", ")
        );
        return ExitCode::from(2);
    }

    println!("pistol-cli {} (WP-01 scaffold)", env!("CARGO_PKG_VERSION"));
    for command in PENDING_COMMANDS {
        println!("  pending: {command}");
    }
    ExitCode::SUCCESS
}
