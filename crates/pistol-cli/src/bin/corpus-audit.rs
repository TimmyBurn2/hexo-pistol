use std::path::PathBuf;
use std::process::ExitCode;

use pistol_cli::corpus::audit::Audit;
use pistol_cli::corpus::read;
use pistol_cli::corpus::record::Record;
use pistol_cli::flags;
use pistol_cli::sha256::sha256_hex;

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
corpus-audit — audit a corpus against its own stated source_filter

usage:
  corpus-audit --corpus <path> [--expect-sha <hex>]

  --corpus      a JSONL corpus of games, one per line.
  --expect-sha  refuse unless the corpus hashes to this.

`dataset_metadata.json` claims the population was filtered to \"rated, >=20
moves, decisive by six-in-a-row\". Two of those three conjuncts are decidable
from the pinned bytes with no network, and this audits them. Corroboration
establishes that the stated filter WAS APPLIED and not that it was the ONLY
filter applied; the provenance gap leaves that residual and no byte-level audit
can close it (docs/decisions.md D-456).

This program READS. It never edits a corpus.

exit: 0 both auditable conjuncts hold, 1 one does not (which means the metadata
      describing the dataset is wrong, and outranks anything downstream of it),
      2 a refusal.
";

/// Exit code for a corpus that contradicts its own stated filter.
const FILTER_FALSE: u8 = 1;
/// Exit code for anything refused before doing work.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match run(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("corpus-audit: {why}");
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
    flags::only(&flags, &["--corpus", "--expect-sha"], USAGE)?;

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

    let audit = Audit::of(&records);
    println!("corpus-audit");
    println!("  corpus sha256           {digest}");
    println!("{audit}");
    println!();

    for (label, offenders) in [
        (">= 20 moves", &audit.short),
        ("decisive by six-in-a-row", &audit.indecisive),
        ("continued past a win", &audit.decided_early),
        (
            "`winner` agrees with the winning stone",
            &audit.winner_disagrees,
        ),
        ("replayable at all (repeated cell)", &audit.malformed),
    ] {
        if offenders.is_empty() {
            continue;
        }
        println!("games failing `{label}` (first 20, by game_hash):");
        for &index in offenders.iter().take(20) {
            println!(
                "  {}  {} moves",
                records[index].game_hash,
                records[index].moves.len()
            );
        }
        println!();
    }

    // The third conjunct. `elo` is a schema field, so its PRESENCE is checkable
    // while "rated" as a platform property is not: a rating on every game is
    // consistent with the filter and does not establish it.
    let rated = if audit.rated == audit.total {
        "every game carries a rating for both sides. This measures the `null` \
         case only: a record with NO `elo` key is refused by the reader \
         (ELO_KEY_REQUIRED) and can never be counted here, and a rating being \
         present is in any case consistent with `rated` without being proof of it"
    } else {
        "some game carries a null rating, so `rated` is UNAUDITABLE from these bytes"
    };
    println!("rated conjunct: {rated}");
    println!(
        "source_filter auditable conjuncts: {}",
        if audit.filter_holds() {
            "BOTH HOLD"
        } else {
            "AT LEAST ONE IS FALSE — the metadata is wrong (D-456 STOP)"
        }
    );

    Ok(if audit.filter_holds() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(FILTER_FALSE)
    })
}
