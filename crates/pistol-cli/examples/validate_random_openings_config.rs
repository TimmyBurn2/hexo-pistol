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
