#![allow(dead_code)] // each test file uses a subset of these helpers.

use pistol_engine::{Config, EngineError};

/// A complete, in-range, instrument-mode document.
pub const VALID: &str = r#"
schema_version = 3

[engine]
mode = "instrument"

[search]
tt_bytes = 1048576

[search.candidate_policy]
kind = "radius"
radius = 3

[eval]
backend = "handcrafted_v0"
weights_file = "configs/eval_v0_weights.toml"

[instrument]
threads = 1
tie_break = "lexicographic"

[play]
movetime_epsilon_ms = 50

[solver]
on_search_path = false
per_call_node_cap = 16384
trigger = "any_open_four"
epsilon_num = 1
epsilon_den = 4
zone_orders = 3
free_stone_radius = 8
tt_entries = 1048576
attacker_policy = "one_free_stone"
"#;

/// [`VALID`] with one substring rewritten.
pub fn replacing(from: &str, to: &str) -> String {
    assert!(VALID.contains(from), "fixture has no `{from}` to replace");
    VALID.replace(from, to)
}

/// A complete, in-range, instrument-mode document under
/// `CandidatePolicy::Staged` (`U3_tier_t.md` §10's schema).
pub const VALID_STAGED: &str = r#"
schema_version = 3

[engine]
mode = "instrument"

[search]
tt_bytes = 1048576

[search.candidate_policy]
kind = "staged"
quiet_radius = 2
safety_net_top_k = 0
tier_t_top_k = 0
# Re-sort the ROOT's candidates by the previous iteration's scores (W2, the one
# ordering mechanism the opponent study found sealbot has and this engine does
# not). false is the committed value until an SPRT says otherwise.
root_reorder = false
# The aspiration window's half-width in evaluation units (S1): each iteration
# after the first opens (previous - delta, previous + delta) and re-searches at
# full width if the score falls outside. 0 opens every iteration full width and
# is the committed value until an SPRT says otherwise.
aspiration_delta = 0
# One-cell forced-reply extensions a single line may grant (S2): a FILTERED row
# whose cover is one cell is singular by construction, so extending it a whole
# turn costs branching 1. 0 grants none and is the committed value.
extension_budget = 0
# LATE MOVE REDUCTIONS (S3). Least depth in TURNS a child must still have for a
# candidate deep in a BATCHED row's unforced range to be scanned one turn
# shallower; any such scan that beats alpha is re-searched at full depth before
# it may change a move. 0 arms no reduction and is the committed value.
lmr_min_depth_turns = 0
# Candidates of the unforced range scanned at full depth before reductions
# begin. Read only when lmr_min_depth_turns is non-zero.
lmr_late_index = 3
tier_t_own_count = 2
tier_t_opponent_count = 3
q_depth_turns = 0
q_triggers = "defensive_only"
killers = false
history = false
countermove = false

[eval]
backend = "handcrafted_v0"
weights_file = "configs/eval_v0_weights.toml"

[instrument]
threads = 1
tie_break = "lexicographic"

[play]
movetime_epsilon_ms = 50

[solver]
on_search_path = false
per_call_node_cap = 16384
trigger = "any_open_four"
epsilon_num = 1
epsilon_den = 4
zone_orders = 3
free_stone_radius = 8
tt_entries = 1048576
attacker_policy = "one_free_stone"
"#;

/// [`VALID_STAGED`] with one substring rewritten.
pub fn replacing_staged(from: &str, to: &str) -> String {
    assert!(
        VALID_STAGED.contains(from),
        "staged fixture has no `{from}` to replace"
    );
    VALID_STAGED.replace(from, to)
}

/// [`VALID`] with every line whose trimmed form starts with `prefix` removed.
pub fn without_key(prefix: &str) -> String {
    let kept: Vec<&str> = VALID
        .lines()
        .filter(|line| !line.trim_start().starts_with(prefix))
        .collect();
    assert!(
        kept.len() < VALID.lines().count(),
        "fixture has no line starting with `{prefix}`"
    );
    kept.join("\n")
}

/// [`VALID_STAGED`] with every line whose trimmed form starts with `prefix`
/// removed — the staged document's own `without_key`, so a schema test can
/// drop the staged-only keys (WP-1.7's three gates included) and see the
/// refusal.
pub fn without_staged_key(prefix: &str) -> String {
    let kept: Vec<&str> = VALID_STAGED
        .lines()
        .filter(|line| !line.trim_start().starts_with(prefix))
        .collect();
    assert!(
        kept.len() < VALID_STAGED.lines().count(),
        "staged fixture has no line starting with `{prefix}`"
    );
    kept.join("\n")
}

/// [`VALID`] with a whole table removed, sub-tables included.
pub fn without_table(name: &str) -> String {
    let header = format!("[{name}]");
    let sub_prefix = format!("[{name}.");
    let mut kept = Vec::new();
    let mut dropping = false;
    for line in VALID.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('[') {
            dropping = trimmed.starts_with(&header) || trimmed.starts_with(&sub_prefix);
        }
        if !dropping {
            kept.push(line);
        }
    }
    assert!(
        kept.len() < VALID.lines().count(),
        "fixture has no `{header}`"
    );
    kept.join("\n")
}

/// Parse and validate, expecting a rejection; yields `(key, why)`.
///
/// Panics loudly if the document is accepted — a schema test that silently
/// passes because nothing was checked is worse than no test.
pub fn rejection(document: &str) -> (String, String) {
    let outcome = Config::parse_unvalidated(document).and_then(|config| {
        config.validate()?;
        Ok(config)
    });
    match outcome {
        Err(EngineError::Config { key, why }) => (key, why),
        Err(other) => panic!("expected a config rejection, got: {other}"),
        Ok(_) => panic!("expected a rejection, but this was accepted:\n{document}"),
    }
}

/// The repository root, from this package's location.
pub fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root is two directories up from this package")
}

/// [`VALID`], validated, with the committed weights file named absolutely.
///
/// A config names its weights file relative to the working directory
/// (docs/decisions.md D-21 keeps validation from touching the filesystem at all),
/// and a cargo test's working directory is its own package — which a test cannot
/// change, because the process is shared with every other test in the binary. So
/// the absolute path is stated here rather than inherited from wherever cargo was
/// invoked.
pub fn buildable(document: &str) -> Config {
    let mut config = accepted(document);
    config.eval.weights_file = repo_root().join("configs/eval_v0_weights.toml");
    config
}

/// Parse and validate, expecting success.
pub fn accepted(document: &str) -> Config {
    let config = Config::parse_unvalidated(document)
        .unwrap_or_else(|error| panic!("fixture should parse, got: {error}"));
    config
        .validate()
        .unwrap_or_else(|error| panic!("fixture should validate, got: {error}"));
    config
}
