//! Who each engine IS, by content — captured once before the first game and
//! re-verified at every spawn.
//!
//! # Why identity closes over the eval weights
//!
//! A path is not identity: `target/release/pistol` is a different program after
//! every build (docs/decisions.md D-147), so the run digests the binary and the
//! config document. What that still missed was the eval weight table: two
//! workspaces differing only in `eval_v0_weights.toml` produced byte-identical
//! `experiment_sha256` and every other recorded digest while `nelo_pair` moved
//! by 98 points (docs/decisions.md D-188, wp13_results §6b). The engine now
//! digests the weights file it loads and reports `id weights_sha256 <hex>` in
//! its handshake; the arena REQUIRES that field and folds it into the identity.
//! The ENGINE digests rather than the arena because the engine is the process
//! that actually reads the file, resolved against its own working directory —
//! an arena-side digest could attest bytes the engine never loaded, and would
//! couple this crate to the engine's config schema, which it deliberately does
//! not read (docs/decisions.md D-198).
//!
//! # Why identity is re-verified at every spawn
//!
//! Engines are respawned from disk for every game, while digests used to be
//! taken once: a config edited eighteen seconds into a live run produced exit 0
//! and a report attesting the OLD config (docs/decisions.md D-188's operating
//! rule). That rule is now code: every spawn re-digests the config document and
//! re-states the whole handshake (which is deterministic by construction), and
//! any drift from the run-start capture is the named abort
//! [`crate::error::ArenaError::IdentityDrift`] — never a game result, never a
//! silent continuation (docs/decisions.md D-199).
//!
//! Two windows stay open and are recorded rather than implied: the binary is
//! not re-digested per spawn (re-digesting a multi-megabyte file per game buys
//! a case nobody has observed), and a swap landing between a spawn's own
//! verification and that engine's config read — on the LAST game, with no later
//! spawn to catch it — is invisible to any check made from outside the process.

use std::path::Path;

use crate::channel::Channel;
use crate::config::EngineSection;
use crate::error::ArenaError;
use crate::handshake::{self, Identity};

/// The handshake field naming the eval weight table by content.
pub const WEIGHTS_FIELD: &str = "weights_sha256";

/// The identity of one engine, gathered before the first game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineIdentity {
    /// The handshake's `id` lines, verbatim, in the order given.
    pub id_lines: Vec<String>,
    /// The digest of the binary that was actually run.
    pub binary_sha256: String,
    /// The digest of the config it was run with.
    pub config_sha256: String,
    /// The digest of the eval weight table, as the engine itself reported it.
    pub weights_sha256: String,
}

/// The SHA-256 of a file this run depends on.
pub fn digest_of(path: &Path) -> Result<String, ArenaError> {
    let bytes = std::fs::read(path)
        .map_err(|io| ArenaError::io(format!("reading {}", path.display()), io))?;
    Ok(pistol_cli::sha256::sha256_hex(&bytes))
}

/// One engine's identity: what it says about itself, and what it is by content.
///
/// The engine is started once here, before any game, purely to shake hands.
/// That costs one process per side and buys the run's instrument (CLAUDE.md
/// rule 6): the `id` lines carry the candidate radius, the table size and the
/// weights digest, and a log that cannot recover those cannot be re-run. It
/// also fails the run early on an engine a strength claim may not come from,
/// rather than on the first game.
pub fn capture(engine: &EngineSection, timeout_ms: u64) -> Result<EngineIdentity, ArenaError> {
    // THE BINARY IS BOUND BY CONTENT AND THE CHECK COMES FIRST, before the
    // process starts. A path is not an identity — the same path is a different
    // program after every build — and the stale-binary case is the one that
    // exits 0: a decoy sitting where cargo did not write is a regular file, is
    // executable, speaks the protocol, and plays every game (docs/decisions.md
    // D-252's reproducer). Refusing here rather than at validation keeps
    // validation pure and offline (D-21), and refusing before the spawn means a
    // run this document does not describe never produces a game.
    let binary_sha256 = digest_of(&engine.binary)?;
    if binary_sha256 != engine.binary_sha256 {
        return Err(ArenaError::EngineBinaryDigestMismatch {
            engine: engine.label.clone(),
            binary: engine.binary.display().to_string(),
            expected: engine.binary_sha256.clone(),
            found: binary_sha256,
        });
    }
    let mut channel = Channel::start(&engine.label, &engine.binary, &engine.config)?;
    let spoken = handshake::shake(&mut channel, timeout_ms)?;
    channel.shutdown();
    let weights = spoken
        .field(WEIGHTS_FIELD)
        .unwrap_or_else(|| unreachable!("shake refused a handshake without {WEIGHTS_FIELD}"))
        .to_string();
    Ok(EngineIdentity {
        id_lines: spoken.lines,
        binary_sha256,
        config_sha256: digest_of(&engine.config)?,
        weights_sha256: weights,
    })
}

/// Refuse a spawn whose engine is no longer the one the run started with.
///
/// Two comparisons, each against the run-start capture: the config document's
/// bytes (digested here, from outside the process), and the ENTIRE id-line
/// vector the fresh handshake answered with — which subsumes the weights digest,
/// since that is one of the lines, and additionally catches a config swapped in
/// the run-start window between the first engine's config read and its digest
/// (the id lines attest values from the bytes each process actually read). The
/// handshake is deterministic by construction — nothing in it is measured or
/// timed — so on an honest run this can never fire.
pub fn verify_respawn(
    engine: &EngineSection,
    expected: &EngineIdentity,
    spoken: &Identity,
) -> Result<(), ArenaError> {
    let found = digest_of(&engine.config)?;
    if found != expected.config_sha256 {
        return Err(ArenaError::IdentityDrift {
            engine: engine.label.clone(),
            what: format!("config document `{}`", engine.config.display()),
            expected: expected.config_sha256.clone(),
            found,
        });
    }
    if spoken.lines != expected.id_lines {
        let (expected_line, found_line) = first_divergence(&expected.id_lines, &spoken.lines);
        return Err(ArenaError::IdentityDrift {
            engine: engine.label.clone(),
            what: String::from("handshake identity"),
            expected: expected_line,
            found: found_line,
        });
    }
    Ok(())
}

/// The first id line the two handshakes disagree on, for a refusal that names
/// what moved rather than dumping two vectors.
fn first_divergence(expected: &[String], found: &[String]) -> (String, String) {
    let absent = || String::from("<no such line>");
    for at in 0..expected.len().max(found.len()) {
        let a = expected.get(at);
        let b = found.get(at);
        if a != b {
            return (
                a.cloned().unwrap_or_else(absent),
                b.cloned().unwrap_or_else(absent),
            );
        }
    }
    unreachable!("called on two equal id-line vectors")
}
