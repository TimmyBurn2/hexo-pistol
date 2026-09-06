use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;

use pistol_cli::random_openings::{
    BookVersion, config::RandomOpeningsConfig, document, filter, generate,
};

const USAGE: &str = "\
build_book_v3 — generate book_v3 and filter it against the two committed books

usage:
  build_book_v3 --config <path> --against <book> [--against <book>] \\
                --openings <n> --out-dir <dir>

  --config    the v3 generator config. Its n_openings is the OVER-generation:
              the filter drops candidates already held by a book named with
              --against, and exactly --openings must survive.
  --against   a committed book to be disjoint from, under canonical form. Given
              once per book.
  --openings  how many openings the finished book holds. A survivor count that
              is not this number is REFUSED rather than truncated.
  --out-dir   where the book is written, under its own version's file name.

The generator is UNCHANGED: this filters AFTER generation, which is what keeps
the change to a version tag (docs/decisions.md D-647).";

fn run(words: &[String]) -> Result<(), String> {
    let mut config_path: Option<PathBuf> = None;
    let mut against: Vec<PathBuf> = Vec::new();
    let mut openings: Option<usize> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut rest = words.iter();
    while let Some(key) = rest.next() {
        let value = rest
            .next()
            .ok_or_else(|| format!("{key} wants a value\n\n{USAGE}"))?;
        match key.as_str() {
            "--config" => config_path = Some(PathBuf::from(value)),
            "--against" => against.push(PathBuf::from(value)),
            "--openings" => {
                openings = Some(value.parse().map_err(|why| format!("--openings: {why}"))?);
            }
            "--out-dir" => out_dir = Some(PathBuf::from(value)),
            other => return Err(format!("unknown flag {other}\n\n{USAGE}")),
        }
    }
    let config_path = config_path.ok_or_else(|| format!("--config is required\n\n{USAGE}"))?;
    let out_dir = out_dir.ok_or_else(|| format!("--out-dir is required\n\n{USAGE}"))?;
    let wanted = openings.ok_or_else(|| format!("--openings is required\n\n{USAGE}"))?;
    if against.is_empty() {
        return Err(format!("--against is required at least once\n\n{USAGE}"));
    }

    let text = std::fs::read_to_string(&config_path)
        .map_err(|io| format!("{}: {io}", config_path.display()))?;
    let config: RandomOpeningsConfig =
        toml::from_str(&text).map_err(|why| format!("{}: {why}", config_path.display()))?;
    config.validate().map_err(|why| why.to_string())?;
    if config.generate.book != BookVersion::V3 {
        return Err(format!(
            "{} states a book this tool does not build; it builds v3 only",
            config_path.display()
        ));
    }

    let mut excluded: BTreeSet<filter::Key> = BTreeSet::new();
    for book in &against {
        let text =
            std::fs::read_to_string(book).map_err(|io| format!("{}: {io}", book.display()))?;
        let keys = filter::keys_of_document(&book.display().to_string(), &text)
            .map_err(|why| why.to_string())?;
        println!(
            "build_book_v3: {} states {} canonical forms",
            book.display(),
            keys.len()
        );
        excluded.extend(keys);
    }
    println!(
        "build_book_v3: {} distinct canonical forms excluded",
        excluded.len()
    );

    let drawn = generate(&config).map_err(|why| why.to_string())?;
    let before = drawn.openings.len();
    let (book, rejected) =
        filter::retain_disjoint(drawn, &excluded, wanted, config.generate.n_openings)
            .map_err(|why| why.to_string())?;
    println!(
        "build_book_v3: drew {before}, rejected {rejected} already held, {} survive",
        book.openings.len()
    );

    let path = out_dir.join(config.generate.book.file_name());
    std::fs::write(&path, document::render(&config, &book).as_bytes())
        .map_err(|io| format!("{}: {io}", path.display()))?;
    println!("build_book_v3: wrote {}", path.display());
    Ok(())
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match run(&words) {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("build_book_v3: FAIL: {why}");
            ExitCode::from(1)
        }
    }
}
