//! Two of CLAUDE.md's structural rules, made executable.
//!
//! Rule 11: pistol-api stays empty until the API layer is specified, and the
//! `Engine` trait plus this crate's line protocol are the contract that layer will
//! adapt. Rule 2 and the crate map: this crate talks to an engine through that
//! trait and to the rules through pistol-core, and to nothing else.
//!
//! A rule that lives only in a review checklist rots quietly; this makes breaking
//! either one a red test, which is the same argument docs/decisions.md D-29 makes
//! for the no-code-side-default probe.

mod common;

use common::repo;

/// The dependency names in a manifest's `[dependencies]` table.
///
/// Read by scanning rather than by parsing. This crate does now have a TOML
/// dependency — `random-openings` reads a committed config document
/// (docs/decisions.md D-176) — so the original reason for the scan has gone,
/// and it stays a scan for a second one that has not: a manifest test that
/// parsed manifests with the parser under test would be reading the workspace
/// through one of the things it is policing. Every dependency in this workspace
/// is written on one line, which is what makes the scan enough for the inline
/// form; the table form is recognised separately, because a scan that only knew
/// one of the two spellings would be a guard with a door in it.
fn dependency_names(manifest: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut inside = false;
    for raw in manifest.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            // A dependency has two spellings and both count. `[dependencies.x]`
            // names `x` and then holds `version =` and `workspace =` lines that
            // are NOT dependency names, so the section is recorded and the scan
            // stays off through its body. Without this the table form was a way
            // to add any dependency and keep every assertion below green, which
            // is the opposite of what a shape test is for.
            if let Some(name) = line.strip_prefix("[dependencies.") {
                names.push(name.trim_end_matches(']').trim_matches('"').to_string());
                inside = false;
                continue;
            }
            inside = line == "[dependencies]";
            continue;
        }
        if !inside || line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, _)) = line.split_once('=') else {
            panic!("this scan expects one dependency per line, got `{line}`");
        };
        names.push(key.trim().trim_matches('"').to_string());
    }
    names
}

fn manifest(crate_name: &str) -> String {
    let path = repo("crates").join(crate_name).join("Cargo.toml");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

#[test]
fn pistol_cli_manifest_names_only_core_and_engine() {
    // Two claims, asserted separately because only the first is a rule.
    let mut names = dependency_names(&manifest("pistol-cli"));
    names.sort();

    // The seam (CLAUDE.md rule 11). This crate says everything it says to an
    // engine through the `Engine` trait, so a pistol-search or pistol-eval
    // dependency here would be reaching past it.
    let mut inside: Vec<String> = names
        .iter()
        .filter(|name| name.starts_with("pistol-"))
        .cloned()
        .collect();
    inside.sort();
    assert_eq!(
        inside,
        vec![String::from("pistol-core"), String::from("pistol-engine")],
        "a search or eval dependency here would be reaching past the `Engine` seam"
    );

    // The footprint, which is a ledger rather than a rule: every entry is
    // listed so that a fourth cannot arrive unremarked. The three non-pistol
    // ones are the schema discipline CLAUDE.md rule 1 asks of the committed
    // random-openings config, and they cost the shipping binary nothing —
    // pistol-engine already links all three to load its own config
    // (docs/decisions.md D-176). Nothing on the engine's path in this crate
    // reads TOML.
    assert_eq!(
        names,
        vec![
            String::from("pistol-core"),
            String::from("pistol-engine"),
            String::from("serde"),
            String::from("serde_path_to_error"),
            String::from("toml"),
        ],
        "a dependency arrived in pistol-cli without a line in docs/decisions.md saying why"
    );
}

#[test]
fn pistol_arena_manifest_names_only_core_engine_and_cli() {
    // The prefix check in `pistol_api_is_still_empty` admits any `pistol-*`
    // dependency, so without this the arena could grow a pistol-search or
    // pistol-eval dependency and stay green — the same reach past the seam the
    // test above this one exists to stop for the CLI. pistol-cli is admitted
    // because it owns the line protocol's one spelling (docs/decisions.md D-5,
    // D-167): the arena is a CLIENT of that protocol, and a second spelling of
    // the verbs in the arena would be a second protocol.
    let mut names = dependency_names(&manifest("pistol-arena"));
    names.retain(|name| name.starts_with("pistol-"));
    names.sort();
    assert_eq!(
        names,
        vec![
            String::from("pistol-cli"),
            String::from("pistol-core"),
            String::from("pistol-engine"),
        ],
        "the arena talks to engines through the protocol and to the rules through pistol-core;          a search or eval dependency here would be reaching past the seam"
    );
}

#[test]
fn pistol_api_is_still_empty() {
    // Rule 11: no code, and nothing to link against, until there is a spec.
    assert!(
        dependency_names(&manifest("pistol-api")).is_empty(),
        "pistol-api takes no dependency until the API layer is specified"
    );

    let source = repo("crates/pistol-api/src/lib.rs");
    let text = std::fs::read_to_string(&source).expect("pistol-api has a lib.rs");
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        assert!(
            line.is_empty() || line.starts_with("//!") || line.starts_with("//"),
            "pistol-api/src/lib.rs:{} carries code: `{line}`",
            number + 1
        );
    }

    // And no other crate has grown a transport dependency in its place.
    for crate_name in [
        "pistol-core",
        "pistol-eval",
        "pistol-search",
        "pistol-engine",
        "pistol-cli",
        "pistol-solver",
        "pistol-arena",
    ] {
        for name in dependency_names(&manifest(crate_name)) {
            assert!(
                name.starts_with("pistol-")
                    || matches!(name.as_str(), "serde" | "toml" | "serde_path_to_error"),
                "{crate_name} depends on `{name}`, which is not one of this workspace's \
                 dependencies; a transport dependency anywhere is rule 11's breach"
            );
        }
    }
}

#[test]
fn pistol_core_takes_no_dependency_at_all() {
    // The rules layer is std-only, permanently, dev-dependencies included
    // (docs/decisions.md D-37).
    let text = manifest("pistol-core");
    assert!(
        dependency_names(&text).is_empty(),
        "pistol-core is std-only by contract"
    );
    assert!(
        !text.contains("[dev-dependencies]"),
        "including dev-dependencies, which is why its test tree carries its own SHA-256"
    );
}
