mod common;

use common::{Scratch, openings_prefix, run, self_match};

/// A cap that leaves room for a real game past the four-turn openings.
const TURN_CAP: u32 = 10;

#[test]
fn arena_abandons_a_run_when_an_engine_closes_its_pipe_and_keeps_running() {
    // REVIEW-impl's blocker, pinned. A closed pipe and an exited process are
    // TWO events: the reader thread ends at EOF on stdout, which a child can
    // produce while continuing to run. Reaping such a child with an unbounded
    // `wait()` blocks where the watchdog cannot see it — control has already
    // left the receive — so the arena hung forever and wrote no report at all,
    // defeating both the liveness device and the promise that an abandoned run
    // keeps its evidence (docs/decisions.md D-159, D-160).
    //
    // The peer is a shell script rather than a `--behave` mode because closing
    // this process's own stdout is not something safe Rust can do, and this
    // workspace denies `unsafe` (Cargo.toml). It speaks just enough protocol to
    // get to the first `go`.
    let scratch = Scratch::new("mute");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    let mute = scratch.write(
        "mute-engine.sh",
        "#!/usr/bin/env bash\n\
         # A test instrument: handshakes, then closes stdout and does not exit.\n\
         while IFS= read -r line; do\n\
           case \"$line\" in\n\
             pistol*) printf 'id name mute\\nid protocol v0\\nid mode instrument\\nid weights_sha256 0000000000000000000000000000000000000000000000000000000000000000\\npistolok\\n' ;;\n\
             go*) exec 1>&-; while true; do sleep 3600; done ;;\n\
             quit*) exit 0 ;;\n\
             *) : ;;\n\
           esac\n\
         done\n",
    );
    std::fs::set_permissions(&mute, std::os::unix::fs::PermissionsExt::from_mode(0o755))
        .expect("the script is executable");

    let script = mute.display().to_string();
    let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
    spec.binary_b = &script;
    spec.hang_ms = 500;

    let started = std::time::Instant::now();
    let ran = run(&scratch, &spec, "mute");
    let elapsed = started.elapsed();

    // The bound is what was missing. Ten seconds is enormous against a 500 ms
    // watchdog and still fails outright against an unbounded wait.
    assert!(
        elapsed < std::time::Duration::from_secs(10),
        "the run took {elapsed:?}; the watchdog must bound reaping a child that closed its \
         pipe and kept running"
    );
    assert_eq!(ran.code(), 1, "an abandoned run does not exit clean");
    let report = ran.report();
    assert!(
        report.starts_with("arena_report_aborted "),
        "the evidence is kept, in a report of the kind that carries no verdict:\n{report}"
    );
    assert!(
        !report.lines().any(|line| line.starts_with("verdict ")),
        "and it carries no verdict:\n{report}"
    );
    assert!(
        report
            .lines()
            .any(|line| line.starts_with("aborted Killed")),
        "a child that stops answering and will not exit is not an answer this arena can \
         adjudicate, so it abandons the run rather than forfeiting a game:\n{report}"
    );
}

/// Write an executable shell-script engine into the scratch directory.
///
/// A script rather than a `--behave` mode where the misbehaviour is something
/// safe Rust cannot do to its own file descriptors, or where writing a megabyte
/// without a newline is clearer as three words of shell than as a Rust loop.
fn script_engine(scratch: &Scratch, name: &str, body: &str) -> String {
    let path = scratch.write(name, body);
    std::fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o755))
        .expect("the script is executable");
    path.display().to_string()
}

/// The handshake every script engine below answers with. The `weights_sha256`
/// value is a fixed well-formed token, not a digest of anything: the arena
/// requires the FIELD of every engine (docs/decisions.md D-198), and what these
/// instruments misbehave about is elsewhere.
const SCRIPT_HANDSHAKE: &str = "printf 'id name script\\nid protocol v0\\nid mode instrument\\nid \
                                weights_sha256 0000000000000000000000000000000000000000000000000000000000000000\\npistolok\\n'";

#[test]
fn arena_refuses_a_self_contradictory_or_non_canonical_weights_identity() {
    // RED-TEAM found both leniencies against 55aa5e6 (docs/decisions.md
    // D-205). Two `weights_sha256` lines naming DIFFERENT tables is a
    // self-contradictory identity — `Identity::field` silently took the first,
    // so a report could carry two digests with no complaint, in the exact
    // property D-198 exists to make unambiguous. And uppercase hex is
    // non-canonical: no pistol build emits it, and two runs of the SAME
    // weights differing only in case would compute different
    // `experiment_sha256` and read as different experiments.
    let scratch = Scratch::new("weights-identity");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    let zeros = "0".repeat(64);
    let cases = [
        (
            "doubled-weights.sh",
            format!(
                "id name script\\nid protocol v0\\nid mode instrument\\nid weights_sha256 \
                 {zeros}\\nid weights_sha256 {}\\npistolok\\n",
                "1".repeat(64)
            ),
            "more than once",
        ),
        (
            "uppercase-weights.sh",
            format!(
                "id name script\\nid protocol v0\\nid mode instrument\\nid weights_sha256 \
                 {}\\npistolok\\n",
                "A".repeat(64)
            ),
            "lowercase",
        ),
    ];
    for (name, handshake, expected) in cases {
        let script = script_engine(
            &scratch,
            name,
            &format!(
                "#!/usr/bin/env bash\n\
                 while IFS= read -r line; do\n\
                   case \"$line\" in\n\
                     pistol*) printf '{handshake}' ;;\n\
                     quit*) exit 0 ;;\n\
                     *) : ;;\n\
                   esac\n\
                 done\n"
            ),
        );
        let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
        spec.binary_b = &script;
        let ran = run(&scratch, &spec, name);
        assert_eq!(
            ran.code(),
            2,
            "{name} is refused before any game.\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&ran.output.stdout),
            String::from_utf8_lossy(&ran.output.stderr)
        );
        assert!(
            ran.report.is_none(),
            "{name}: nothing is written, because there is no run to report on"
        );
        let stderr = String::from_utf8_lossy(&ran.output.stderr);
        assert!(
            stderr.contains("weights_sha256") && stderr.contains(expected),
            "{name}'s refusal names the field and the rule: {stderr}"
        );
    }
}

#[test]
fn arena_forfeits_engine_that_answers_when_nothing_was_asked() {
    // RED-TEAM's worst finding, pinned. The channel beneath the referee is a
    // plain queue with no request identifier — the protocol has none to offer —
    // so a line an engine volunteers is not noise: it is read as the answer to
    // the NEXT `go`, from a position the engine was never shown. The observed
    // consequence was a game whose move list contained turns neither engine
    // intended, recorded as an ordinary clean win and counted into the SPRT
    // sample with nothing anywhere to say so (docs/decisions.md D-172).
    let scratch = Scratch::new("out-of-turn");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    // Two `bestmove` lines for one `go`. The second is a legal-looking pair far
    // from the cluster, so a desynchronized referee would happily play it.
    let doubled = script_engine(
        &scratch,
        "doubled.sh",
        &format!(
            "#!/usr/bin/env bash\n\
             first=1\n\
             while IFS= read -r line; do\n\
               case \"$line\" in\n\
                 pistol*) {SCRIPT_HANDSHAKE} ;;\n\
                 go*)\n\
                   if [ \"$first\" = 1 ]; then first=0\n\
                     printf 'info totals depth_turns 1 seldepth 1 nodes 1 nps 1 time 0 hashfull 0 score cp 0 pv 5,0/6,0\\nbestmove 5,0/6,0\\nbestmove 8,0/9,0\\n'\n\
                   else\n\
                     printf 'info totals depth_turns 1 seldepth 1 nodes 1 nps 1 time 0 hashfull 0 score cp 0 pv 20,0/21,0\\nbestmove 20,0/21,0\\n'\n\
                   fi ;;\n\
                 quit*) exit 0 ;;\n\
               esac\n\
             done\n"
        ),
    );
    let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
    spec.binary_b = &doubled;
    let ran = run(&scratch, &spec, "out-of-turn");

    assert_eq!(
        ran.code(),
        1,
        "an out-of-turn line forfeits, so the run is not clean"
    );
    let report = ran.report();
    assert!(
        report.contains("reason protocol_error") && report.contains("forfeit_by b"),
        "the offender and the reason are named:\n{report}"
    );
    assert!(
        report.contains("when nothing had been asked of it"),
        "and the refusal says what the violation was:\n{report}"
    );
    // The load-bearing assertion: the volunteered move is NOT in any move list.
    for moves in report.lines().filter(|line| line.starts_with("moves ")) {
        assert!(
            !moves.contains("8,0/9,0"),
            "the stale line must never be replayed as a move: {moves}"
        );
    }
}

#[test]
fn arena_forfeits_engine_that_writes_a_line_longer_than_it_will_read() {
    // RED-TEAM's second finding. The read cap must refuse an over-long line, not
    // hand the truncated chunk over as though it were one: an engine streaming
    // endless bytes that happen to begin `info ` was read as endless legitimate
    // progress, so the watchdog that fires on silence never got the chance to
    // and a run could be occupied indefinitely (docs/decisions.md D-172).
    let scratch = Scratch::new("overlong");
    let openings = scratch.write("openings.txt", &openings_prefix(1));
    let honest_config = scratch.stub_config("honest.toml", "honest");
    let flood = script_engine(
        &scratch,
        "flood.sh",
        &format!(
            "#!/usr/bin/env bash\n\
             while IFS= read -r line; do\n\
               case \"$line\" in\n\
                 pistol*) {SCRIPT_HANDSHAKE} ;;\n\
                 go*) while true; do printf 'info '; head -c 1048571 /dev/zero | tr '\\0' x; done ;;\n\
                 quit*) exit 0 ;;\n\
               esac\n\
             done\n"
        ),
    );
    let mut spec = self_match(&openings, &honest_config, 1, TURN_CAP, 1);
    spec.binary_b = &flood;
    // Deliberately short. The point is that the run ends on the PROTOCOL
    // violation and not by outlasting the flood, so the bound below is what
    // would fail against a referee that treated a truncated chunk as a line.
    spec.hang_ms = 2_000;

    let started = std::time::Instant::now();
    let ran = run(&scratch, &spec, "overlong");
    let elapsed = started.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "the run took {elapsed:?}; an engine that never writes a newline must not be able to \
         occupy the arena"
    );
    assert_eq!(ran.code(), 1);
    assert!(
        ran.report().contains("with no newline"),
        "the refusal names what was wrong with it:\n{}",
        ran.report()
    );
    assert!(
        ran.report().contains("reason protocol_error"),
        "an over-long line is a deterministic protocol violation, so it forfeits rather than \
         abandoning the run:\n{}",
        ran.report()
    );
}
