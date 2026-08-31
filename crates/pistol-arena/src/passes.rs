use std::io::Write as _;
use std::path::Path;
use std::process::ExitCode;

use crate::error::ArenaError;

/// One report, read back as the run it describes.
///
/// A REGULAR FILE, CHECKED BEFORE IT IS READ. `fs::read` on a FIFO blocks until
/// a writer appears, with no channel yet in existence and so no watchdog to end
/// it — a hang where a refusal belongs (docs/decisions.md D-252's sibling case
/// in `identity::digest_of`).
pub fn read_report(source: &Path) -> Result<crate::transcript::Transcript, ArenaError> {
    let meta = std::fs::metadata(source)
        .map_err(|io| ArenaError::io(format!("reading {}", source.display()), io))?;
    if !meta.is_file() {
        return Err(ArenaError::config(
            "replay report",
            format!("{} is not a regular file", source.display()),
        ));
    }
    let bytes = std::fs::read(source)
        .map_err(|io| ArenaError::io(format!("reading {}", source.display()), io))?;
    let source_sha256 = pistol_cli::sha256::sha256_hex(&bytes);
    let text = std::str::from_utf8(&bytes).map_err(|why| {
        ArenaError::config(
            "replay report",
            format!("{} is not UTF-8: {why}", source.display()),
        )
    })?;
    crate::transcript::read(text, source_sha256)
}

/// Walk one report position by position, asking each at the label budget.
pub fn capture(
    source: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
    label_nodes: u64,
) -> Result<ExitCode, ArenaError> {
    let transcript = read_report(source)?;
    let go_line = crate::capture::label_go_line(label_nodes);
    let records = crate::capture::run(&transcript, label_nodes)?;
    let rendered = crate::capture_file::render(&transcript, &go_line, &records);
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;
    println!(
        "arena: captured {} position(s) from {} game(s) at {go_line}",
        records.len(),
        transcript.games.len()
    );
    println!(
        "{}",
        crate::capture_file::manifest_row(&transcript, &go_line, &rendered, out_path)?
    );
    println!("arena: capture written to {}", out_path.display());
    Ok(ExitCode::SUCCESS)
}

/// Turn one capture into the training corpus: a pure file transform.
pub fn labels(
    capture_path: &Path,
    report: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
) -> Result<ExitCode, ArenaError> {
    let transcript = read_report(report)?;
    let text = std::fs::read_to_string(capture_path)
        .map_err(|io| ArenaError::io(format!("reading {}", capture_path.display()), io))?;
    let capture = crate::capture_file::read(&text)?;
    let records = crate::labels::run(&capture, &transcript)?;
    let rendered = crate::labels_file::render(&capture, &transcript, &records);
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;
    println!("arena: labelled {} position(s)", records.len());
    println!(
        "{}",
        crate::labels_file::manifest_row(&capture, &transcript, &rendered, out_path)?
    );
    println!("arena: corpus written to {}", out_path.display());
    Ok(ExitCode::SUCCESS)
}
