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

`git grep -n 'eval_v0_quiet_fit_weights'` at `6749754`, `LC_ALL=C sort`, **8 lines
in 6 files**:

```
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

`/usr/bin/grep -rn 'CTSS' docs/` at `6749754` returns 9 lines. The two the
dispatcher names:

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
