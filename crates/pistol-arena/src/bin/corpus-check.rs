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

  It reads and prints. It writes nothing and removes nothing.

exit: 0 every document loads
      1 a document this build refuses (the reason is named)
      2 THE RUN IS VOID: no document was named, or one could not be read at
        all. A void is not a refusal and must not be read as one.
";

fn main() -> ExitCode {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("corpus_check: RUN VOID: no corpus was named\n\n{USAGE}");
        return ExitCode::from(VOID);
    }
    let mut refused = 0;
    for path in &paths {
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
            Ok(corpus) => println!(
                "corpus_check: {} ok, {} record(s), capture_sha256 {}",
                path.display(),
                corpus.records.len(),
                corpus.capture_sha256
            ),
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
