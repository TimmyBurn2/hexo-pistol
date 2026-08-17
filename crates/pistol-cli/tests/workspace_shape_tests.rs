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
/// Read by scanning rather than by parsing: this crate has no TOML dependency,
/// and adding one so that a test can read a manifest would be a strange way to
/// keep a dependency list short. Every dependency in this workspace is written on
/// one line, which is what makes the scan enough.
fn dependency_names(manifest: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut inside = false;
    for raw in manifest.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
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
    let mut names = dependency_names(&manifest("pistol-cli"));
    names.sort();
    assert_eq!(
        names,
        vec![String::from("pistol-core"), String::from("pistol-engine")],
        "this crate says everything it says to an engine through the `Engine` trait \
         (CLAUDE.md rule 11); a search or eval dependency here would be reaching past the seam"
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
