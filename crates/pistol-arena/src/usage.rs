use crate::label_cache::LabelCache;

/// What the `arena` binary does, and what it refuses to guess.
///
/// Extracted from the binary because a third and fourth mode arm push
/// `bin/arena.rs` past `tools/file_justification_check.sh`'s cap, and a
/// justification for a file that is mostly help text is the wrong trade
/// (`docs/experiments/wp20m_design.md` §1). `pub` rather than `pub(crate)`:
/// the binary is a separate crate from this library.
pub const USAGE: &str = "\
arena — the paired-openings SPRT judge for pistol

usage:
  arena --config <path> --out <path>
  arena --replay <report path> --out <path> --workers <n>
  arena --capture <report path> --out <path> --label-nodes <n> [--census | --label-cache]
  arena --labels <capture path> --report <report path> --out <path>

  --config  an arena config. Always explicit: there is no default path and no
            built-in configuration (CLAUDE.md rule 1). It states the openings,
            the budget, the turn cap, the worker count and the SPRT bounds.
  --out     where to write the report. CLAIMED exclusively at dispatch
            (create_new/O_EXCL), so an existing file — a previous report, or
            another run in flight — is refused by name before any game: a run
            that silently overwrote a report would destroy the evidence for a
            claim somebody has already made. A refusal before any game removes
            the empty claim again. Match logs are artifacts and are never
            written inside the repository (CLAUDE.md rule 8).

  --replay  a report THIS program wrote. Its games are re-driven warm through
            the engines it attests — every seat spawned, every recorded move
            fed, every turn an engine searched asked again at the run's own
            budget — and the first turn of each game where an answer disagrees
            with the record is reported. Only a `nodes` budget replays, by name:
            the premise is that a re-driven engine answers what it answered, and
            wall-clock does not promise that (CLAUDE.md rule 4). Refuses a
            report whose engines are no longer the ones it attests, before any
            game. The flags are in this order; there is no other spelling.
  --workers how many games are replayed at once, on the command line because
            there is no config document here to state it and no code-side
            default for a tunable (CLAUDE.md rule 1). The pass replays EVERY
            game of the report with no early stop, so what it finds does not
            depend on this number.

  --capture a report THIS program wrote, walked position by position with the
            engine asked again at the LABEL budget. One `newgame` precedes every
            ask, so no label is produced on a table another ask warmed. The
            source report must carry a `nodes` budget — every other kind is
            refused when the report is read — and its two seats must attest the
            SAME engine, which a self-play report's do and an A-versus-B
            report's do not. Writes one record per asked position, verbatim but
            for the two wall-clock fields, and prints a manifest row.
  --label-nodes
            the node budget every label ask is made at, spelled the way this
            program will echo it back. It is the only budget this mode takes:
            there is no wall-clock spelling to refuse, because a wall-clock
            label would be a fact about the machine.
  --census  ask each label at the engine's census token and keep the trigger
            rows it answers with, in a second file named beside --out. LAST,
            and optional: a capture without it is the capture this pipeline has
            always taken. It changes the `go` line and therefore the capture's
            own identity digest, so a census-on capture is a DIFFERENT
            instrument from the otherwise identical census-off one. An engine
            that cannot serve a census refuses the token by name, and the run
            is refused with it — a capture that quietly wrote no rows would be
            indistinguishable from one whose engine never fired a trigger.
  --label-cache
            memoise, within this run, the answer to a `position` line this run
            has already asked, so an identical prefix is not searched twice.
            LAST, and optional: absent means OFF, which is the capture this
            pipeline has always taken. A hit returns the exact bytes the first
            ask produced, so the written capture is byte-identical either way
            and carries no trace of the mode; the one record that a run was
            cached is the counts line this mode prints beside the manifest row
            — `arena: label cache on: asks A records R hits H
            key_pos_collisions P key_full_collisions F fold_ms M`, or `arena:
            label cache off: asks A records R` — where `asks` counts calls to
            the engine, `hits` is `records - asks`, and the two collision
            counters say how many misses a coarser key would have merged.
            Refused with --census, by name: a hit performs no search and emits
            no census row.

  --labels  a capture THIS program wrote, turned into the training corpus. Reads
            no engine and spawns nothing: it is a pure function of the capture
            and the report that capture came from, so a disagreement about what
            a column MEANS costs a re-run of this mode rather than a re-run of
            the engine. What each column means is fixed by keyed params on the
            corpus's own face, because units that live only in a design document
            are lost on the first copy.
  --report  the report the capture was taken from. Its digest must be the one
            the capture's header names, and the capture's identity must be the
            one its own inputs produce, or the run is refused before a record is
            read.

  Only instrument budgets are accepted. A `movetime` budget is refused by name:
  wall-clock is not reproducible, and it is not even a ceiling — the first
  deepening iteration cannot be interrupted (docs/decisions.md D-74, D-95).

  The verdict is read off the PAIR-level LLR. The game-level LLR is reported
  beside it as a diagnostic and is not the verdict (docs/decisions.md D-154).

exit: 0 completed cleanly, 1 abandoned or forfeited (report still written),
      2 a document this build refuses (no report).
";

/// What a capture line may end with.
///
/// # Errors The two words together are refused BY
/// NAME, in either order, before any file is claimed; a word twice or out of
/// place is the usage refusal, which names neither.
pub fn capture_tail(tail: &[&str]) -> Result<(bool, LabelCache), String> {
    match tail {
        [] => Ok((false, LabelCache::Off)),
        ["--census"] => Ok((true, LabelCache::Off)),
        ["--label-cache"] => Ok((false, LabelCache::On)),
        ["--census", "--label-cache"] | ["--label-cache", "--census"] => Err(String::from(
            "--label-cache with --census is refused: a cache hit performs no search and emits \
             no census row, so a cached census capture would under-report firings at exit 0",
        )),
        _ => Err(usage_error()),
    }
}

pub fn usage_error() -> String {
    format!(
        "--config and --out are both required, or --replay, --out and --workers, or --capture, \
         --out and --label-nodes, or --labels, --report and --out, each in that order\n\n{USAGE}"
    )
}

/// The worker count, with its SPELLING validated and not merely its value.
///
/// # Errors
/// `+4`, ` 4` and `04` all parse to four and would land in a document's timing
/// block unnormalised, describing a run nobody can reproduce by copying the line
/// back (tools/SHELL_CHECKLIST.md item 8).
pub fn workers_of(word: &str) -> Result<usize, String> {
    let parsed = usize::try_from(count_of(word, "worker count")?)
        .map_err(|_| format!("`{word}` is more workers than this machine can address"))?;
    if parsed == 0 {
        return Err(String::from("--workers 0 would replay nothing at all"));
    }
    Ok(parsed)
}

/// A count off the command line, with its SPELLING validated and not merely its
/// value.
///
/// # Errors
/// `+4`, ` 4` and `04` all parse to four and would land in a document
/// describing a run nobody can reproduce by copying the line back
/// (tools/SHELL_CHECKLIST.md item 8).
pub fn count_of(word: &str, what: &str) -> Result<u64, String> {
    let parsed: u64 = word
        .parse()
        .map_err(|_| format!("`{word}` is not a {what}\n\n{USAGE}"))?;
    if parsed.to_string() != word {
        return Err(format!(
            "`{word}` is a {what} spelled a way this program will not echo back; write it as \
             `{parsed}`\n\n{USAGE}"
        ));
    }
    if parsed == 0 {
        return Err(format!("a {what} of zero asks for nothing at all"));
    }
    Ok(parsed)
}
