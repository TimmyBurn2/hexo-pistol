use std::process::ExitCode;

use pistol_core::{Coord, Player, canonical_form};
use pistol_engine::PositionSpec;

const USAGE: &str = "\
book_keys — print the canonical form of every opening a book states, one per line

usage:
  book_keys <book path>

The rendering matches crates/pistol-arena/src/labels.rs's `key_full` exactly —
`q,r:p1` pairs, space-joined, over pistol-core's `canonical_form` — so a key
printed here is comparable with a corpus row's `key_full` column by string
equality. It GENERATES NOTHING: it shares no draw, no filter and no rendering
with the book builder, which is what makes it usable as that builder's
independent check. It DOES share `canonical_form` — the fold itself — and must,
since that is the identity D-644 is stated over (docs/decisions.md D-647).";

fn render(stones: &[(Coord, Player)]) -> String {
    stones
        .iter()
        .map(|(at, player)| format!("{at}:{}", player.name()))
        .collect::<Vec<String>>()
        .join(" ")
}

fn run(path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|io| format!("{path}: {io}"))?;
    let mut printed = 0usize;
    for (index, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let spec: PositionSpec = line
            .parse()
            .map_err(|why| format!("{path}:{}: {why}", index + 1))?;
        let state = spec
            .replay()
            .map_err(|why| format!("{path}:{}: {why}", index + 1))?;
        println!(
            "{}",
            render(&canonical_form(&state.played().collect::<Vec<_>>()))
        );
        printed += 1;
    }
    if printed == 0 {
        return Err(format!("{path} states no openings"));
    }
    Ok(())
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    let [path] = words.as_slice() else {
        eprintln!("book_keys: FAIL: exactly one book path is required\n\n{USAGE}");
        return ExitCode::from(1);
    };
    match run(path) {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("book_keys: FAIL: {why}");
            ExitCode::from(1)
        }
    }
}
