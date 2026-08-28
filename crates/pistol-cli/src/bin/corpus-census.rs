use std::path::PathBuf;
use std::process::ExitCode;

use pistol_cli::corpus::census::{
    Census, EARLY_TURNS, Keys, Kind, flatten, keys_of_all_at, uncoloured,
};
use pistol_cli::corpus::read;
use pistol_cli::corpus::record::Record;
use pistol_cli::flags;
use pistol_cli::sha256::sha256_hex;
use pistol_core::symmetry::{transform, transform_sequence};

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
corpus-census — symmetry-equivalence census over a corpus of human games

usage:
  corpus-census --corpus <path> [--expect-sha <hex>]

  --corpus      a JSONL corpus of games, one per line. Never committed
                (CLAUDE.md rule 8); the report identifies it by SHA-256.
  --expect-sha  refuse unless the corpus hashes to this, so a census cannot be
                read as a measurement of a document it was not taken over.
  --early-turns the turn boundary the early-position key is cut at. Defaults to
                the shipped constant; a sweep is how the collision profile over
                depth is measured rather than assumed from one cut.

This program REPORTS. It never edits, reorders, or deduplicates a corpus.

exit: 0 no census found a class of size two or more, 1 some census did (which
      is a finding to read, not a failure of this program: only a SEQUENCE or
      FINAL-POSITION class is the WP-P1b step-1 HARD STOP; shared openings are
      expected), 2 a refusal.
";

/// Exit code for a run that found collisions.
const CLASSES_FOUND: u8 = 1;
/// Exit code for anything refused before doing work.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match run(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("corpus-census: {why}");
            ExitCode::from(REFUSED)
        }
    }
}

fn run(words: &[&str]) -> Result<ExitCode, String> {
    if words == ["--help"] || words == ["-h"] {
        print!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    let flags = flags::pairs(words, USAGE)?;
    let corpus_path = PathBuf::from(flags::one(&flags, "--corpus", USAGE)?);
    flags::only(
        &flags,
        &["--corpus", "--expect-sha", "--early-turns"],
        USAGE,
    )?;
    let early_turns = match flags::optional(&flags, "--early-turns")? {
        Some(word) => word
            .parse::<u32>()
            .map_err(|_| format!("`--early-turns` takes a turn number, got `{word}`"))?,
        None => EARLY_TURNS,
    };

    let bytes = std::fs::read(&corpus_path)
        .map_err(|error| format!("cannot read {}: {error}", corpus_path.display()))?;
    let digest = sha256_hex(&bytes);
    if let Some(expected) = flags::optional(&flags, "--expect-sha")?
        && expected != digest
    {
        return Err(format!(
            "{} hashes to {digest}, not the expected {expected}",
            corpus_path.display()
        ));
    }
    let text = String::from_utf8(bytes)
        .map_err(|error| format!("{} is not utf-8: {error}", corpus_path.display()))?;
    let records: Vec<Record> = read(&corpus_path, &text).map_err(|error| error.to_string())?;

    let keys = keys_of_all_at(&records, early_turns);
    let ungrouped = keys.iter().filter(|entry| entry.is_none()).count();

    println!("corpus-census");
    println!("  corpus sha256           {digest}");
    println!("  games read              {}", records.len());
    println!("  games the turn grouping refused  {ungrouped}");
    println!("  early-position cut      turn {early_turns} boundary");
    println!();

    // Each census describes its classes on the SAME shape its key was built
    // from: sequences for the sequence key, boards for the position keys,
    // colour-flattened boards for the uncoloured ones. A describer handed any
    // other shape adjudicates a different question than the key asked.
    let sequence = Census::build(
        Kind::Sequence,
        &keys,
        |k: &Keys| Some(k.sequence.clone()),
        |k: &Keys, s| transform_sequence(&k.turns, s),
    );
    let final_position = Census::build(
        Kind::FinalPosition,
        &keys,
        |k: &Keys| Some(k.final_position.clone()),
        |k: &Keys, s| transform(&k.stones, s),
    );
    let early = Census::build(
        Kind::EarlyPosition,
        &keys,
        |k: &Keys| k.early_position.clone(),
        |k: &Keys, s| transform(&k.early_stones.clone().unwrap_or_default(), s),
    );
    let early_flat = Census::build(
        Kind::EarlyPositionUncoloured,
        &keys,
        |k: &Keys| k.early_stones.as_deref().map(uncoloured),
        |k: &Keys, s| transform(&flatten(&k.early_stones.clone().unwrap_or_default()), s),
    );
    let final_flat = Census::build(
        Kind::FinalPositionUncoloured,
        &keys,
        |k: &Keys| Some(uncoloured(&k.stones)),
        |k: &Keys, s| transform(&flatten(&k.stones), s),
    );

    for census in [&sequence, &final_position, &early, &early_flat, &final_flat] {
        println!("{census}");
        println!();
    }

    for census in [&sequence, &final_position, &early, &early_flat, &final_flat] {
        report_classes(census, &records);
    }

    println!("identity fields this corpus carries");
    println!("  game_hash               yes (SHA-256 over the move sequence)");
    println!("  player names            NO");
    println!("  timestamps              NO");
    println!("  game ids                NO (game_hash is content, not identity)");
    println!("  elo                     yes, per side, nullable");
    println!();

    let verdict = if sequence.classes.is_empty() {
        "no two games are the same game up to a symmetry"
    } else {
        "SOME GAMES ARE THE SAME GAME UP TO A SYMMETRY — escalate (WP-P1b step 1 HARD STOP)"
    };
    println!("sequence-key verdict: {verdict}");

    // Every census counts, not the two the first draft gated on: the
    // uncoloured final-position key is the one that would catch a
    // colour-swapped whole-game duplicate, and an exit code blind to it speaks
    // for four of five censuses (WP-P1b RED-TEAM MAJOR-3).
    let found = [&sequence, &final_position, &early, &early_flat, &final_flat]
        .iter()
        .any(|census| !census.classes.is_empty());
    Ok(if found {
        ExitCode::from(CLASSES_FOUND)
    } else {
        ExitCode::SUCCESS
    })
}

/// Name every member of every class, so a finding can be reproduced by hash.
fn report_classes(census: &Census, records: &[Record]) {
    if census.classes.is_empty() {
        return;
    }
    println!("classes under {}", census.kind);
    for class in &census.classes {
        let hashes: Vec<&str> = class
            .members
            .iter()
            .map(|&index| records[index].game_hash.as_str())
            .collect();
        let elements: Vec<String> = class.elements.iter().map(|s| s.to_string()).collect();
        println!(
            "  size {}  identity-equal {}  elements [{}]  {}",
            class.members.len(),
            class.identical_under_identity,
            elements.join(", "),
            hashes.join(" ")
        );
    }
    println!();
}
