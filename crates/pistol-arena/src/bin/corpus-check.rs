use std::path::PathBuf;
use std::process::ExitCode;

/// A document this build refuses.
const REFUSED: u8 = 1;
/// No answer was taken: nothing was named, or a named path could not be read.
const VOID: u8 = 2;

const USAGE: &str = "\
corpus-check — read a corpus back through the loader that ships with the writer

usage:
  corpus-check <corpus path>...

  Reads each document through the SAME loader `arena --labels` writes for, and
  reports what it holds. A corpus its own loader refuses is a corpus nothing
  downstream can consume, and the write side's guards check a record's arity
  rather than its grammar — the key shapes, the three score kinds, the four
  token sets and the body digest are only enforced here.

  It also SUMMARISES the columns a pre-registration reads a number off: the
  completed-depth distribution, and how many values each of the closed-set
  columns actually takes. A corpus whose `result` column holds one value is a
  corpus that exercises one arm of that column's guard, and a document that
  concluded otherwise would be reading a green run for more than it says.

  It reads and prints. It writes nothing and removes nothing.

exit: 0 every document loads
      1 a document this build refuses (the reason is named)
      2 THE RUN IS VOID: no document was named, or one could not be read at
        all. A void is not a refusal and must not be read as one.
";

/// The median of a depth column, averaging the two middle values on an even
/// count so the statistic is the one a reader of `statistics.median` expects.
fn median(sorted: &[u32]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        f64::from(sorted[n / 2])
    } else {
        (f64::from(sorted[n / 2 - 1]) + f64::from(sorted[n / 2])) / 2.0
    }
}

/// How many distinct values a closed-set column takes, and which.
fn spread(values: &mut Vec<String>) -> String {
    values.sort();
    values.dedup();
    format!("{} ({})", values.len(), values.join(","))
}

/// The one-line summary of what a corpus holds.
///
/// Separate from the load so that a refusal never prints half of one.
fn summarise(corpus: &pistol_arena::labels_file::Corpus) -> String {
    let mut depths: Vec<u32> = corpus.records.iter().map(|r| r.depth_turns).collect();
    depths.sort_unstable();
    let mean = if depths.is_empty() {
        0.0
    } else {
        depths.iter().map(|d| f64::from(*d)).sum::<f64>() / depths.len() as f64
    };
    let mut kinds: Vec<String> = corpus
        .records
        .iter()
        .map(|r| r.score_kind.clone())
        .collect();
    let mut results: Vec<String> = corpus.records.iter().map(|r| r.result.clone()).collect();
    let mut ends: Vec<String> = corpus.records.iter().map(|r| r.end.clone()).collect();
    let mut moves: Vec<String> = corpus.records.iter().map(|r| r.to_move.clone()).collect();
    format!(
        "depth_turns median {:.1} mean {:.4} min {} max {}; score_kind {}; to_move {}; \
         result {}; end {}",
        median(&depths),
        mean,
        depths.first().copied().unwrap_or(0),
        depths.last().copied().unwrap_or(0),
        spread(&mut kinds),
        spread(&mut moves),
        spread(&mut results),
        spread(&mut ends),
    )
}

/// A caller's path, guarded before it reaches a printed record.
///
/// A newline or a control character in a value this program prints would INJECT
/// LINES into a receipt somebody parses (tools/SHELL_CHECKLIST.md item 9). Its
/// sibling `tools/cold_label_check.py` guards the same boundary; this one did
/// not, which a review found.
fn printable(path: &std::path::Path) -> bool {
    path.display().to_string().chars().all(|c| !c.is_control())
}

fn main() -> ExitCode {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("corpus_check: RUN VOID: no corpus was named\n\n{USAGE}");
        return ExitCode::from(VOID);
    }
    let mut refused = 0;
    for path in &paths {
        if !printable(path) {
            eprintln!("corpus_check: RUN VOID: a named path carries a control character");
            eprintln!("corpus_check: no answer was taken; this is NOT a refusal");
            return ExitCode::from(VOID);
        }
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(io) => {
                eprintln!(
                    "corpus_check: RUN VOID: {} could not be read: {io}",
                    path.display()
                );
                eprintln!("corpus_check: no answer was taken; this is NOT a refusal");
                return ExitCode::from(VOID);
            }
        };
        match pistol_arena::labels_file::read(&text) {
            Ok(corpus) => {
                println!(
                    "corpus_check: {} ok, {} record(s), capture_sha256 {}",
                    path.display(),
                    corpus.records.len(),
                    corpus.capture_sha256
                );
                println!("corpus_check:   {}", summarise(&corpus));
            }
            Err(error) => {
                eprintln!("corpus_check: {} REFUSED: {error}", path.display());
                refused += 1;
            }
        }
    }
    if refused == 0 {
        println!("corpus_check: {} document(s) load", paths.len());
        return ExitCode::SUCCESS;
    }
    eprintln!(
        "corpus_check: {refused} of {} document(s) refused",
        paths.len()
    );
    ExitCode::from(REFUSED)
}
