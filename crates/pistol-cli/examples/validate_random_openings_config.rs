//! Validate random-openings configs, for the config gate.
//!
//! An example rather than a subcommand, for the reason docs/decisions.md D-25
//! gives for the engine's and pistol-arena repeats for its own: config checking
//! is a tools-side gate, and a binary's surface should not grow a general
//! maintenance command to hold it.
//!
//! Usage: `cargo run -p pistol-cli --example validate_random_openings_config -- <path> …`
//! Exit:  0 every document validates, 1 one or more were refused, 2 no path.

use std::path::PathBuf;
use std::process::ExitCode;

use pistol_cli::random_openings::config::RandomOpeningsConfig;

fn main() -> ExitCode {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("validate_random_openings_config: give at least one path");
        return ExitCode::from(2);
    }
    let mut refused = 0;
    for path in &paths {
        match RandomOpeningsConfig::load(path) {
            Ok(_) => println!("validate_random_openings_config: {} ok", path.display()),
            Err(error) => {
                eprintln!(
                    "validate_random_openings_config: {} REFUSED: {error}",
                    path.display()
                );
                refused += 1;
            }
        }
    }
    if refused == 0 {
        println!(
            "validate_random_openings_config: {} document(s) ok",
            paths.len()
        );
        return ExitCode::SUCCESS;
    }
    ExitCode::from(1)
}
