# WP-1.8c design-phase instruments — the command blocks, verbatim

Every number in `docs/experiments/wp18c_design.md` §2, §4b and §4c came from
one of the blocks below, run in this worktree
(`/home/tom/Projects/HeXO-AlphaBeta-wp18c`, branch `wp18c`) on `omarchy`.
These are DESIGN-PHASE instruments, not registered gates: they inform the
design, and the rule-5 bench in §6 is what adjudicates. They are recorded here
so the design's instrument register can name a revision for them.

Scratch build directories are outside the tree, so nothing here writes into
the worktree's own `target/`:

```
BASE=/home/tom/.cache/wp18c-target      # the COMMITTED sources
LEGS=/home/tom/.cache/wp18c-target2     # the four legs of design §3
TTFIX=/home/tom/.cache/wp18c-target3    # the four legs + design §4c option B
```

## 1. The ANCHOR fixture set (85 positions)

Regenerated from D-438's own transcripts by the WP-1.8b extractor, unchanged:

```
python3 tools/wp18b_probe_extract.py /home/tom/.cache/wp18c-probe \
  ../HeXO-AlphaBeta/artifacts/sealbot_anchor_v1/g001.jsonl \
  ../HeXO-AlphaBeta/artifacts/sealbot_anchor_v1/g002.jsonl
```

## 2. The CORPUS and TRIGGER-RICH fixture sets

Each committed bench fixture's `start moves` tail becomes one solver-fixture
case; the `expect` is a placeholder the cost instrument never reads (it
asserts no expectation, exactly as `wp18b_probe.rs` does not).

```
python3 - <<'PY'
import pathlib
SETS = [('crates/pistol-cli/tests/fixtures/bench_positions_v1.txt',
         '/home/tom/.cache/wp18c-corpus', 'corpus'),
        ('crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt',
         '/home/tom/.cache/wp18c-trigger', 'trig')]
for src, out, stem in SETS:
    out = pathlib.Path(out); out.mkdir(parents=True, exist_ok=True)
    n = 0
    for line in pathlib.Path(src).read_text().splitlines():
        if not line.startswith('start moves'):
            continue
        body = line[len('start moves'):].split(' #')[0]
        cells = [c for tok in body.split() for c in tok.split('/')]
        n += 1
        name = '%s-%02d' % (stem, n)
        (out / (name + '.txt')).write_text(
            'case %s\nplies %s\nexpect nowin\n' % (name, ' '.join(cells)))
    print(src, '->', n)
PY
```

## 3. The cost instrument

**The paths in this section are the LADDER revisions' and are left as they
were**: at T0-T4 the instrument was an EXAMPLE, `examples/wp18c_cost.rs`, and
rewriting the commands to the shipped path would stop them reproducing the
revisions they govern. The IMPL ships the same program as a binary,
`crates/pistol-solver/src/bin/solver-cost.rs`, with a driving test and a
`<fixture> <config> <cap>` argument list instead of a fixture DIRECTORY —
which is the one behavioural difference: the ladder walked a directory of
one-case files, the shipped binary takes a fixture.

`crates/pistol-solver/examples/wp18c_cost.rs`, plus the `Instant` timers in
`dfpn.rs` and `policy.rs` that print the `ATTRIB`, `BP` and `TP` lines. Built
per side:

```
CARGO_TARGET_DIR=$LEGS cargo build --release --locked \
  -p pistol-solver --example wp18c_cost
$LEGS/release/examples/wp18c_cost <fixture-dir> configs/solver_v0.toml <cap>
```

The four §2.3 endpoints were taken SERIALLY and with nothing else running,
by `/home/tom/.cache/wp18c-clean.sh`:

```
$BASE/release/examples/wp18c_cost /home/tom/.cache/wp18c-corpus configs/solver_v0.toml 4096
$LEGS/release/examples/wp18c_cost /home/tom/.cache/wp18c-corpus configs/solver_v0.toml 4096
$BASE/release/examples/wp18c_cost /home/tom/.cache/wp18c-probe  configs/solver_v0.toml 4096
$LEGS/release/examples/wp18c_cost /home/tom/.cache/wp18c-probe  configs/solver_v0.toml 4096
```

Node-identity between the two sides of each pair is checked, not assumed:

```
join -j1 <(grep '^case' A | awk '{print $2,$4,$6,$8}' | LC_ALL=C sort) \
         <(grep '^case' B | awk '{print $2,$4,$6,$8}' | LC_ALL=C sort) \
  | awk '$2!=$5||$3!=$6||$4!=$7'
```

An empty result is the receipt; a non-empty one is a value change and the leg
that caused it is not output-identical.

## 4. The cap ladder

`/home/tom/.cache/wp18c-ladder.sh`, which for each set and each cap in
`{128, 256, 512, 1024, 2048, 4096, 16384}` records the verdict counts, the
`TOTAL` line, and one `PROOF` row per winning case. It REFUSES rather than
records when the run produced no `TOTAL` line, or when the verdict counts do
not sum to the case count — the EXIT-0-WRONG-ANSWER shape
`tools/SHELL_CHECKLIST.md` exists for, guarded here even though this is not a
`tools/` script.

## 5. The §4c reproducer

The one that matters is the smallest: the same position, the same cap, the
same config, run alone and run in company, on BOTH binaries.

```
mkdir -p /home/tom/.cache/wp18c-one3
cp /home/tom/.cache/wp18c-probe/g002-t39-p1.txt /home/tom/.cache/wp18c-one3/
$BASE/release/examples/wp18c_cost /home/tom/.cache/wp18c-one3 configs/solver_v0.toml 4096
$LEGS/release/examples/wp18c_cost /home/tom/.cache/wp18c-one3 configs/solver_v0.toml 4096
grep g002-t39 artifacts/wp18c_cost_anchor_committed_v1.txt \
               artifacts/wp18c_cost_anchor_fastpath_v1.txt
```

## 6. The `perf` profile

Mechanism evidence for §2.2, not a share the design quotes as its own:

```
perf record -g --call-graph=dwarf -F 199 -o /home/tom/.cache/wp18c-perf.data -- \
  $BASE/release/examples/wp18c_cost /home/tom/.cache/wp18c-one configs/solver_v0.toml 1024
perf report -i /home/tom/.cache/wp18c-perf.data --no-children --percent-limit 0.8 --stdio
```

## Receipts on disk

| file | what it is |
| --- | --- |
| `artifacts/wp18c_cost_corpus_committed_v1.txt` | CORPUS, committed sources, cap 4096 |
| `artifacts/wp18c_cost_corpus_fastpath_v1.txt` | CORPUS, four legs, cap 4096 |
| `artifacts/wp18c_cost_anchor_committed_v1.txt` | ANCHOR, committed sources, cap 4096 |
| `artifacts/wp18c_cost_anchor_fastpath_v1.txt` | ANCHOR, four legs, cap 4096 |
| `artifacts/wp18c_cap_ladder_committedTT_v1.txt` | the §4b ladder, committed replacement law |
| `artifacts/wp18c_ttlaw_anchor_v1.txt` | ANCHOR at cap 4096 under §4c option B |

## 7. The ON/OFF verification driver (design §6b)

One binary, two seats that differ only in `[solver] on_search_path`, the
committed corpus fixture, the instrument budget. Written independently of the
DECISION-RED-TEAM's own driver so §6b's number rests on two implementations and
not on one.

```
BIN=/home/tom/.cache/wp18c-target2/release/pistol
FIX=crates/pistol-cli/tests/fixtures/bench_positions_v1.txt
while IFS= read -r line; do
  case "$line" in start\ moves*) ;; *) continue ;; esac
  tail="${line#start moves }"; tail="${tail%% #*}"
  printf 'pistol\nnewgame\nposition start moves %s\ngo nodes 50000\nquit\n' "$tail" \
    | timeout 900 "$BIN" --config <SEAT> | grep 'info totals'
done < "$FIX"
```

A position that prints no `info totals` line is recorded VOID and not counted —
the EXIT-0-WRONG-ANSWER shape `tools/SHELL_CHECKLIST.md` names, guarded here
even though this is not a `tools/` script. The aggregate is
`sum(nodes) / sum(time)`, and the per-position lines are kept so the aggregate
can be re-derived from them.

## 8. The ON-seat cap sweep (design §6b)

The same driver with `per_call_node_cap` rewritten per run:

```
for cap in 32 128 512 2048; do
  sed "s/^per_call_node_cap = .*/per_call_node_cap = $cap/" <ON SEAT> > "$cfg"
  grep -q "per_call_node_cap = $cap" "$cfg" || { echo "cap $cap CONFIG EDIT FAILED"; continue; }
  ... the loop above ...
done
```

The `grep -q` after the `sed` is the check that the seat the run measures is
the seat the run says it measures: a `sed` that matched nothing exits 0 and
leaves the previous cap in place, which would report four runs of one seat as a
sweep.

## 9. The bench (design §6)

Seats: `configs/bench_wp18c_solver_on.toml` and
`configs/bench_wp18c_solver_off.toml` — complete, schema-validated, and
differing only in `[solver] on_search_path`. Fixtures:
`crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` (both bands reported
separately) and `bench_solver_positions_v1.txt` (repaired and pinned at
WP-1.8c, design §4e). Budget `go nodes 50000`. Driver: §7's loop, one seat at a
time, reps and the IQR gate at the D-215/D-362 convention. Both nps and
time-to-depth are reported per position, with §6's note on why they are two
readings of one wall rather than independent evidence.
