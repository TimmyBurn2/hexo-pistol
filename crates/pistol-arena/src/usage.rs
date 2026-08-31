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
  arena --capture <report path> --out <path> --label-nodes <n>

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

  Only instrument budgets are accepted. A `movetime` budget is refused by name:
  wall-clock is not reproducible, and it is not even a ceiling — the first
  deepening iteration cannot be interrupted (docs/decisions.md D-74, D-95).

  The verdict is read off the PAIR-level LLR. The game-level LLR is reported
  beside it as a diagnostic and is not the verdict (docs/decisions.md D-154).

exit: 0 completed cleanly, 1 abandoned or forfeited (report still written),
      2 a document this build refuses (no report).
";
