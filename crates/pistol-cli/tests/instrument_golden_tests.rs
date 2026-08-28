mod common;

use common::{GATE, engine, repo, talk};
use pistol_cli::sha256::sha256_hex;
use pistol_engine::Engine;

const GOLDEN_FIXTURE: &str = "crates/pistol-cli/tests/fixtures/instrument_golden_v1.txt";
const GOLDEN_SHA256: &str = "8fbcc309f82c06dde138a6afd1fddafb12e86f5cd7ddd87c9bc66d5bbc298057";

/// Every how-many-th case runs in a debug build. 5 keeps ~20 s of debug time
/// while touching 8 cases across both budgets.
const DEBUG_STRIDE: usize = 5;

/// One pinned search: the protocol lines in, the normalized lines out.
struct Case {
    position: String,
    budget: String,
    expected: Vec<String>,
}

/// The fixture, loaded strictly: the pin first, then a grammar where every
/// line is `case`, `position …`, `budget …`, `expect …`, or a comment, and
/// anything else panics naming the line.
fn golden_cases() -> Vec<Case> {
    let path = repo(GOLDEN_FIXTURE);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    assert_eq!(
        sha256_hex(&bytes),
        GOLDEN_SHA256,
        "{GOLDEN_FIXTURE} does not match its pin; regeneration requires a superseding ADR line"
    );
    let text = String::from_utf8(bytes).expect("the fixture is text");

    let mut cases: Vec<Case> = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "case" {
            cases.push(Case {
                position: String::new(),
                budget: String::new(),
                expected: Vec::new(),
            });
            continue;
        }
        let current = cases.last_mut().expect("a directive before any `case`");
        if let Some(rest) = line.strip_prefix("position ") {
            current.position = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("budget ") {
            current.budget = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("expect ") {
            current.expected.push(rest.to_string());
        } else {
            panic!("unknown fixture directive: {line}");
        }
    }
    assert_eq!(cases.len(), 40, "twenty positions, two budgets");
    for case in &cases {
        assert!(
            !case.position.is_empty() && !case.budget.is_empty() && !case.expected.is_empty(),
            "an incomplete case certifies nothing"
        );
    }
    cases
}

/// Strip the two fields that measure the machine instead of the search, the
/// same normalization tools/determinism.sh applies.
fn normalized(line: &str) -> String {
    let mut kept: Vec<&str> = Vec::new();
    let mut tokens = line.split(' ');
    while let Some(token) = tokens.next() {
        if token == "nps" || token == "time" {
            tokens.next();
            continue;
        }
        kept.push(token);
    }
    kept.join(" ")
}

#[test]
fn instrument_behavior_byte_identical_pre_post() {
    let cases = golden_cases();
    let stride = if cfg!(debug_assertions) {
        DEBUG_STRIDE
    } else {
        1
    };

    let mut engine = engine(GATE);
    let mut compared = 0usize;
    for (index, case) in cases.iter().enumerate() {
        if index % stride != 0 {
            continue;
        }
        // `newgame` before each case, exactly as the fixture was generated;
        // D-90's layout comparison is what makes the subset independent of
        // which cases ran before it.
        engine.new_game();
        let answers = talk(
            &mut engine,
            &[
                &format!("position {}", case.position),
                &format!("go {}", case.budget),
            ],
        );
        let got: Vec<String> = answers.iter().map(|line| normalized(line)).collect();
        assert_eq!(
            got, case.expected,
            "case {index} (position `{}`, go {}) diverged from revision 3926110",
            case.position, case.budget
        );
        compared += 1;
    }
    assert!(
        compared >= cases.len() / stride,
        "the subset shrank: {compared} cases compared"
    );
}
