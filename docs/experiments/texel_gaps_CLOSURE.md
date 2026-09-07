# tools/texel instrument gaps — CLOSURE

ROUTINE package, dispatched 2026-09-06. Seven independent items, no engine
behaviour change, no strength claim, no governed run. Each item closes or
STOPs on its own.

Rulings this package executes: **D-657** (TX-1, candidate retired), **D-658**
(TX-2/TX-3 constraint set and minimiser check), **D-659** (the `game` column),
**D-660** (TX-3, the toml header), **D-661** (the CTSS citation). Their prior
rulings are **D-639 (R-C)** and **D-640 (R-D)**.

## §0 First actions

- `tail -n 8 docs/decisions.md` at `beb91b1` — last number **D-650**.
- All eleven paste-block keys ABSENT (`/usr/bin/grep -F "[<key>]"`); appended as
  **D-651 … D-661** in commit `6749754`, message
  `decisions: post-book_v3 rulings and texel package`. `tools/decision_key_check.sh`
  after the commit: `663 decision keys in docs/decisions.md, no repeat outside the
  exemption`.
- **dev green at `beb91b1`**, from `artifacts/book_v3_ci_FINAL_beb91b1.txt`:
  `=== gate 1/21: cargo fmt --all --check` … `=== gate 21/21: governing-document
  citations`, then `ci: all gates passed` and `EXIT=0`.

### A missing input, named rather than worked around

The dispatcher's reading list names `HANDOFF_pistol_architect_v2.md §5 R-C and
R-D`. **No such file is in the tree** (`/usr/bin/find . -name 'HANDOFF*'` returns
`docs/experiments/wp22_HANDOFF.md`, `sessions/HANDOFF_pistol_architect.md`,
`sessions/opt-arc/HANDOFF.md`, and none carries an `R-C`). The MECHANISM is not
missing: R-A … R-F are landed ADR lines, `docs/decisions.md` D-637 … D-642, and
R-C (D-639) and R-D (D-640) are quoted in full below. Per §0.5 this is a moved
premise, not a failed one.

**R-C, D-639, verbatim**: *"tools/texel arithmetic trusted, instrument layer not;
before any Phase 2 trainer: tempo_constraints gains w3 >= w2 + 1, candidate digits
get a pinning mechanism, options.py minimiser check covers all six branches, as one
ROUTINE package; trainer objective = deployed objective, no constant term — flips
never, D-633's lesson."*

**R-D, D-640, verbatim**: *"SPSA/Texel tuning moves Stage 4 to Stage 2; ROADMAP
corrected in this commit; eval_v0_weights.toml header corrected in the tools/texel
package after quoting weights_sha256's digest scope (goldens may depend on it);
SPSA added to Phase 2's matrix as a row because D-621 shows label fitting cannot see
tactical terms — flips if SPSA's game budget exceeds what D-568-class books can
supply."*

## §1 Premises, quoted at HEAD (D-477)

Every quotation below is taken at `6749754` (paste-block commit; the code it
quotes is unchanged from `beb91b1`) with `/usr/bin/grep`, `sed -n` or `git grep`,
never the harness `grep` (D-265).

### P1 — `fit.py`'s constraint set and the refusal it leans on

`tools/texel/fit.py:374-380`, verbatim (the dispatcher's `374-381` is the same
body plus its blank line):

```
374:def tempo_constraints(top, total):
375-    """The schema, on the one free weight `w1`, with `w2 = total - top - w1`.
376-
377-    `w1 >= 1` and `w2 >= w1 + 1`, the latter being `total - top - 2*w1 >= 1`.
378-    """
379-    return [([1.0, 0.0], 1.0),
380-            ([-2.0, 0.0], 1.0 - (float(total) - float(top)))]
```

Two constraints. The schema is three relations — `w1 >= 1`, `w2 >= w1 + 1`,
`w3 >= w2 + 1` — and `fit()` parameterises `quiet = [w1, total - top - w1, top]`
(`fit.py:396`), so the third reads `top >= (total - top - w1) + 1`, i.e.
**`w1 >= total - 2*top + 1`**, and it is absent.

`round_to_schema`'s INPUT refusal, `tools/texel/fit.py:253-257`:

```
253:    if w[0] < 1.0 - 1e-9 or any(w[k + 1] < w[k] + 1 - 1e-9 for k in range(len(w) - 1)):
254-        raise FitError(
255-            f"fit: the real-valued answer {['%.4f' % x for x in w]} is not schema-feasible; "
256-            "rounding an infeasible point is the projection this module exists without"
257-        )
```

`fit.py:255` — the dispatcher's citation — is the refusal's message line. It is
load-bearing rather than redundant exactly as `training_pipeline_2026-09.md` §7
Gap A states.

### P2 — every reference to the retired candidate

`git grep -n 'eval_v0_quiet_fit_weights'` at `6749754`, `LC_ALL=C sort`, **9 lines
in 7 files** — 8 in 6 is the count at `beb91b1`, and the ninth line is D-657
itself, appended by `6749754`, which is the correction D-665 records:

```
docs/decisions.md:1380:D-657 [texel-candidate-retired]: configs/eval_v0_quiet_fit_weights.toml and its digit test are retired …
configs/instrument_quiet_fit_v0.toml:118:weights_file = "configs/eval_v0_quiet_fit_weights.toml"
crates/pistol-cli/tests/eval_weights_call_site_tests.rs:113:const CANDIDATE_DOCUMENT: &str = "configs/eval_v0_quiet_fit_weights.toml";
docs/experiments/wp22_phase1_design.md:289:`configs/eval_v0_quiet_fit_weights.toml`.
docs/experiments/wp22_phase1_impl_REVIEW.md:17:| config validation (6) | ... weights=configs/eval_v0_quiet_fit_weights.toml ...
docs/experiments/wp22_phase1_impl_REVIEW.md:45:literals.** Mutant **R3** — editing `configs/eval_v0_quiet_fit_weights.toml` to the committed
docs/research/training_pipeline_2026-09.md:293:`configs/eval_v0_quiet_fit_weights.toml` to any other legal table passed every
tools/texel/test_texel.py:860:    Editing `configs/eval_v0_quiet_fit_weights.toml` to any other legal table
tools/texel/test_texel.py:865:    candidate = FIT.committed_table("configs/eval_v0_quiet_fit_weights.toml")
```

`tools/texel/test_texel.py:857-873`, the docstring the dispatcher names:

```
857:def test_the_committed_candidate_satisfies_the_pins_it_is_registered_under():
858-    """The registered candidate is pinned by nothing but this.
859-
860-    Editing `configs/eval_v0_quiet_fit_weights.toml` to any other legal table
861-    passed every gate. What defines the candidate is not its digits but the two
862-    pins the matrix selects it under, and those are checkable without the corpus.
863-    """
```

**The reference list is not the retirement list.** The candidate document is the
head of a three-link chain of committed configuration, each link naming the next:
`configs/arena_wp22_phase1_quiet_dryrun.toml:47` names
`configs/instrument_quiet_fit_v0.toml`, which at line 118 names
`configs/eval_v0_quiet_fit_weights.toml`. Deleting the head leaves the other two
naming a file the tree does not hold, which `tools/config_check.sh` (gate 6) and
`crates/pistol-cli/tests/eval_weights_call_site_tests.rs` both read. The chain is
what retires; see §2 item B for the disposition and the coverage replacement.

### P3 — `options.py`'s six rows and the `kind == "index"` guard

`tools/texel/options.py:25-32`:

```
25:PINS = (
26-    ("J   w3 = 60", "index", 2, 60.0),
27-    ("    w1 = 2", "index", 0, 2.0),
28-    ("    w2 = 12", "index", 1, 12.0),
29-    ("    sum = 74", "sum", None, 74.0),
30-    ("K   unpinned", "free", None, None),
31-    ("T   w3=60 & sum=74", "both", None, None),
32-)
```

Six rows, four `kind`s: `index` (3 rows), `sum`, `free`, `both`. The recommended
row is `T`, the `both` row at line 31.

`tools/texel/test_texel.py`, the guard: `if kind == "index":` at **line 769**,
holding the orthogonality check through line 790; `elif kind == "sum":` at 791,
`elif kind == "both":` at 794, `else:` (the `free` row) at 799. The test's own
comment, `tools/texel/test_texel.py:772-776`: *"HOLDING THE PIN IS WHAT A RESCALE
DOES TOO, so it cannot tell a minimiser from one — and a rescale of the free solve
is exactly the construction the matrix was corrected for. The defining property of
a least-squares answer is that its RESIDUAL is orthogonal to every free direction,
which a rescale violates."*

`tools/texel/options.py:38-44`, the unchecked assertion:

```
38:    THE PIN IS APPLIED INSIDE THE SOLVE, which is what makes each row a
39-    minimiser rather than a rescale of a free one. The solve is the EQUALITY-
40-    constrained optimum and the schema's inequalities are checked afterwards by
41-    `round_to_schema`, so a pin whose optimum lies outside the schema is
42-    reported INFEASIBLE where `fit.py`'s `constrained_min` would return a
43-    boundary point. Every row this prints is strictly interior, so the two agree
44-    here; a caller adding a pin that binds should use `constrained_min`.
```

### P4 — `extract.py`'s kept columns and its per-row join verification

`tools/texel/extract.py:23-25`:

```
23:# labels_file.rs:17-50 — the column order, by name so a reader can check it.
24:MOVES, KEY_FULL, TO_MOVE = 2, 5, 6
25:SCORE_KIND, SCORE_VALUE, DEPTH, BOOK, RESULT = 7, 8, 10, 13, 14
```

`game` is `CorpusRecord` field **0** (`crates/pistol-arena/src/labels_file.rs:19`,
*"The source report's game index."*) and is not among them.

`tools/texel/extract.py:57-65`, the per-row verification:

```
57:                # THE JOIN IS VERIFIED ON EVERY ROW, not sampled. A join that
58-                # addresses the wrong record produces well-formed rows whose
59-                # features belong to another position, and nothing downstream
60-                # could tell.
61-                if rec[KEY_FULL] != manifest_key_full:
62-                    raise SystemExit(
63-                        f"extract: corpus {index} record {record_number} has key_full "
64-                        f"{rec[KEY_FULL][:40]!r}, the manifest says {manifest_key_full[:40]!r}"
65-                    )
```

**The manifest cannot verify `game`.** `artifacts/arc3r_sweep_deduped_manifest.txt`
states its own columns in its header: *"columns: corpus_index, record_number,
key_seq, key_pos, key_full, depth_turns, result, end"* — no game column. So the
join check is the only per-row referent the manifest affords, and item D needs a
second one that does not come from the manifest. §2 item D takes it and reports
what it measured.

### P5 — what `weights_sha256` hashes

`crates/pistol-cli/src/bin/pistol.rs:97`:

```
97:    identity.push(format!("weights_sha256 {}", weights_digest(&weights_file)?));
```

`crates/pistol-cli/src/bin/pistol.rs:193-201`:

```
193:fn weights_digest(path: &Path) -> Result<String, String> {
194-    let bytes = std::fs::read(path).map_err(|io| {
...
200-    Ok(pistol_cli::sha256::sha256_hex(&bytes))
201-}
```

**It hashes FILE BYTES**, not parsed values: `std::fs::read` then `sha256_hex`.
`crates/pistol-arena/src/identity.rs:9` (`pub const WEIGHTS_FIELD: &str =
"weights_sha256";`) and `:20-21` are the arena's capture of that same string; the
arena computes nothing. TX-3's condition is therefore met on the DEFER branch.

### P6 — the two CTSS sites, and threat_calculus_v1.md's scope rule

`/usr/bin/grep -rn 'CTSS' docs/` at `6749754` returns **12** lines in five files
(`ROADMAP.md` 1, `decisions.md` 1, `minimax_report.md` 6, `sealbot_notes.md` 1,
`search_next_2026-09.md` 3). The two the dispatcher names:

- `docs/research/minimax_report.md:138`:
  `| CTSS / conservative defense; CRZS | Sound proof/disproof vs 2-stone defender | CRZS solved 100% of Connect6 puzzle set | Medium-high | 15–25 | **BUILD (CTSS) / PROTOTYPE (CRZS)** |`
- `docs/research/threat_calculus_v1.md` §9 — quoted in §2 item F below.

`search_next_2026-09.md:238-249` is the finding: *"**"CTSS" could not be traced to
a defining paper** — it appears as prior work in Yang & Yen's CRZS paper (TAAI
2011, DOI `10.1109/taai.2011.65`) and nowhere that defines it. The property —
**conservative defence: a search may reject a real win but must never accept a
false one** — has a clean primary citation in **Allis 1994 §5.3.3**. Cite the
property, not the acronym."*

### P7 — `openings.rs:47-48` and D-143

`crates/pistol-arena/src/openings.rs:44-52`:

```
44:/// Read `take` openings from `path`, starting after `skip`, refusing anything
45:/// that is not a fixture.
46:///
47:/// A contiguous window: the book is emitted in content-hash order, so any
48:/// window is as much a sample as a prefix is (docs/decisions.md D-143), and
49:/// `skip t, take t` is disjoint from `skip 0, take t` by construction — which
50:/// is what a confirmatory run on the SAME book needs (docs/decisions.md D-202;
51:/// WP-1.3's confirmation had to move to the other book for want of this knob).
```

**D-143, verbatim opening**: *"The openings fixture has NO CAP and is emitted in
`game_hash` order."* True of the fixture D-143 governs — `crates/pistol-cli/tests/
fixtures/openings_v1.txt:32` states `# param emission_order game_hash_asc` — and
FALSE of every random book `load` also reads:
`random_openings_v1.txt:38`, `_v2.txt:53` and `_v3.txt:66` each state
`# param emission_order generation_order`, and the generator says why
(`crates/pistol-cli/src/random_openings/document.rs:190-194`): *"Emission order is
GENERATION order, and that is deliberate. The draws are independent, so a prefix of
this file is already a sample; sorting it by anything — coordinates, digest — would
make a prefix a slice of whatever was sorted on instead (docs/decisions.md D-143's
argument, in a book that gets it for free)."* The conclusion survives for both
kinds of book; the cited reason holds for one of them.

### P8 — how the texel tests are invoked

`tools/ci.sh:21` `readonly GATE_TOTAL=21`; `tools/ci.sh:189-190`:

```
189:step "gate 18/$GATE_TOTAL: offline texel and census tooling"
190:gate "texel tools" tools/texel_tests.sh
```

`tools/texel_tests.sh` is **not** a `cargo test` invocation at all — it is a shell
gate whose last line is `python3 tools/texel/test_texel.py`, with a `void()` (exit
2) for a missing `python3` or a missing suite. **The runner is `python3`, its scope
is the whole of `tools/texel/test_texel.py`, and the suite drives itself**: two
module-level `for` loops (`test_texel.py:913-946` and `:1024-1028`) call every test
by name and `raise SystemExit(1)` on any failure. There is no selection flag and
therefore no narrow-versus-bare split of the D-648 kind: a test added to the file
runs only if it is added to one of those two tuples, which is the failure mode this
suite has instead.

The Rust half of item B's blast radius runs under gate 3
(`tools/ci.sh:86 cargo test --workspace --locked`, the BARE form) and under
`cargo test -p pistol-cli --test eval_weights_call_site_tests` (the narrow form);
both are exercised in §3.

## §2 Per-item state

| item | state | commit |
|---|---|---|
| A tempo_constraints = the schema | **CLOSED** | `0587616` |
| B TX-1 candidate retired / TX-2 fit receipt | **CLOSED** | `77d6f98`, `fed26ee`, `d48174f`, `6b85681`, `19d565a` |
| C minimiser check in all six rows | **CLOSED** | `25f4e61`, `8e7fdf0`, `6b85681` |
| D `game` kept and per-row verified | **CLOSED** | `bd9c8c4`, `6b85681` |
| E toml header | **CLOSED as DEFERRED** (D-663) | ruling only |
| F citation corrections | **CLOSED** | `02a16c8` |
| G openings window comment | **CLOSED** | `8e8dda5` |

Nothing STOPped. Three forks were taken on the architect default and each is an
ADR line rather than a note here: **D-662** (the retirement is a chain and the
seat-pair guard is owed), **D-663** (E's branch and why), and the `game` column's
spelling, which is D-659 executed and is argued in item D below.

### Item A — the constraint set IS the schema

`tools/texel/fit.py:375-391` now returns three rows, one per schema relation.
The third is `w1 >= total - 2*top + 1`, which is `w3 >= w2 + 1` written on the one
free weight.

**The registered fit does not move, and that is checked rather than asserted.**
At the registered pins the third row's bound is `74 - 120 + 1 = -45`, dominated by
`w1 >= 1`, and `test_the_third_relation_cannot_bind_at_the_REGISTERED_pins` pins
it. MEASURED on a 2 000-row synthetic set: the answer is `[2, 12, 60]` before and
after and the table is `[2, 12, 60, 300, 1500]` before and after. The one output
that moves is the printed `singular active sets skipped`, **1 → 3**, because two
of the three rows share the normal `[1, 0]` and every size-2 active set is now
linearly dependent — a legitimate skip `constrained_min` counts rather than
swallows, and the sets are `(0,1)`, `(0,2)`, `(1,2)`.

**THE REVIEW'S "68 of 268" COULD NOT BE RE-DERIVED AND IS NOT TRANSCRIBED.**
`docs/experiments/wp22_phase1_review_rounds_4_5.md:140` reports *"68 of 268
admissible committed triples reach the flatness refusal"*, and no instrument in
the tree defines that space: `git grep -n 'triple' -- docs/` returns no
enumeration, and `C(n,3) = 268` has no integer solution, so the space is not
"every triple up to a bound" either. Attempted and rejected: triples to a bound
(N = 5..29 give 10 … 3654, never 268), triples with a bounded sum, and triples
with the committed quiet sum (456). Under `docs/process.md`'s re-derivation
clause a number reproduced by nobody's command is not reproduced, so the test
states its OWN space and counts it:

- **1140 triples** with `1 <= w1 < w2 < w3 <= 20`.
- **525** of them can bind, counted independently as `w1 + w2 > w3` — which is
  the condition for the missing bound `total - 2*top + 1` to exceed the bound
  that was there, derived rather than observed.
- With the two-constraint set the solve reaches `fit.py:255` on **exactly those
  525**, set equality and not just count equality; with the shipped set, on
  **none**.

**THERE ARE THREE REFUSALS IN `round_to_schema` AND ONLY ONE OF THEM IS ITEM A's,
which is why the test asserts on the message and not on the exception.** The
review's phrase was *"the flatness refusal"*; `training_pipeline_2026-09.md` §7
names the INPUT check at `fit.py:255`, and they are different guards. The
flatness one is the OUTPUT loop, and no constraint set can remove it, because
Python rounds half to even so a feasible `['1.5000','2.5000','4.0000']` becomes
`[2, 2, 4]` — `round_to_schema`'s own docstring says exactly this and calls an
earlier revision's claim to the contrary FALSE. MEASURED, and on a DIFFERENT
instrument from the test's so the two are not confused: driving the composition
from 240 synthetic rows over the same 1140 triples, the flatness guard fires 204
times and the input guard **0 times, under the old constraint set and the new
alike** — which is why a row-driven enumeration is the wrong instrument for item A
and a stated quadratic is the right one. Under the stated quadratic the input
guard fires 525 times before the fix and 0 after, and the flatness guard 0 times
either way.

**Why the test drives a stated quadratic.** `A = I`, `b = 0` puts the
unconstrained minimiser at `w1 = 0`, below every lower bound the schema can
state, so the answer lands ON the binding constraint at every one of the 1140
pins and the missing relation decides the outcome. Rows would do this only where
the corpus happens to want a low `w1`: MEASURED, over three generating tables on
240 synthetic rows the two-constraint set reached the input refusal **0 times**,
so a row-driven enumeration would have had a negative control that never fires.
The composition under test — `tempo_constraints`, `constrained_min`,
`round_to_schema` — is `fit()`'s own at `fit.py:405-408`.

### Item B — the candidate retired, the mechanism replaced

**TX-1.** The retirement is the chain D-662 records: three committed configs and
two Rust tests. `configs/eval_v0_quiet_fit_weights.toml`,
`configs/instrument_quiet_fit_v0.toml` and
`configs/arena_wp22_phase1_quiet_dryrun.toml` are gone;
`tools/config_check.sh` after: `21 engine config(s), 1 weight table(s), 18 arena
config(s), 3 book config(s), 2 solver config(s)`.

**What coverage the retired tests carried, listed before they were deleted, per
R1.** `test_the_committed_candidate_satisfies_the_pins_it_is_registered_under`
(`test_texel.py`) carried four property checks on the candidate's digits, all of
them about the candidate and all of them dying with it — and `training_pipeline`
§7 Gap B is that four other legal tables passed every one. In
`eval_weights_call_site_tests.rs`,
`the_committed_candidate_differs_from_the_committed_table_and_changes_the_search`
carried *two committed weight documents make the search answer differently*,
which `one_perturbed_table_entry_changes_search_output` carries on scratch
documents four lines above it — same claim, no retired file.
`the_two_seat_configs_resolve_to_different_weight_documents` carried the SEAT-PAIR
guard, and that one is **owed, not replaced**, for the reason D-662 records — with
its count corrected by **D-664**, because D-662 said three committed arena configs
pair identical seats and the answer is FIVE. The miscount came from a
`grep -A 4` window that missed a `config =` line sitting further from its section
header, which is `docs/process.md`'s own "checked against the wrong population"
defect happening inside a ruling that cites that document, and the two configs it
missed make the conclusion stronger rather than weaker: `arena_wp13_fair_corpus`
and `arena_wp13_fair_random` are first-player-rate MEASUREMENTS whose headers
pre-register the self-match as the instrument.

**TX-2, the replacement mechanism.** `tools/texel/fit.py` gains `receipt_lines`,
`receipt_digest` and `print_receipt`; `main` prints the receipt last. The body
names the inputs by digest (rows and `configs/eval_v0_weights.toml`), the pin rule
(`options_row=both w3=60 sum=74`), the populations each filter clause left, the
split, the real-valued answer and the table. The rows file's PATH is deliberately
absent so a receipt cannot pin the directory a run happened in.
`tools/texel/fixtures/fit_rows_v1.txt` is the committed input: 400 rows,
**twenty columns — `extract.py`'s own width, including the `game` key** — and
`test_the_committed_fixture_is_what_the_stated_generator_writes` asserts its body
is what the stated `synth_rows` parameters produce, so the fixture and its
generator cannot drift apart in silence. It shipped at nineteen columns and the
review caught it: a receipt pinned on an artefact the pipeline can no longer emit
exercises syntax where `docs/process.md`'s dry-run discipline asks for
attribution. `fit.py` now refuses a row file of any other width by name, so the
skew cannot recur invisibly.

The pinned digest is
`5e17e8ff64e13dfb9d0c4e95ec191f04f243dab327c3c84b1d28e8cbe5bbba7d`, and the test
**re-derives it from the printed body** rather than reading fit.py's own number
back — which is what makes M4 (a digest over the pins alone) a red test rather
than an invisible one. `test_the_receipt_digest_MOVES_when_the_input_does` changes
one label and shows the digest moves, because a pin that cannot move is a
constant.

**And the retirement is defended.**
`test_the_retired_candidate_chain_is_named_by_RECORDS_and_by_nothing_live` reads
the tracked working tree with `git grep -l -F` and refuses a live reference to any
of the three — case-insensitively, and by BASENAME because that is what the
measurement that found the references used — with a CONTROL, the ruling must be
found, so a pass cannot come from a search that matched nothing. **There is no
allowlist**: which documents may name a retired file is not knowable from a path
and does not need to be, so the rule is the property `_live_tree` states — a
retired name may stand in any record and in no config, crate or tool. An
allowlist of records was tried first and §7 records what it cost.

### Item C — six rows, and the two KKT conditions kept apart

`_free_directions` states, per `options.PINS` kind, the quiet directions the pin
leaves free — `{e_k : k != j}` for a single index pin, `{e0-e2, e1-e2}` for the
sum pin, `{e0-e1}` for the two-pin `both` row, all three units for `free` — and it
is derived from what each pin MEANS, not read out of `options.py`. Each row is
then checked on the least-squares defining property (residual orthogonal to every
free direction, and to the intercept), and a **rescaled free solve that holds the
same pin** must fail it. All six rows, negative control firing in all six.

The rows carry a deterministic residual (`(number * 7919) % 401 - 200`, no seed
and no clock) because noiseless rows make the residual zero at the unpinned answer
and an orthogonality test over a zero residual passes whatever the answer is.

**A MUTATION FOUND A DEFECT HERE TWICE, AND THE SECOND TIME IT WAS THE FIRST
FIX.** M2 empties the `both` row's free-direction list. At `d48174f` it SURVIVED:
the two KKT conditions were folded into one boolean and the intercept condition
alone still rejected the rescale, so a row whose directions had gone missing
still looked defended. `8e7fdf0` split them and added `bool(directions)` to the
free-direction condition — **and that made the POSITIVE check fire and the
NEGATIVE CONTROL vacuous**, because `not control_free` is true whenever the list
is empty. The finding's sentence was discharged and its property re-created one
step to the left, in the control; REVIEW-impl's MAJOR-3 caught it and proved both
directions with mutants of its own. `6b85681` is the fix that holds: emptiness is
a property of the DIRECTIONS, so it is checked where they are built — a per-kind
direction-count check — and asserted in BOTH the positive check and the negative
control, while `_minimiser_conditions` reports the two KKT conditions and nothing
else. M2 now dies at its registered check.

**The interior sentence.** The sentence at `options.py:43-44` as of `beb91b1` — *"Every row this prints
is strictly interior, so the two agree here"* — is gone; `schema_slacks` and `interior` compute
it, `main` prints an `interior` column per solved row plus a
`N of M solved row(s) strictly interior` line and a named line for any row that is
not, and the docstring now points at the check. The committed row prints `-`
because it is a reference and not a solve, so it makes no claim about the two
minimisers agreeing. The `no` branch is driven through the SHIPPED solve with a
pin `options.py` does not ship (`index`, entry 0, value 1.0), which lands the
answer exactly on the `w1 >= 1` bound — the case the retired docstring said could
not arise.

### Item D — `game`, and why the column is not the bare field

**The premise the ruling rests on is true and the obvious execution of it is
wrong.** `training_pipeline_2026-09.md` §1 says *"A game-level split is one column
in `extract.py`"*, and `artifacts/research_2026-09/game_count.txt` states the
quantity as **`distinct (tranche, game) : 3487`**. MEASURED here over the deduped
manifest and the sixteen corpus files: the bare `game` field takes **218** values
where `(corpus_index, game)` takes **3487**, because the corpus numbers games from
0 within each tranche — tranche 1's header says `# derived games 436` and so does
every other tranche's but the sixteenth's. A bare column would put sixteen
different games in one group, and `extract.py`'s own docstring says *"The trainer
reads this and never the corpus"*, so nothing downstream could recover the
tranche. **This is a way the code produces a wrong answer, so it is fixed rather
than deferred to Phase 2**: the column is spelt `<corpus_index>:<game_index>` and
the row file's header says so on its own second line. No split logic is added and
none is implied — how to group is still Phase 2's.

**The per-row verification, and what referent it uses.** The manifest CANNOT
verify `game`: its own header states its columns as *"corpus_index,
record_number, key_seq, key_pos, key_full, depth_turns, result, end"*. So the
existing join check says which RECORD was read and nothing about which FIELD. The
referent that does is the corpus's own emission order — records come game by game,
so `game` never decreases as record numbers rise. MEASURED over all 89 805 deduped
rows: `game` non-decreasing **True**; `turns_played`, the adjacent field and the
likeliest misread, non-decreasing **False**. That is the negative control, and
`test_extract_REFUSES_a_game_column_that_is_not_the_game_index` builds it by
setting every record's game field to its own `turns_played`. A second refusal
covers a `game` field that is not a number, by name rather than by `int()`'s
`ValueError` (hard rule 3).

**The shipped tool was driven on the registered workload, and it reproduces a
number it does not share an instrument with.**
`python3 tools/texel/extract.py` over the real manifest prints
`extract: 89805 row(s) written from 3487 distinct game(s), every score_kind KEPT`
— against `game_count.txt`'s independently produced `deduped records : 89805` and
`distinct (tranche, game) : 3487`. No row was refused. Counted a third way, from
the emitted file rather than from either instrument:
`/usr/bin/grep -v '^#' rows_real.txt | cut -f20 | LC_ALL=C sort -u | wc -l` gives
**3487**. Receipt: `artifacts/texel_gaps/RECEIPT_texel_gaps.sha256`.

### Item E — DEFER, and the reason is not the one the ruling guessed at

TX-3's condition is measured in P5 and the answer is FILE BYTES, so D-660's DEFER
branch fires. **D-663 records it, and records that no golden is the reason.**
D-663 names two sites; the enumeration is THREE and it is exhaustive.
`git grep -n 'eval_v0_weights' -- crates/` returns every crate-side reader, and
the three that take a digest of the file —
`crates/pistol-cli/tests/baseline_snapshot_tests.rs:453` and `:645`, and
`crates/pistol-cli/tests/handshake_identity_tests.rs:15` — all recompute it at run
time from `std::fs::read`, so none pins a literal and none would churn. No
committed literal digest of the document exists outside prose: `git grep
41ef5496` finds it in `docs/` alone. What the edit
would break is the provenance link between the 89 805-position labelled corpus and
the weights that labelled it, cited as a bare digest in a GOVERNING document that
no mechanism checks. The cost is stated rather than hidden: `configs/
eval_v0_weights.toml:46-47` goes on saying *"SPSA/Texel replaces these in Stage
4"* against `docs/ROADMAP.md:490`'s *"SPSA/TEXEL TUNING MOVES HERE FROM STAGE 4
(D-640)"*, and a reader following the config's own pointer lands on the correction.

### Item F — the corrections are listed, and there are seven of them

The dispatcher named two sites; D-661 says *"cited nowhere as a technique"*, and a
repair that fixes one instance of a class while a sibling stands is this project's
D-305/D-331 defect. So every technique-use was corrected, and here they are:

**Seven sites, and the CLOSURE said seven where the P6 premise said nine and the
answer to both is 12 lines in five files — the count is corrected above and this
table is the edit list, which is a different thing from a match count.**

| # | site | what changed |
|---|---|---|
| 1 | `docs/research/minimax_report.md:14` | the acronym list loses "CTSS/ITSS"; CRZS gains its DOI |
| 2 | `docs/research/minimax_report.md:48` | the bullet names the PROPERTY with Allis 1994 §5.3.3, says in terms that CTSS is not used as a technique name, and marks the CRZS "100 %" UNVERIFIED |
| 3 | `docs/research/minimax_report.md:138` | the table row: property + CRZS DOI; verdict text unchanged in force |
| 4 | `docs/research/minimax_report.md:141` | "CTSS conservative-defense pattern" → the property with its citation |
| 5 | `docs/research/minimax_report.md:164` | Stage 3's line, same substitution |
| 6 | `docs/ROADMAP.md:501` | Stage 3's line, same substitution |
| 7 | `docs/research/sealbot_notes.md:88` | "the cousin of CTSS" → the cousin of the property |

**No verdict moved.** `BUILD (CTSS) / PROTOTYPE (CRZS)` became
`BUILD (conservative defence) / PROTOTYPE (CRZS)`; every other BUILD/PROTOTYPE/
SKIP token is byte-identical. No theorem text was touched.

**AND ONE PREMISE OF `search_next` §3.3 IS FALSE, WHICH IS ITSELF A FINDING.** It
says *"`minimax_report.md:138` and `threat_calculus_v1.md` §9 treat CTSS as an
established technique"*. `git grep -c -F 'CTSS' 6749754 -- docs/research/threat_calculus_v1.md`
**exits 1 — the document never held the word**, at that revision or any other
(D-668). What §9 — a
table headed "Adopted with sources" — actually lacked was the DOIs the same
paragraph asks for, so it gains two rows: `ADOPT-VC`, carrying VCDT (Wu & Lin
2010, IEEE TCIAIG 2(3):191-207, Fig. 6; Zhang, Huang, Zhang & Liu, 2018 CCDC, DOI
`10.1109/ccdc.2018.8408300`) and VCST (Wu, Su, Li, Zhang & Zhou, 2021 33rd CCDC,
DOI `10.1109/ccdc52312.2021.9601901`), and `ADOPT-CONSERV`, naming Allis 1994
§5.3.3 as the source of a soundness direction §7 already requires and adopting
nothing new. The document's existing attribution at `:76-77` was left alone,
because nothing in `search_next` §3.3 says it is wrong.

### Item G — the window justification

`crates/pistol-arena/src/openings.rs:47`'s *"the book is emitted in content-hash
order"* was true of the one book D-143 governs and false of the three
`load` actually reads most: `openings_v1.txt:32` states
`# param emission_order game_hash_asc`, and `random_openings_v1.txt:38`,
`_v2.txt:53` and `_v3.txt:66` each state
`# param emission_order generation_order`. The comment now says the reason is the
order each book states in its OWN header, and that neither order can correlate
with an opening's strength — which is the property that makes a window a sample,
and is the generator's own argument at
`crates/pistol-cli/src/random_openings/document.rs:190-194`. Comment only;
`cargo test -p pistol-arena --locked` passed 11 + 18 + 1 + 1 tests with no golden
touched.

## §3 Mutation evidence (R2, D-55y)

Harness `artifacts/texel_gaps/mutants_texel.py`. **FOUR RUNS ARE KEPT AND THE
CLOSING ONE IS THE LAST**: `mutants_texel_d48174f_M2ALIVE.log` (six mutants, M2
ALIVE), `mutants_texel_8e7fdf0.log` (six, M2 dead elsewhere, harness exit 1),
`mutants_texel_6b85681.log` (ten, all dead, exit 0) and
**`mutants_texel_5ab5152.log` — eleven mutants at the closing revision, all dead
at their registered check, harness exit 0**, taken in `/home/tom/pistol-wt/texel-mut`
because a `tools/ci.sh` run was in progress in the other worktree. The first
three are the evidence for the item-C findings and for the confirmation's own
verdicts, and are kept for that reason rather than as the closing answer.

**D-650 IN ITS PYTHON FORM, and it is why the harness purges.** D-650's defect was
a restore whose mtime let cargo keep a mutant compiled. CPython validates a
`.pyc` by the source's `(mtime, size)`, which is the same shared-fingerprint
mechanism, so a restore putting back an older mtime at an equal size can serve a
stale mutant. The harness therefore purges every `__pycache__` and touches every
source before the baseline and after every restore, and asserts the restore
landed with `git diff --quiet` — aborting the whole run if it did not, because a
restore nobody checked is the trust the original defect exploited.

```
worktree /home/tom/pistol-wt/texel-mut  revision 5ab515293630f0f531ee0b5997a136fef47c4963
baseline GREEN from a purged cache: exit 0
DEAD    M1 item A: the third schema relation dropped from tempo_constraints
DEAD    M2 item C: orthogonality replaced by pin-holds in the `both` branch
DEAD    M7 item B: the retirement guard escaped by spelling the BASENAME
DEAD    M11 item B: the retirement guard escaped by CASE, which a literal search misses
DEAD    M8 item D: the distinct-game count taken on the bare index
DEAD    M9 item B: the receipt drops the weights digest, so the tripwire stops firing
DEAD    M10 item B: the committed fixture drifts one column from its stated generator
DEAD    M3 item C: the interior check disabled -- everything reports interior
DEAD    M4 item B: the receipt digest computed over the PINS line alone
DEAD    M5 item D: the `game` column written without its per-row verification
DEAD    M6 item B: a reference to the retired candidate re-added to a live config
11 of 11 dead at their registered check; 0 died elsewhere; 0 alive
post-run suite: exit 0
```

**Five of the eleven are findings from the two review rounds turned into
call-site mutants** — M7 and M11 the retirement guard's two escapes, M8 the
undefended count, M9 the tripwire, M10 the fixture-generator drift — which is
what makes them evidence rather than decoration: each one is a defect that was
actually in the tree, not a break invented to be caught.

**AND THE HARNESS ITSELF WAS A REVIEW FINDING TWICE OVER.** It spelled four
outcomes as exit 1, three of which are "no answer was taken" — a reader could not
tell a live mutant from a full disk (`tools/SHELL_CHECKLIST.md` item 12,
obligation 1); it now exits **2** with a `RUN VOID:` prefix for a red baseline, a
moved mutation site and a failed restore, and preflights `python3`, the suite and
a clean tree before it starts (obligation 2). And its restore assertion compared
`git diff --quiet` against the INDEX — the same referent `git checkout --`
restores FROM, so the criterion was a property the defect class preserves and a
staged mutant would have been restored and certified. It compares against
**HEAD** now.

**AND THE FIRST RUN IS KEPT, because it holds the finding.** At `d48174f` M2 was
**ALIVE** with the other five dead, and that run is
`artifacts/texel_gaps/mutants_texel_d48174f_M2ALIVE.log`, re-taken to a file
rather than left in a transcript — the disposition CLAUDE.md's own note about
WP-1.8c's four reports asks for. It is the evidence for the item C finding above
and its registered check is that revision's wording, not this one's.

**M2's history is the package's own story in miniature**: ALIVE at `d48174f`;
dead at `8e7fdf0` but at the POSITIVE check rather than its registered one, which
the harness reported as WRONG and exited 1 on and which was the visible symptom of
MAJOR-3's vacuous control; and dead at its registered check at `6b85681`. The
registration was never moved to match a result — what moved, once, was the check's
NAME, when MAJOR-9's fix renamed M6's target, and the harness followed the rename.

## §4 Receipts, and one instrument run that is VOID

`artifacts/texel_gaps/RECEIPT_texel_gaps.sha256` (gitignored artifacts, sha-indexed
here per hard rule 8 and D-469):

```
e05c13b0bf442ad66d85a68f4a316947d6dcff37818660b8cf235f90e1dc5547  ci_5ab5152.txt
898f5328189fd096149c0219572c6ad1dd5719856f745bb24fe791717c2bac56  ci_f73539e.txt
96c66fcf266fbb9dd6fb33b707d95c8859a955b0eb134e14697ef1372501a5cb  ci_VOID_tree_changed_under_it.txt
a9fce935c29faa033d2102fe85a91ccc25e04e7f7dc04b22bc04b0b1cfce8c9b  mutants_texel.py
95112117b06119b740278f59241e4395df149f7cc41ce65f862728c54d1ba0d6  mutants_texel_5ab5152.log
2e2ad06c48f86bc2f97c528a78e2992220a4422745bdc92f30b014920d9a03e3  mutants_texel_6b85681.log
be50a7130f2dbc07bef0c1f94e4f1866f7c49a5f5b681eba2bdbfc2f83723e7f  mutants_texel_8e7fdf0.log
181f1c3320f945dad3125555c218fbbe797712c7d98989871ccb36c7d862ed7b  mutants_texel_d48174f_M2ALIVE.log
b3c21e1687ac0665cb865714e1d88d96d5c06031227e2cfc0f7312c27b75e968  extract_real.log
eb4264a945e9147d6fdfcfc46ef670457bbee52bb97f403c10d31bdd08552d26  rows_real.txt
```

**Two CI logs are kept beside the closing one and both are non-verdicts.**
`ci_VOID_tree_changed_under_it.txt` is the run this session voided by moving the
worktree under it; `ci_f73539e.txt` is the run that went red at gate 3 under an
exported `CARGO_TARGET_DIR` and produced D-672's reproducer. Neither adjudicates
anything, and they are kept because a finding's evidence is worth more than a
tidy directory.

**A CI RUN WAS VOIDED BY THIS SESSION AND IS RECORDED RATHER THAN QUIETLY
RE-TAKEN.** A first `tools/ci.sh` was started in the mutation worktree at
`8e7fdf0` and, while it was running, that same worktree was checked out to
`d48174f` to re-take the M2-ALIVE evidence. A gate run whose tree moves under it
adjudicates nothing — it is the VOID class of `tools/SHELL_CHECKLIST.md` item 12,
self-inflicted — so the run was killed, its log kept as
`ci_VOID_tree_changed_under_it.txt`, and the closing run taken afterwards in a
tree nobody touched. This is the same law the Process section states for mutation
worktrees, applied to the gate runner, and the reason it is written down is that
a green log from a moving tree is indistinguishable from a green log.

## §5 Known limits, said plainly

- **The review's "68 of 268" is not reproduced and not transcribed**, and it is
  now RECORDED as rejected with its attempted reproducers (**D-666**), which
  CLAUDE.md's Process requires and this document alone did not satisfy. A second
  party attempted it independently and failed the same way. Item A's test states
  its own space (1140 triples, 525 binding).
- **Rule 9's mechanized cap does not reach these files.**
  `tools/file_justification_check.sh` enumerates tracked `.rs` and `.sh` by ADR
  (D-131, D-234, D-467), so `tools/texel/*.py` is outside it. `fit.py` and
  `test_texel.py` were already over the soft cap before this package
  (448 and 1033 lines at `beb91b1`) and are further over now. This package does
  not widen the gate — that is a decision with its own costs and it has no ADR —
  and it does not add an entry the gate would then refuse for naming a path
  outside its set. **RECORDED AS OWED.**
- **~~The `game` column makes an extract-written row file 20 fields where
  `read_rows` reads 19.~~ CLOSED by the fix round.** This limit was written when
  `read_rows` was width-blind, and REVIEW-impl's MAJOR-8 showed what it cost:
  19, 20 and 21 columns were all accepted and 18 raised a bare `IndexError`. It
  is now `COLUMNS = 20` with a named `FitError`, `synth_rows` writes the game key
  so the fixture is an instance of the kind the pipeline emits, and both
  directions carry a test. **The residual question for Phase 2 is unchanged and
  smaller**: whether a trainer wants `read_rows` to expose `game` as a grouping
  key or to hand it the column raw.
- **The seat-pair guard has no subject and is owed by the next registered SPRT**
  (D-662). Nothing in the tree will remind anyone except that ADR line.
- **`configs/eval_v0_weights.toml` still says Stage 4** where the ROADMAP says
  Stage 2 (D-663). Deliberate, deferred, and mechanically enforced by the fit
  receipt.
- **The fit receipt digests `configs/eval_v0_weights.toml`**, so Phase 2's
  weight-file change makes `FIT_RECEIPT_SHA256` red. That is the intended
  behaviour of a reproduction pin and the test's failure message says how to
  re-derive it, but it is a cost a successor should meet knowingly rather than
  as a surprise.

## §6 REVIEW-impl, and the fix round it forced

**The review FAILED the package** at `8e7fdf0`, on items B and D and on the
package's own record. Report: `docs/experiments/texel_gaps_impl_REVIEW.md`,
fresh context, strongest model, pinned revision matching HEAD at the time, with
its own worktrees on `/home` and its own mutants. **Every finding below was
verified with the reviewer's reproducer before its fix landed.** The fix round is
`6b85681` and this document; the standing cap is one fix round per review, so a
second failure on any item STOPs that item.

| finding | what it caught | disposition |
|---|---|---|
| **BLOCKING-1** | the CLOSURE at the reviewed revision stopped at §1 P8, forward-referenced §2 and §3 four times, and cited **no gate log for any gate** | §2-§7 land, and §7 quotes the gates' own output |
| **MAJOR-1** | the fit receipt was pinned over a **19-column** fixture while item D, in the same package, made `extract.py` write **20** — two green tests pinning the fixture and the extractor APART | `synth_rows` writes the game key, the fixture is regenerated at 20 columns, `FIT_RECEIPT_SHA256` re-pinned to `5e17e8ff…` |
| **MAJOR-2** | item D's headline number was defended by nothing: `games.add((index, game))` → `games.add(game)` survived the whole suite and prints 218 for the corpus's 3487 | the extract test captures stdout and asserts `from 4 distinct game(s)`; mutant **M8** |
| **MAJOR-3** | `8e7fdf0`'s own fix disarmed the control it was fixing — `bool(directions)` in `free_ok` made the POSITIVE check fire and the negative control vacuous, the finding's property one step to the left | emptiness moves out of `_minimiser_conditions` to the caller, checked in BOTH directions plus a per-kind direction-count check; M2 now dies at its registered check |
| **MAJOR-4** | the harness spelled four outcomes as exit 1, three of them "no answer was taken" | exit 2 with a `RUN VOID:` prefix, a preflight for `python3`, the suite and a clean tree |
| **MAJOR-5** | `restore()` checked `git diff --quiet` against the INDEX, the same referent `git checkout --` restores from — a criterion the defect class preserves | `git diff --quiet HEAD --`, plus the clean-tree preflight |
| **MAJOR-6** | D-662's *"MEASURED … at `6749754`: 8 references in 6 files"* is **9 in 7** there; 8-in-6 is `beb91b1`'s | D-665; P2 above corrected and the ninth line listed |
| **MAJOR-7** | D-662's *"FALSE of three committed documents"* is false of **eighteen** on its own reading (one weight table is committed), and D-664 had corrected the OTHER reading | D-665, superseding D-664 |
| **MAJOR-8** | `read_rows` was width-blind — 19, 20 and 21 columns all accepted, 18 an unnamed `IndexError` | `COLUMNS = 20` and a named `FitError`, with a test driving both directions |
| **MAJOR-9** | the retirement guard searched the full path where the measurement used the basename, so a basename re-add escaped it; and `RETIRED_MAY_NAME` was a bare list that silences the check when a live path is added | the search is by basename; the list was backed by a property and then, after §7's finding, **replaced** by it; mutants **M7** and **M11** |
| **MAJOR-10** | the D-663 tripwire's message named the constant and not the obligation, so a Phase 2 session would re-pin, go green, and rot the provenance link | the message names the re-quote and the citing sites; **D-669** enumerates them; mutant **M9** |
| MINOR-1 | the CLOSURE's own CTSS count was 9 where its own command returns 12 | corrected in P6 |
| MINOR-2 | gate 21 FORCED the §9 amendment where D-662 called it precedent | D-665 |
| MINOR-3 | gate 6 does not force link 3 — **an arena config may name an engine config the tree does not hold with every gate green** | D-665, recorded and unclosed |
| MINOR-4 | the unreproduced "68 of 268" was recorded nowhere, and its refusal is misnamed | **D-666** |
| MINOR-5 | item G had no ADR line and `book_v3_SUMMARY.md` still listed it as owed | **D-667**, and the ledger updated |
| MINOR-6 | rule 9's gate is `.rs`/`.sh` only, so these Python files are outside it | **D-670**, recorded as owed |
| MINOR-7 | the docstring said two normals are parallel; all three are, and `skipped` goes 1 → 3 | corrected |
| MINOR-8 | the receipt excluded the rows PATH for machine-independence and emitted the weights PATH | the docstring says why the two differ |
| MINOR-9 | D-663 named one citing site for the digest and never enumerated | **D-669** enumerated them, and miscounted its own list — corrected and retired by **D-671**, §7 |
| MINOR-10 | `search_next` §3.3's premise about `threat_calculus_v1.md` §9 is FALSE, and `ADOPT-VC` is an addition made under a correction ruling | **D-668** |

**What the reviewer could not break, and it is worth as much as the findings.**
A subject-side mutant (their H1) that makes `options.solve_pinned`'s `both` row
return a rescale of the free solve **dies** — item C's property is genuinely
defended and only its evidence's registration was wrong. Item D's non-decreasing
referent is externally derived in `docs/process.md`'s sense and its negative
control is measured rather than asserted (their H2, H5 both die). `FIT_RECEIPT_SHA256`
reproduces from an `awk` lift of the printed body piped to `sha256sum`, sharing
nothing with `fit.py`. Item F moved **no** BUILD/PROTOTYPE/SKIP verdict, and the
reviewer seconds the widening from two sites to seven on D-661's own ground. The
reviewer also could not reproduce "68 of 268" by any principled enumeration,
independently, which is why D-666 records it as rejected rather than restated.

### The mutation set, re-taken at the fix round

Ten mutants at `6b85681`, four of them the reviewer's own findings turned into
call-site mutants (M7 basename escape, M8 bare-index count, M9 dropped weights
digest, M10 fixture-generator drift). Baseline GREEN from a purged cache,
**10 of 10 dead at their registered check, harness exit 0**, post-run suite
green. Log: `artifacts/texel_gaps/mutants_texel_6b85681.log`.

**M6's registered check text was updated and that is not a post-hoc move.**
MAJOR-9's fix RENAMED the check (`and only RECORDS name configs/…toml` became
`and nothing under configs/, crates/ or tools/ names eval_v0_quiet_fit_weights`),
so the harness's registration was tracking a string the code no longer prints.
The registration follows the rename; no threshold moved, and M6 died at the
renamed check in the same run it was renamed for.

**M10 was re-shaped, and this one is worth reading.** Its first form dropped the
game column from `synth_rows`, and the suite then RAISED — `read_rows`'s new
arity refusal firing at the first test that generates rows — so no named check
fired and the harness printed `got: []`. A mutant whose death cannot be
attributed to a check is evidence of nothing in particular, so M10 now mutates
the committed FIXTURE instead and dies at
`and they are the rows the stated parameters produce`. The harness also learned
to print a raise's last line rather than an empty list, because `got: []` beside
a non-zero exit is not a legible verdict.

## §7 The scoped confirmation, and what it found

`docs/experiments/texel_gaps_confirm_REVIEW.md`, fresh context, pinned at
`f73539e`, with its own worktrees under `/home/tom/confirm-wt/` and its own
mutants. Its question was not whether the fix round was good work but whether
each of REVIEW-impl's 21 findings had its PROPERTY discharged or its SENTENCE.
**Nineteen were DISCHARGED, verified by rebuilding the reviewer's own mutants and
harder variants; two were MOVED; and it found one NEW defect that alone made the
revision unlandable.**

**THE FOUR SURVIVING FINDINGS WERE ONE PROBLEM, AND NAMING IT IS THE POINT OF
THIS SECTION.** Each was a hand-maintained LIST or a hand-counted NUMBER standing
where a PROPERTY or a derived value was available:

| what was found | the list or number | what replaced it |
|---|---|---|
| **NEW: gate 18 RED at `f73539e` in a clean checkout** | `RETIRED_MAY_NAME`, an allowlist of records — and the very next document landed, `texel_gaps_impl_REVIEW.md`, quotes the retired basenames in its own reproducers and was not on it | the allowlist is **deleted**. `_live_tree` alone decides: a retired name may stand in any record and in no config, crate or tool. MAJOR-9 had already said a list is not a property; the error was adding the property beside the list instead of in place of it |
| **MAJOR-9 residual** | a case-sensitive `git grep -F` where the claim is about a NAME | `-F -i`, and mutant **M11** re-adds `configs/Eval_V0_Quiet_Fit_Weights.toml` to a live config |
| **MAJOR-10 residual** | the tripwire's message listed four citing sites where D-669 had found more | the message names **the command**, and points at D-669 as the one place that enumerates — a list in a failure message rots exactly like the constant beside it |
| **MINOR-9 / D-669** | "nine lines in six files", counted off its own seven-path list by eye | **D-671**, which corrects it to seven AND retires the number: the population is self-referential — at `19d565a` the same command answers 15 lines in 10 files, because this package's own review, confirmation and closure now discuss the digest — so the COMMAND is the claim and the load-bearing subset is a judgement over two sites |

**BLOCKING-1 was rated MOVED for a reason worth keeping.** Its fix text said gate
output would be *"quoted in §7"* and the document ended at §6 — a forward
reference to a section that did not exist, which is the shape BLOCKING-1 was
opened to fix, recreated in the sentence closing it. This is that §7.

**What the confirmation could not break.** MAJOR-3's fix — the crux — holds: with
the `both` row's direction list emptied, the positive check AND the negative
control both fail, neither vacuous. MAJOR-5's fix was attacked with the exact
D-650-class construction: a staged mutant passes `git diff --quiet --` (exit 0,
falsely clean) and fails `git diff --quiet HEAD --` (exit 1), so the new referent
closes the hole. D-665's four measured claims all re-derived by an independent
method, including the 18-of-18 and exactly-5 seat enumerations by a `tomllib`
parse rather than the `grep -A 4` window that caused the original miscount.

**Two sub-claims of D-666 are recorded as NOT independently re-derived** — the
"204 of 1140" flatness count and the set equality between the 525 that can bind
and the 525 the two-constraint set refuses — and the honest disposition differs
for the two. **The set equality is not a measurement at all: it is what the
shipped test asserts**, `sorted(refused_old) == sorted(can_bind)`, so it is
pinned by gate 18 on every run and M1 dies at it. **The 204 is a supporting
figure and it was re-taken, by this session and not by a second party**, which is
weaker evidence and is labelled as such: the row-driven probe over the same 1140
triples answers `old {OK: 936, FLAT: 204}` and `new {OK: 936, FLAT: 204}` — the
same under both constraint sets, with the INPUT refusal at zero throughout, which
is the whole point the figure makes.

### A finding this package walked into and is recording rather than fixing

`tools/ci.sh` was first run in a worktree with `CARGO_TARGET_DIR` exported — the
rule for MUTATION and verification work applied to the gate runner, which has
path assumptions that rule was never about. The run went red at gate 3, and the
message is the finding: `solver_determinism: RUN VOID: no binary at
target/release/solver-selftest after a green build`. **The script was right and
the seam was not.** `tools/solver_determinism.sh:32` resolves its binary by a
path literal where D-250 moved four sibling gates onto cargo's own artifact
stream, and `crates/pistol-cli/tests/solver_determinism_gate_tests.rs:18` asserts
`output.status.success()`, so a VOID reads as a failing determinism gate —
`tools/SHELL_CHECKLIST.md` item 12 obligation 3, and D-281/D-285's reading
exactly. **D-672** records both halves with the reproducer; neither is fixed
here, because a fix round is not where an unrelated gate is re-plumbed.

## §8 The gates, cited from their own output

**A gate claim cites the gate's own log, never a wrapper's exit status**
(CLAUDE.md, Closure). The run below is `tools/ci.sh` at **`5ab5152`**, taken in
`/home/tom/pistol-wt/texel-gaps`, a worktree nobody edited while it ran and with
**no `CARGO_TARGET_DIR` export** — the export is what voided the first attempt
(§7, D-672). Log: `artifacts/texel_gaps/ci_5ab5152.txt`.

`5ab5152` is the revision immediately before this section: a log cannot quote
itself, so what the closing run adjudicates is every line of this package except
this section and the receipt line that names it. Nothing under `crates/`,
`configs/` or `tools/` differs between them.

```
=== gate 1/21: cargo fmt --all --check
=== gate 2/21: build from the git-tracked file set
=== gate 3/21: cargo test --workspace --locked
=== gate 4/21: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate 5/21: artifact rejection
=== gate 6/21: config validation
=== gate 7/21: perft oracle
=== gate 8/21: tactical fixture at its pre-registered threshold
=== gate 9/21: cross-process determinism
=== gate 10/21: differential search oracle
=== gate 11/21: staged generator soundness (four parts)
=== gate 12/21: solver oracle (four gates)
=== gate 13/21: solver determinism
=== gate 14/21: movetime ceiling on the D-95 reproducer class
=== gate 15/21: arena self-match smoke
=== gate 16/21: sealbot anchor platform suite
=== gate 17/21: file-justification check
=== gate 18/21: offline texel and census tooling
=== gate 19/21: decision-key uniqueness
=== gate 20/21: carve-document label consistency
=== gate 21/21: governing-document citations

ci: all gates passed
```

The four gates this package can move, in their own words:

```
config_check: 21 engine config(s), 1 weight table(s), 18 arena config(s), 3 book config(s), 2 solver config(s)
test_texel: all checks passed (including census_classes)
decision_key_check: 674 decision keys in docs/decisions.md, no repeat outside the exemption
governing_citation_check: 15 governing document(s), 0 proposed path(s)
docs/experiments/wp22_phase1_design.md: 16 citation(s) checked, 0 unreproduced
```

**And gate 13 passed here, which is the point of D-672.** `solver_determinism`
is green in this run and was RUN VOID in the first attempt, from the same tree at
a neighbouring revision — the difference was the environment, not the subject.
That is the whole content of the finding: the gate could not say so in a way its
own test could hear.

## §9 What a successor needs

- **The package is CLOSED on all seven items.** Nothing STOPped. Every state is
  in §2's table; every ADR line this package wrote or corrected is D-651 … D-672.
- **Three things are recorded as OWED and nothing in the tree will remind
  anyone**: the seat-pair guard, which has no committed subject and belongs to
  the next registered SPRT (D-662, D-665); rule 9's cap, which the gate does not
  reach for `.py` (D-670); and the solver-determinism seam, which reads its own
  void as a failure (D-672).
- **One deferral is mechanical rather than remembered.** `FIT_RECEIPT_SHA256`
  goes red on any edit to `configs/eval_v0_weights.toml`, and its message says
  what that owes (D-663, D-669, D-671). Phase 2 will meet it.
- **The lesson this package paid for twice, stated once.** A fix that adds a
  property beside a list has not replaced the list, and the list is what breaks.
  Three counts in this package's own rulings were wrong because they were read
  off a list by eye instead of derived by a command, and one gate went red
  because an allowlist was one document stale. Where a property is available,
  the list is the defect — not a belt beside a brace.

## ERRATA

Appended by the pre-Phase-2 sweep (`docs/experiments/pre_phase2_sweep_CLOSURE.md`,
items H1-H3), append-only. Nothing above is edited and nothing this package did is
undone. Each entry names the ruling it is written under.

### E-1 — the closing revision is not the revision the CI ran at, and the diff is docs-only (D-674)

§8 cites `tools/ci.sh` at **`5ab5152`** and says so in its own words: *"a log
cannot quote itself, so what the closing run adjudicates is every line of this
package except this section and the receipt line that names it."* The package
CLOSED at **`007f821`**. D-674 gives two ways to hold a package's closing
revision green, and this is the second: the diff between them is quoted, and it is
docs-only.

```
$ git diff --stat 5ab5152..007f821
 docs/experiments/texel_gaps_CLOSURE.md | 144 ++++++++++++++++++++++++++++-----
 1 file changed, 123 insertions(+), 21 deletions(-)

$ git diff --name-status 5ab5152..007f821
M	docs/experiments/texel_gaps_CLOSURE.md
```

One file, and it is this one. Nothing under `crates/`, `configs/` or `tools/`
differs, which is the claim §8 already makes for `5ab5152` against `f73539e` and
which now covers the closing revision too. `docs/decisions.md` is not in the diff
either — D-674's wording admits decisions because a closure commonly appends one,
and this closure did not.

**What this entry does NOT claim.** It does not claim that every gate would pass
at `007f821`; it claims that no gate's SUBJECT moved between the cited run and the
close. Gate 21 reads documents, so a docs-only diff is not by itself proof for
that gate — but `texel_gaps_CLOSURE.md` is not on
`tools/governing_citation_check.sh`'s `GOVERNING` array (the array is at
`tools/governing_citation_check.sh:47-63`), so gate 21 does not read it, and the
only file the diff touches is one no gate reads.

### E-2 — the population behind "525 of 1140", and its relation to "68 of 268" (D-479)

D-479's rule is that a number is bound to the run and the population that produced
it, and that a document stating a ratio names the terms of BOTH its numerator and
its denominator. §2 item A and D-666 state two ratios about one guard, and only one
of them had its population written down. Both are named here.

**"525 of 1140" — the space is stated, closed-form, and holds no engine.** The
denominator is the set of integer triples `{(w1, w2, w3) : 1 <= w1 < w2 < w3 <= 20}`,
whose size is `C(20,3) = 1140`. The numerator is the subset satisfying
`w1 + w2 > w3`, which is the condition for the schema relation `w3 >= w2 + 1`'s
missing bound `total - 2*top + 1` to exceed the bound `tempo_constraints` already
carried — derived from the schema, not observed from a run. It is **525**. The bound
20 is the test's own choice and nothing else in the tree fixes it; the ratio is a
property of that stated box and moves with it. RE-DERIVED for this entry by an
instrument sharing nothing with the shipped test — a `python3` `itertools.combinations`
enumeration rather than `tools/texel/`'s solver path:

```
|{1<=w1<w2<w3<=20}| = 1140
  of which w1+w2>w3 : 525
C(n,3)==268 for n in 3..60: []
```

**"68 of 268" — the space was never stated, and the two ratios are not about the
same thing.** `docs/experiments/wp22_phase1_review_rounds_4_5.md:140` reports
*"68 of 268 admissible committed triples reach the flatness refusal"*, repeated at
`docs/experiments/wp22_HANDOFF.md:167`. D-666 records it as NOT REPRODUCED with the
attempted reproducers. This entry adds the part D-666 left implicit and D-479
requires: **the two ratios name different guards over different populations, so
neither corrects the other and neither is the other's re-derivation.**

| | denominator | numerator | the guard reached |
|---|---|---|---|
| "68 of 268" | "admissible committed triples", a space no instrument in the tree defines and `C(n,3)` cannot produce | 68 | named as **the flatness refusal** — `round_to_schema`'s OUTPUT loop |
| "525 of 1140" | `1 <= w1 < w2 < w3 <= 20`, stated by the shipped test | 525 | the **INPUT** check, `fit.py:274-278` at `ac61305` |

`docs/research/training_pipeline_2026-09.md` §7 (Gap A) names the input check
correctly — *"The schema's third relation, `w3 >= w2 + 1`, is enforced **only** by
`round_to_schema`'s *input* check at `fit.py:255`"* — so the misnaming is the
review's phrase and not §7's. **That citation's LINE has moved and its MECHANISM
has not** (D-477): at `ac61305` the input refusal is `fit.py:274-278` and the
flatness loop is `fit.py:287-293`, both inside `round_to_schema` at `fit.py:263`,
because this package's own item-A commit lengthened the docstring above them. §2
above and §7 of `training_pipeline_2026-09.md` are RECORDS and keep the number
they were written with; this entry states where the guards are now. The two guards behave oppositely under the fix, which
is why conflating them would have hidden the whole result: under the stated
quadratic the INPUT guard fires 525 times before the fix and 0 after, while the
FLATNESS guard fires 0 times either way; over the 240-row synthetic probe the
FLATNESS guard fires 204 times under both constraint sets and the input guard 0
times under both. **No constraint set can retire the flatness guard** — it exists
for Python's half-to-even rounding — so a ratio about it could never have measured
Gap A's fix, whatever its population turned out to be.

### E-3 — the grant ledger: neither closure doc quotes one, so neither has one (D-673)

D-673: *"a loop grant exists only as the operator's words quoted in the package's
closure doc; an unrecorded grant did not happen."* Searched at `ac61305` with
`/usr/bin/grep -rn -i "grant\|operator"` over each package's closure doc and the
documents it names.

**`texel_gaps` (this document): NO GRANT IS RECORDED, AND NONE WAS NEEDED.** The
document quotes no operator words anywhere. The standing cap is one fix round per
review, and this package ran two reviews with one fix round each — REVIEW-impl
(`texel_gaps_impl_REVIEW.md`, §6) fixed at `8e7fdf0`/`6b85681`/`f73539e`, and the
scoped confirmation (`texel_gaps_confirm_REVIEW.md`, §7) fixed at `19d565a`. Two
fix rounds in the package, one per review, and the cap is per review. **No
exceedance. Nothing to undo, and nothing was undone.**

**`book_v3` (`docs/experiments/book_v3_SUMMARY.md`): A GRANT IS ASSERTED AND NOT
QUOTED, WHICH IS THE PROCESS FINDING.** `book_v3_SUMMARY.md:12-14` reads *"This
package stopped once and was resumed by the operator, who returned the blocking
decision to the session rather than ruling on it. It then ran three more rounds
under an explicit two-round grant."* The operator's words appear nowhere in that
document, in `book_v3_registration.md`, or in `matrix_book_v3_storage.md`. Under
D-673 the grant did not happen, and the sentence rests on nothing a successor can
check.

**What the finding is, stated exactly, because the obvious reading is the wrong
one.** The finding is NOT that book_v3 blew a cap. The cap D-481 states is one fix
round per review, and `book_v3_registration.md:502-509` records exactly one — §13,
*"This is the package's **one** fix round (D-481)"*. The "three more rounds" of the
summary sentence are PACKAGE rounds after the R1 stop, which the standing cap does
not govern and which the asserted grant was therefore not needed for. So the
defect is the sentence's shape rather than its arithmetic: **it states a budget
(two) and a spend (three) where neither term is anchored to anything a reader can
verify, and a successor reading it cannot tell whether a cap was exceeded, whether
a grant existed, or whether the two numbers are even in the same unit.** That is
what D-673 exists to make impossible going forward: the grant is the quote or it is
not a grant.

**Nothing is undone in either package.** D-673's last clause is explicit, and both
packages' work stands on its own gates.
