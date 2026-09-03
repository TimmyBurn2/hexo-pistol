use std::io::Write as _;
use std::path::Path;
use std::process::ExitCode;

use crate::error::ArenaError;
use crate::label_cache::LabelCache;

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
///
/// `census` is the claimed census file, present exactly when the run asked for
/// the census. Claimed by the caller before any game, for the reason `--out`
/// is (docs/decisions.md D-200): a census file discovered to be unwritable
/// after sixty hours of asking is sixty hours of rows nothing can hold.
pub fn capture(
    source: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
    label_nodes: u64,
    census: Option<(std::path::PathBuf, std::fs::File)>,
    cache: LabelCache,
) -> Result<ExitCode, ArenaError> {
    let transcript = read_report(source)?;
    let request = match census {
        Some(_) => pistol_engine::CensusRequest::On,
        None => pistol_engine::CensusRequest::Off,
    };
    let go_line = crate::capture::label_go_line(label_nodes, request);
    let mut rows: Vec<String> = Vec::new();
    let (records, counts) = {
        let mut sink = crate::capture::CensusSink {
            request,
            rows: &mut rows,
        };
        crate::capture::run(&transcript, label_nodes, &mut sink, cache)?
    };
    let rendered = crate::capture_file::render(&transcript, &go_line, &records);
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;
    // EVERY BYTE OF THE RUN REACHES DISK BEFORE ANYTHING IS CLAIMED ON STDOUT.
    // A failure after this point abandons BOTH files (`bin/arena.rs`), so a
    // manifest row printed before the census write would name bytes the same
    // run then removes — and a manifest row is what rule 8 and D-469 lean on.
    let census_written = match census {
        Some((census_path, census_file)) => {
            // STREAMED, and buffered: the artifact is estimated at gigabytes,
            // and holding a rendered copy of it beside the rows it was rendered
            // from is the cost this pass exists not to pay.
            let mut out = std::io::BufWriter::new(census_file);
            crate::census_file::write_into(&mut out, &census_path, &transcript, &go_line, &rows)?;
            let row = crate::census_file::manifest_row(&transcript, &go_line, &rows, &census_path);
            Some((census_path, row))
        }
        None => None,
    };
    println!(
        "arena: captured {} position(s) from {} game(s) at {go_line}",
        records.len(),
        transcript.games.len()
    );
    println!("{}", counts.line(cache));
    println!(
        "{}",
        crate::capture_file::manifest_row(&transcript, &go_line, &rendered, out_path)?
    );
    println!("arena: capture written to {}", out_path.display());
    if let Some((census_path, row)) = census_written {
        println!("arena: captured {} census row(s)", rows.len());
        println!("{row}");
        println!("arena: census written to {}", census_path.display());
    }
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
