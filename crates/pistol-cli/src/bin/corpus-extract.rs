use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pistol_cli::corpus::documents::{bench_fixture, openings_fixture};
use pistol_cli::corpus::openings::Candidate;
use pistol_cli::corpus::stats::Stats;
use pistol_cli::corpus::verdict::Replayed;
use pistol_cli::corpus::{bench, openings, read, replay};
use pistol_cli::flags;
use pistol_cli::sha256::sha256_hex;

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
corpus-extract — opening and bench fixtures from a corpus of human games

usage:
  corpus-extract --corpus <path> --out-dir <dir>

  --corpus    a JSONL corpus of decisive rated games, one per line. It is an
              external artifact and is never committed (CLAUDE.md rule 8); every
              file this writes identifies it by SHA-256 and never by this path,
              so the same bytes give the same outputs on any machine.
  --out-dir   where openings_v1.txt and bench_positions_v1.txt are written. An
              existing file is overwritten.

exit: 0 every game replayed, 1 some game did not (the outputs are still written,
      and every excluded game is listed by hash in both headers), 2 a document
      this build refuses.
";

/// Exit code for a run that completed with games excluded.
const GAMES_EXCLUDED: u8 = 1;
/// Exit code for anything refused before doing work.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match run(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("corpus-extract: {why}");
            ExitCode::from(REFUSED)
        }
    }
}

/// Read, replay, curate, write, report.
fn run(words: &[&str]) -> Result<ExitCode, String> {
    if words == ["--help"] || words == ["-h"] {
        print!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    let flags = flags::pairs(words, USAGE)?;
    let corpus_path = PathBuf::from(flags::one(&flags, "--corpus", USAGE)?);
    let out_dir = PathBuf::from(flags::one(&flags, "--out-dir", USAGE)?);
    flags::only(&flags, &["--corpus", "--out-dir"], USAGE)?;

    let bytes = std::fs::read(&corpus_path)
        .map_err(|error| format!("cannot read {}: {error}", corpus_path.display()))?;
    let digest = sha256_hex(&bytes);
    let text = String::from_utf8(bytes)
        .map_err(|error| format!("{} is not utf-8: {error}", corpus_path.display()))?;
    let records = read(&corpus_path, &text).map_err(|error| error.to_string())?;

    let replays: Vec<Replayed> = records.iter().map(replay::replay).collect();
    let stats = Stats::gather(digest.clone(), &records, &replays);

    let mut excluded: Vec<String> = Vec::new();
    let mut candidates: Vec<Candidate<'_>> = Vec::new();
    for (record, replayed) in records.iter().zip(&replays) {
        if replayed.verdict.is_eligible() {
            candidates.push(Candidate {
                record,
                turns: &replayed.turns,
            });
            continue;
        }
        let index = replayed
            .verdict
            .move_index()
            .map_or_else(|| "-".to_string(), |index| index.to_string());
        excluded.push(format!(
            "{} {} move {index}",
            record.game_hash,
            replayed.verdict.name()
        ));
    }
    excluded.sort();

    // The stats block goes out BEFORE anything can refuse. It is the escalation
    // output — a corpus that does not replay clean is the finding this tool
    // exists to surface (docs/decisions.md D-149) — and a later refusal must not
    // throw away the measurement that motivates it.
    print!("{stats}");
    println!();
    if !excluded.is_empty() {
        // Every excluded game, in full. The fixture headers cap their list
        // because a header is a document rather than a log; this is stdout, has
        // no size budget, and is where the header points.
        println!("EVERY EXCLUDED GAME ({}):", excluded.len());
        for line in &excluded {
            println!("  {line}");
        }
        println!();
    }

    let selection = openings::select(&candidates)
        .map_err(|why| format!("{why}\n\n(the stats block above is still the run's finding)"))?;
    let bench_positions = bench::select(&candidates);

    // The two fixtures carry the same provenance block and are only meaningful
    // as a pair, so a failure writing the second must not leave the first on
    // disk beside a stale partner. Both are rendered, then both are written
    // beside their targets, and only then are they moved into place: a full
    // disk or an unwritable target fails while nothing has been replaced.
    let documents = [
        (
            "openings_v1.txt",
            openings_fixture(&digest, &stats, &selection, &excluded),
        ),
        (
            "bench_positions_v1.txt",
            bench_fixture(&digest, &stats, &bench_positions, &excluded),
        ),
    ];
    std::fs::create_dir_all(&out_dir)
        .map_err(|error| format!("cannot create {}: {error}", out_dir.display()))?;
    let staged: Vec<(PathBuf, PathBuf)> = documents
        .iter()
        .map(|(name, _)| (out_dir.join(format!("{name}.staged")), out_dir.join(name)))
        .collect();
    for ((staged_path, _), (_, rendered)) in staged.iter().zip(&documents) {
        if let Err(why) = write(staged_path, rendered) {
            discard(&staged);
            return Err(why);
        }
    }
    // Both destinations are checked before either is replaced. Two files cannot
    // be moved into place as one operation on an ordinary filesystem, so this is
    // not a transaction — it is a pre-flight that rules out the reachable causes
    // (a target that is a directory, or one that cannot be replaced) while
    // nothing has been touched. If a rename still fails after this, the message
    // says the pair may be inconsistent rather than implying nothing happened.
    for (_, final_path) in &staged {
        if final_path.is_dir() {
            discard(&staged);
            return Err(format!(
                "cannot write {}: it is a directory",
                final_path.display()
            ));
        }
    }
    for (staged_path, final_path) in &staged {
        if let Err(error) = std::fs::rename(staged_path, final_path) {
            discard(&staged);
            return Err(format!(
                "cannot put {} in place: {error}\n(the fixtures are a matched pair; if one was \
                 already replaced, regenerate both)",
                final_path.display()
            ));
        }
    }

    println!(
        "openings                 {} canonical, from {} distinct positions",
        selection.openings.len(),
        selection.distinct_positions
    );
    println!("bench positions          {}", bench_positions.len());
    println!("written to               {}", out_dir.display());
    if stats.excluded() > 0 {
        return Ok(ExitCode::from(GAMES_EXCLUDED));
    }
    Ok(ExitCode::SUCCESS)
}

/// Write a rendered fixture.
fn write(path: &Path, rendered: &str) -> Result<(), String> {
    std::fs::write(path, rendered)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// Remove whatever was staged, so a refusal leaves the directory as it was.
fn discard(staged: &[(PathBuf, PathBuf)]) {
    for (staged_path, _) in staged {
        let _ = std::fs::remove_file(staged_path);
    }
}
