//! Claiming the report path, atomically.
//!
//! `--out` used to be a TOCTOU: an existence check at dispatch and a plain
//! write at the end, so two runs racing past the check both "succeeded" and
//! one silently destroyed the other's report — the evidence for a claim
//! somebody may already have made (wp13_results §6b). The claim is now the
//! check: the file is created with `create_new` (O_EXCL) at dispatch, the
//! report is written through the claimed handle, and a second run asking for
//! the same path fails by name at ITS dispatch, before it has spent anything
//! (docs/decisions.md D-200).
//!
//! Two consequences, stated rather than implied: while a run is live its
//! `--out` file exists and is empty, which is what makes the claim visible;
//! and a run that crashes leaves that empty claim behind, blocking a rerun at
//! the same path until an operator removes it — loud, and evidence that a run
//! died, which is the correct default for a file whose whole job is evidence.

use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::error::ArenaError;

/// Claim `path` exclusively, creating it empty.
///
/// The creation and the existence check are ONE syscall (`O_EXCL`), so there
/// is no window in which two runs can both believe they own the path.
pub fn claim(path: &Path) -> Result<File, ArenaError> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|io| {
            ArenaError::io(
                format!(
                    "claiming {} — a run does not overwrite a previous report, and two runs may \
                     not share an --out path; if no run is live, remove the file",
                    path.display()
                ),
                io,
            )
        })
}

/// Release a claim that will never hold a report.
///
/// A refusal BEFORE any game exits 2 and promises "no report at all", so the
/// empty claim is removed rather than left to masquerade as one. This can only
/// ever remove a file this process created via [`claim`]: a pre-existing file
/// fails `claim` before anything is written, and a concurrent run's own
/// `create_new` fails while the claim stands — so no path through here
/// destroys another run's evidence. The residual is a loud false refusal: a
/// run dispatched in the window between this process's claim and this removal
/// is refused although the path is free milliseconds later (docs/decisions.md
/// D-200).
pub fn abandon(path: &Path) -> Result<(), ArenaError> {
    std::fs::remove_file(path)
        .map_err(|io| ArenaError::io(format!("removing the claimed {}", path.display()), io))
}
