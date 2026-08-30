use std::path::PathBuf;
use std::process::ExitCode;

use pistol_cli::flags;
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::error::RandomOpeningsError;
use pistol_cli::random_openings::{document, generate};

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
random-openings — a seeded synthetic opening book

usage:
  random-openings --config <path> --out-dir <dir>

  --config    a random-openings TOML document. Every parameter that shapes the
              book is in it; there is no flag that overrides one, because a
              fixture whose shape depended on an operator's typing would not be
              reproducible from its own header.
  --out-dir   where the book named by the document's `[generate] book` key is
              written. The NAME is this tool's and not the operator's; which of
              the tool's names is used is the document's. An existing file of
              that name is overwritten — so a v2 document never touches v1's
              bytes, and a v1 document never touches v2's.

exit: 0 written, 2 a document or a run this build refuses.
";

/// Exit code for anything refused.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match run(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("random-openings: {why}");
            ExitCode::from(REFUSED)
        }
    }
}

/// Read the config, generate, write.
fn run(words: &[&str]) -> Result<ExitCode, String> {
    if words == ["--help"] || words == ["-h"] {
        print!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    let found = flags::pairs(words, USAGE)?;
    let config_path = PathBuf::from(flags::one(&found, "--config", USAGE)?);
    let out_dir = PathBuf::from(flags::one(&found, "--out-dir", USAGE)?);
    flags::only(&found, &["--config", "--out-dir"], USAGE)?;

    let config = RandomOpeningsConfig::load(&config_path).map_err(|error| error.to_string())?;
    let book = generate(&config).map_err(|error| error.to_string())?;
    let rendered = document::render(&config, &book);

    // Staged then renamed, so a full disk or an unwritable target fails while
    // the committed book is still the committed book. One file, so this is a
    // rename and not the pre-flight its two-file sibling needs.
    std::fs::create_dir_all(&out_dir)
        .map_err(|error| format!("cannot create {}: {error}", out_dir.display()))?;
    let file_name = config.generate.book.file_name();
    let staged = out_dir.join(format!("{file_name}.staged"));
    let final_path = out_dir.join(file_name);
    write(&staged, &rendered).map_err(|error| error.to_string())?;
    if let Err(error) = std::fs::rename(&staged, &final_path) {
        let _ = std::fs::remove_file(&staged);
        return Err(format!(
            "cannot put {} in place: {error}",
            final_path.display()
        ));
    }

    println!("book                     {}", config.generate.book.label());
    println!("openings                 {}", book.openings.len());
    println!("k_stones                 {}", config.generate.k_stones);
    println!(
        "sampled from             {} cells within {} of the origin",
        book.ball_cells, config.generate.max_radius
    );
    println!(
        "candidates drawn         {} ({} discarded as a symmetry duplicate)",
        book.candidates_drawn, book.symmetry_collisions
    );
    println!("seed                     {}", config.generate.seed);
    println!("written to               {}", final_path.display());
    Ok(ExitCode::SUCCESS)
}

/// Write the rendered book.
fn write(path: &PathBuf, rendered: &str) -> Result<(), RandomOpeningsError> {
    std::fs::write(path, rendered).map_err(|io| RandomOpeningsError::Write {
        path: path.clone(),
        why: io.to_string(),
    })
}
