//! The solver fixture loader for the test tree: reads
//! `tests/fixtures/solver_v0.txt` through the crate's own strict loader and
//! pins its SHA-256 (CLAUDE.md rule 7).
//!
//! The pin is asserted HERE, at load time, so every gate that loads the
//! fixture is already standing on the pinned bytes: an edited fixture is a
//! red test before any gate runs.

use std::fs;
use std::path::PathBuf;

use pistol_solver::fixture::FixtureCase;

use super::sha256::sha256_hex;

/// The registered fixture's SHA-256. Editing the fixture without updating
/// this constant in the same commit is a red test.
pub const SOLVER_V0_SHA256: &str =
    "b0afde0349675c41655a920aca96387db172b2a26d920202d982b1498105a194";

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// The deep fixture's SHA-256: the eight decoy wins, excluded from gate
/// (a)'s differential (R3' measured intractable on them) and covered by
/// gate (b)'s verifier.
pub const SOLVER_DEEP_V0_SHA256: &str =
    "59461fe17d0dd66d686724ec1e9b638be3bdfbec56dc09b21b94ba4e3a9e7951";

/// Load the registered fixture, pin first.
pub fn load_solver_fixture(name: &str) -> Vec<FixtureCase> {
    let path = fixture_path(name);
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("fixture {name}: {error}"));
    if bytes.len() > super::FIXTURE_MAX_BYTES {
        panic!(
            "fixture {name} is {} bytes, over the {} ceiling",
            bytes.len(),
            super::FIXTURE_MAX_BYTES
        );
    }
    let text = String::from_utf8(bytes)
        .unwrap_or_else(|error| panic!("fixture {name} is not UTF-8: {error}"));
    let digest = sha256_hex(text.as_bytes());
    assert_eq!(
        digest, SOLVER_V0_SHA256,
        "fixture {name} does not match its pin {SOLVER_V0_SHA256}"
    );
    pistol_solver::fixture::load(&text).unwrap_or_else(|error| panic!("fixture {name}: {error}"))
}

/// Load the deep fixture, pin first.
pub fn load_deep_fixture() -> Vec<FixtureCase> {
    let name = "solver_deep_v0.txt";
    let path = fixture_path(name);
    let bytes = fs::read(&path).unwrap_or_else(|error| panic!("fixture {name}: {error}"));
    if bytes.len() > super::FIXTURE_MAX_BYTES {
        panic!(
            "fixture {name} is {} bytes, over the {} ceiling",
            bytes.len(),
            super::FIXTURE_MAX_BYTES
        );
    }
    let text = String::from_utf8(bytes)
        .unwrap_or_else(|error| panic!("fixture {name} is not UTF-8: {error}"));
    let digest = sha256_hex(text.as_bytes());
    assert_eq!(
        digest, SOLVER_DEEP_V0_SHA256,
        "fixture {name} does not match its pin {SOLVER_DEEP_V0_SHA256}"
    );
    pistol_solver::fixture::load(&text).unwrap_or_else(|error| panic!("fixture {name}: {error}"))
}
