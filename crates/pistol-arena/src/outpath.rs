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
/// Exit 2 promises "no report at all", so the claim is removed rather than
/// left to masquerade as one. Two paths lead here: a refusal BEFORE any game
/// (the claim is still empty), and a report WRITE that failed partway (disk
/// full after a played run) — there the file holds a truncated non-report,
/// and a partial report left on disk would be worse than none, because
/// nothing marks it partial (REVIEW-impl, docs/decisions.md D-205). This can only
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
