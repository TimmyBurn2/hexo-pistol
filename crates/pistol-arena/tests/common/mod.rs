//! What every arena test needs: a scratch directory, an openings fixture with a
//! correct in-band digest, a stub engine config, and an arena config.
//!
//! Nothing here writes inside the repository. Match logs are artifacts
//! (CLAUDE.md rule 8) and `tools/determinism.sh` already sets the precedent
//! that a gate's transcripts live in a temporary directory.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use pistol_cli::sha256::sha256_hex;

/// The arena binary under test.
pub const ARENA: &str = env!("CARGO_BIN_EXE_arena");
/// The misbehaving instrument.
pub const STUB: &str = env!("CARGO_BIN_EXE_arena-stub-engine");

/// A scratch directory that removes itself.
pub struct Scratch {
    pub dir: PathBuf,
}

impl Scratch {
    /// A fresh directory named for the test that asked for one.
    pub fn new(name: &str) -> Scratch {
        let mut dir = std::env::temp_dir();
        dir.push(format!("pistol-arena-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Scratch { dir }
    }

    /// A path inside it.
    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    /// Write a file inside it and return the path.
    pub fn write(&self, name: &str, body: &str) -> PathBuf {
        let path = self.path(name);
        std::fs::write(&path, body).expect("writing a scratch file");
        path
    }

    /// A stub engine config naming one behaviour.
    pub fn stub_config(&self, name: &str, behave: &str) -> PathBuf {
        self.write(name, &format!("# a test instrument\nbehave {behave}\n"))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// The repository root, found from this crate's manifest.
pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/pistol-arena has two ancestors")
        .to_path_buf()
}

/// The committed openings fixture's body lines.
pub fn committed_body() -> Vec<String> {
    let path = repo().join("crates/pistol-cli/tests/fixtures/openings_v1.txt");
    let text = std::fs::read_to_string(&path).expect("the committed openings fixture");
    let marker = "# body_sha256 ";
    let mut body = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if inside {
            body.push(line.to_string());
        } else if line.starts_with(marker) {
            inside = true;
        }
    }
    assert!(!body.is_empty(), "the fixture has a body");
    body
}

/// An openings fixture over `lines`, with a header whose digest is correct.
pub fn openings_fixture(lines: &[String]) -> String {
    let body: String = lines
        .iter()
        .map(|line| format!("{line}\n"))
        .collect::<Vec<String>>()
        .join("");
    let digest = sha256_hex(body.as_bytes());
    format!("# a test fixture\n# body_sha256 {digest}\n{body}")
}

/// The first `count` openings of the committed fixture, as a fixture.
pub fn openings_prefix(count: usize) -> String {
    let body = committed_body();
    openings_fixture(&body[..count])
}

/// How an arena config is spelled. Every key, because none has a default.
pub struct ConfigSpec<'a> {
    pub openings: &'a Path,
    pub take: usize,
    pub turn_cap: u32,
    pub workers: usize,
    pub hang_ms: u64,
    pub elo1: f64,
    pub budget_kind: &'a str,
    pub budget_value: u64,
    pub binary_a: &'a str,
    pub config_a: &'a Path,
    pub binary_b: &'a str,
    pub config_b: &'a Path,
}

impl ConfigSpec<'_> {
    /// The document.
    pub fn render(&self) -> String {
        format!(
            "schema_version = 1\n\
             [run]\n\
             openings_file = \"{openings}\"\n\
             openings_take = {take}\n\
             turn_cap = {turn_cap}\n\
             n_workers = {workers}\n\
             hang_timeout_ms = {hang}\n\
             [budget]\n\
             kind = \"{kind}\"\n\
             value = {value}\n\
             [sprt]\n\
             elo0 = 0.0\n\
             elo1 = {elo1}\n\
             alpha = 0.05\n\
             beta = 0.05\n\
             [engine_a]\n\
             label = \"a\"\n\
             binary = \"{bin_a}\"\n\
             config = \"{cfg_a}\"\n\
             [engine_b]\n\
             label = \"b\"\n\
             binary = \"{bin_b}\"\n\
             config = \"{cfg_b}\"\n",
            openings = self.openings.display(),
            take = self.take,
            turn_cap = self.turn_cap,
            workers = self.workers,
            hang = self.hang_ms,
            kind = self.budget_kind,
            value = self.budget_value,
            elo1 = self.elo1,
            bin_a = self.binary_a,
            cfg_a = self.config_a.display(),
            bin_b = self.binary_b,
            cfg_b = self.config_b.display(),
        )
    }
}

/// A self-match spec against the honest stub, which every end-to-end test
/// starts from and then varies in one place.
pub fn self_match<'a>(
    openings: &'a Path,
    stub: &'a Path,
    take: usize,
    turn_cap: u32,
    workers: usize,
) -> ConfigSpec<'a> {
    ConfigSpec {
        openings,
        take,
        turn_cap,
        workers,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "depth_turns",
        budget_value: 1,
        binary_a: STUB,
        config_a: stub,
        binary_b: STUB,
        config_b: stub,
    }
}

/// Run the arena and hand back everything it produced.
pub struct Ran {
    pub output: Output,
    pub report: Option<String>,
}

impl Ran {
    /// The report, or a panic naming what the run said instead.
    pub fn report(&self) -> &str {
        self.report.as_deref().unwrap_or_else(|| {
            panic!(
                "no report was written.\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&self.output.stdout),
                String::from_utf8_lossy(&self.output.stderr)
            )
        })
    }

    /// The value of a one-value record, e.g. `verdict`.
    pub fn field(&self, key: &str) -> String {
        let prefix = format!("{key} ");
        self.report()
            .lines()
            .find_map(|line| line.strip_prefix(&prefix))
            .unwrap_or_else(|| panic!("no `{key}` record in the report:\n{}", self.report()))
            .to_string()
    }

    /// Every `game …` record.
    pub fn games(&self) -> Vec<&str> {
        self.report()
            .lines()
            .filter(|line| line.starts_with("game "))
            .collect()
    }

    /// The exit code.
    pub fn code(&self) -> i32 {
        self.output.status.code().unwrap_or(-1)
    }
}

/// Run the arena over a spec.
pub fn run(scratch: &Scratch, spec: &ConfigSpec<'_>, tag: &str) -> Ran {
    let config = scratch.write(&format!("arena-{tag}.toml"), &spec.render());
    let out = scratch.path(&format!("report-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--config")
        .arg(&config)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    Ran {
        report: std::fs::read_to_string(&out).ok(),
        output,
    }
}
