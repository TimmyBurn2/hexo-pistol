# Stage-3 scoped detector — CLOSED at the premise gate. The target was wrong by 63x-85x, and it is now measured.

> **Audience: the next agent.** Every claim names the artifact or command that
> establishes it. §8 is the environment hazards this session actually hit — two
> of them are new. ADR line: **D-508**.

## 0. ONE LINE FOR THE MORNING

**The package this dispatch scheduled cannot be designed yet, because the number
it was told to design against is wrong: "cut solver calls ~6x" is an nps-ratio
shortfall wearing a call-count label, and the real figure — measured on the
committed seats at HEAD, not inherited — is that the detector must pass about
ONE NODE IN 380 where the trigger now fires, not one in six. Nothing was built,
no option was chosen, `dev` is untouched and green; what exists is a memo whose
central claim survived a fresh-context red team, a re-runnable derivation, a
registered bench that re-measures the WP-1.8 arc's abort under today's engine
(it aborts harder), and seven rulings that are yours.**

---

## 1. In plain language, before any technical detail

The solver is expensive. The plan was to call it less often — specifically, six
times less often — and that was supposed to be enough to make it affordable.

It is not enough, and the reason is a plumbing detail with a large consequence.
The search and the solver spend from **the same pot of nodes**. So when the
detector decides *not* to call the solver, those nodes do not vanish — the
search gets them, runs deeper, and reaches more places where it wants to call
the solver again. Cutting calls therefore does not buy what it looks like it
buys, and the arithmetic that assumed it did is off by about two orders of
magnitude.

Measured on today's engine: the gate-on seat currently makes about **24**
expensive solver calls per search and needs to make about **2**. Because those
22 dropped calls hand roughly 45,000 nodes back to the search, the detector has
to be about **380 times** more selective than the current trigger, not 6.

There is a second, smaller finding worth a sentence: WP-1.9 and WP-1.9b made the
search 1.75x-2.0x faster, and they made the solver seat *slightly slower*. The
solver does not use the evaluation function those packages sped up. So the gap
this package was asked to close has widened since it was scheduled.

None of this says the detector is impossible. It says the brief was wrong, and a
brief is the thing you fix before you choose between options rather than after.

---

## 2. What the session did, in order

| step | outcome |
|---|---|
| §0.1 D-504 / D-505 | already appended by WP-1.9b (`docs/decisions.md:1076`, `:1078`); §0.1's own condition, discharged. Next free number was D-508 |
| §0.3 `dev` green | `tools/ci.sh` 19/19 at `21e05f8` — §6 |
| §0.2 `book_v2` | **NOT generated, and that is a finding** — §5 |
| Premise verification | **P2 FAILS**, P4 fails as named and is repaired, P1 and P3 hold — `docs/experiments/stage3_detector_premise_memo.md` |
| PREMISE RED-TEAM | fresh context, **STANDS WITH CORRECTIONS**; 3 BLOCKING, 5 MAJOR, 6 MINOR, all in the argument, none in the conclusion; one fix round applied |
| Current-nps bench | registered before the run, four legs, 0 refused, IQR clean; **the bracket aborts harder at HEAD** |
| Matrix / red-team / design / impl / SPRT | **none reached.** A premise STOP is before the matrix by construction |

---

## 3. The finding, with its receipts

**The mechanism, quoted at the line where the unit is consumed** (which is what
D-477 requires of an axis): `crates/pistol-search/src/pvs.rs:140-142`
— `total_nodes = search_nodes + solver_nodes` — stopped on at `:681`. One
budget, so a gated call funds the search rather than shrinking it.

**The backfill, at its endpoints, with no model and no fitted term.** The OFF
seat holds no solver, so it *is* the gate-everything limit of any detector. Per
position, both seats of the same registered bench: OFF searches 50,176 nodes;
ON searches 2,847 and spends 49,137 in the solver. Gating every call turns the
second into the first — 47,329 nodes come back to the search.

**The inversion.** With `a` = µs/search node, `c` = µs/solver visit, `u` = solver
visits per search node, `ratio(u) = a(1+u)/(a + u·c)`, inverting at the
registered bound to `u = a(1-R)/(R·c - a)`:

| | at WP-1.8c-era nps | **at HEAD, measured** |
|---|---|---|
| CORPUS band 15 (≥ 0.50) | 190.0x | **379.3x** |
| CORPUS band 35 (≥ 0.50) | 250.9x | **507.7x** |
| TRIGGER-RICH (≥ 0.25) | 35.8x | **88.3x** |

**Independent corroboration of the "about two calls", by a route that knows
nothing about the bracket**: `configs/bench_wp18c_solver_on.toml:9-13` derives
the cap 2048 from the 0.5 s deployment turn as *"two capped calls"*. The
inversion lands on 2.11.

---

## 4. The current-nps run — registered first, and it re-measures the WP-1.8 arc

`docs/experiments/stage3_premise_nps_registration.md`, written and amended before
any governed leg. Both committed WP-1.8c seats re-run unchanged at `21e05f8` on
`tools/bench_block.sh`, engine sha256 `e0eb1b19…` — the digest WP-1.9b recorded
for its shipped landing.

| band | OFF nps | ON nps | ratio | WP-1.8c ratio | bound | reading |
|---|---|---|---|---|---|---|
| CORPUS 15 | 439,819 | 19,319 | **0.0439** | 0.0809 | ≥ 0.50 | **ABORTS** |
| CORPUS 35 | 375,684 | 8,953 | **0.0238** | 0.0458 | ≥ 0.50 | **ABORTS** |
| TRIGGER-RICH | 297,184 | 6,557 | **0.0221** | 0.0488 | ≥ 0.25 | **ABORTS** |

IQR clean on all six medians (0.36 %–0.83 %, convention 10 %). `k` on the OFF
seat 1.754 / 1.815 / 2.021; **on the ON seat 0.953 / 0.945 / 0.914** — the
gate-on seat got slower, which is the structural claim measured rather than
argued.

**Criterion N, registered before the run, HOLDS**: all 44 OFF entries reproduce
WP-1.8c's exact node counts, 0 differences, and the ON seat's aggregates match
too — so `safety_net_top_k`, the one key the seats gained since (`e4bb5bf`), is
measured inert at its committed 0 rather than assumed so, and `k` is a speed
ratio.

**This does NOT discharge D-504.** That line discharges the nps-jump limb
*"inside the detector's own bracket"*, and there is no detector. Whether a
standalone re-measurement discharges it instead is ruling 7. The Stage-2-exit
limb and D-428 are untouched; D-505 has not flipped.

---

## 5. `book_v2` was not generated, and the reason is a finding

It cannot be generated without a code change. The output name is a compile-time
constant — `crates/pistol-cli/src/random_openings/mod.rs:17` — used by the
binary and by six test files (`git grep -n "FILE_NAME" -- crates/` returns 21
lines), and the rendered header hard-codes the version and the name of the v1
pinning test. §0.2 is a work package with its own design and REVIEW-impl, not a
first action.

**AND THE NAIVE DISCHARGE DESTROYS THE RECORD.**
`configs/random_openings_v1.toml` states the regeneration command with
`--out-dir crates/pistol-cli/tests/fixtures`, and the tool overwrites an existing
file. Because the name is fixed at compile time, running that command with a
`_v2` config **overwrites `random_openings_v1.txt`** — the book retired for
governed use but still readable as the artifact governing every closed SPRT
verdict. CI catches it (the regeneration test goes red), but only after the
committed bytes are clobbered in the working tree.

---

## 6. Gates

`tools/ci.sh` at `21e05f8`, run in a detached worktree with its own `target/`
(WP-1.9b §8 hazard 1), log `artifacts/stage3_ci_base_v1.txt`. Read from the
gate log's own lines: all nineteen `=== gate N/19:` lines, final line **`ci: all
gates passed`**, `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"`
returns **0**, and gate 9 closes `determinism: ok — 5 seat(s), no difference
outside nps/time in any of them`.

The closure revision adds only documents, `tools/stage3_premise_derive.py` (a
new file, not on any gate path) and one ADR line; `artifacts/stage3_ci_closure_v1.txt`
is the run that adjudicates it.

**One CI run in this session is VOID and is not a failure** —
`artifacts/stage3_ci_VOID_targetdir_v1.txt`. See §8 hazard 1.

**Observed and recorded rather than fixed: `tools/stage3_premise_derive.py` is
over rule 9's soft cap and carries no registry entry.** Gate 17 mechanizes the
rule for tracked `.rs` and `.sh` files only, and `docs/rule9_justifications.md`
says so in its own header, so the gate neither asks for an entry nor would
accept one for a path outside its file set. Two `tools/*.py` files were already
in that position before this package — `wp15b_attribution_check.py` and
`wp16_warm_attribution_check.py`, both larger — so this follows the established
shape rather than setting a new one. **The rule's text is broader than its gate**,
and whether `.py` joins the mechanized set is `tools/` hardening's business
(WP-1.10), not this package's; it is named here so nobody later reads the silence
as an oversight.

---

## 7. What is owed — seven rulings, each one ADR line

They are stated in full in the memo's §8 and are not restated here (D-423).
In one line each: **(1)** which target figure the resumed package designs
against; **(2)** whether D-465's `solver_calls` counter lands first as its own
unit; **(3)** whether the option field (a)-(e) is re-opened against the corrected
target; **(4)** the value fixture's repair and the recall gate's missing
denominator; **(5)** `book_v2`'s scope now that §5 shows it is a work package;
**(6)** the detector's own per-node cost budget, which §3.3 of the memo bounds;
**(7)** whether a standalone re-measurement discharges D-504's nps-jump limb.

**The roadmap consequence, stated because D-471 has a clause about it.** D-471
flips to Stage 2 *"immediately if the detector cannot reach the bracket at its
registered kill point, or if its SPRT reads h0"*. **NEITHER CLAUSE HAS FIRED**:
there is no detector, so there is no kill point and no SPRT. The roadmap
therefore does NOT flip on this session's authority, and D-508 says so. Whether
the corrected target should flip it anyway is inside ruling 3 and is the
operator's.

---

## 8. Environment hazards — two are NEW

1. **NEW. Do not export `CARGO_TARGET_DIR` around `tools/ci.sh`.** CLAUDE.md's
   Environment section says it for `cargo test` in the live tree; it applies to a
   worktree run too, and this session voided a CI run learning that.
   `crates/pistol-cli/tests/solver_link_check_tests.rs` builds its own scratch
   cargo workspaces and a shared target directory makes one fixture read
   another's dep-info — 8 of its 19 tests failed at gate 3/19. **A worktree
   already isolates `target/`; add nothing.**
2. **NEW, and it cost this session its derivation instrument.** `git checkout --
   <path>` on a file that was `git add`ed earlier restores the STAGED version,
   silently discarding every later edit. A fix round's worth of work on
   `tools/stage3_premise_derive.py` went that way and had to be rebuilt. The
   artifacts it had produced survived on disk while the tool that produced them
   did not — which is the "receipt without its instrument" state D-469 exists to
   prevent, arrived at from the other direction. **Stage nothing you are still
   editing, and never `git checkout --` a path to "undo" an edit.**
3. `pkill -f <pattern>` matching the session's own shell kills the shell
   (exit 144) and the rest of the compound command never runs. Match narrowly.
4. Everything WP-1.9b §8 lists still holds: CI reads the working tree, so run the
   green-confirmation in a worktree and edit freely in the live tree; a comment
   changes the release binary; no bench sharing the machine with a build
   (`ps -eo cmd | grep -c '[c]argo'` read 0 before every leg here); `/usr/bin/grep`
   or `git grep` for anything recorded.

---

## 9. Where everything is

| Document | What it is |
|---|---|
| `docs/experiments/stage3_detector_premise_memo.md` | **the memo — revision 2, post-red-team.** The finding, its four premise adjudications, and §8's seven rulings |
| `docs/experiments/stage3_premise_nps_registration.md` | the current-nps run, registered before it was taken, with its result in §7 |
| `tools/stage3_premise_derive.py` | the derivation, re-runnable, both input modes |
| `artifacts/stage3_premise_derivation_v2.txt` | the derivation at WP-1.8c-era nps |
| `artifacts/stage3_premise_nps_derivation_v1.txt` | the derivation at HEAD |
| `artifacts/stage3_premise_nps_*_v1.txt` | the four governed bench legs |
| `artifacts/stage3_ci_*` | the CI runs, including the void |
| `artifacts/stage3_premise_redteam_v1.md` | the red team's report |
| `sessions/Stage-3-detector/` | a POINTER to this file only |

Receipts digest-indexed in `artifacts/stage3_export_receipt_v1.txt`, whose own
sha256 is recorded in §10 — `artifacts/` is gitignored, so that line is the
anchor D-469 asks for.

## 10. Export receipt anchor

`artifacts/stage3_export_receipt_v1.txt` — sha256
**`30abdff445d4ae5dd1243c6c3f2f309fd71da4ca98260e19dfe6b458b500d3d6`**

Twelve files listed, the receipt itself excluded because a file cannot carry
its own digest. That sha256, in this committed document, is the anchor D-469
asks for: `artifacts/` is gitignored, so a digest list living only there anchors
nothing — the lesson `ec48aea` recorded for WP-1.9b, applied ahead of the
mistake.
