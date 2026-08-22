# REVIEW-design (re-review) — WP-1.5b unit U1, `docs/experiments/U1_gate_supersession.md` u-rev 4

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped shell operators restored: `&gt;`/`&amp;` to `>`/`&`).
The dispatching session repaired U1; it did not review it. Reports live in the
tree rather than in a scratchpad because finding IDs are cited across units.
-->

## Header

- **Pinned revision:** `1964026` (`1964026c8efe89a4fea09f8e5c499cd40b7d9c42`).
- **Matches HEAD:** YES at entry and at exit — `git rev-parse HEAD` returned `1964026c8efe89a4fea09f8e5c499cd40b7d9c42` both times; `git diff 1964026 HEAD -- docs/experiments/U1_gate_supersession.md` is empty at both checks. The subject did not move during this review.
- **Document:** `docs/experiments/U1_gate_supersession.md`, u-rev 4, 335 lines (`wc -l`, MEASURED).
- **Prior report:** `docs/experiments/wp15b_U1_REVIEW_urev2.md`, pinned `5baea10`, **PASS** (0/0/0), against u-rev 2. (`docs/experiments/wp15b_U1_REVIEW.md`, pinned `38f21b9`, FAIL 1/0/1, against u-rev 1, is the round that PASS repaired.)
- **Reviewer context:** fresh — no prior turns in this conversation touched the pistol repo before this review.
- **Scope, as given:** re-review of u-rev 4's own edits only. Findings are restricted to (a) a false or drifted claim in surviving text, including any this round's own edits introduced, (b) a NORMATIVE claim that lost its home to a strike, (c) a normative error. The absence of struck text is not a finding; a pointer is not a finding; gate 15 (head/foot u-rev agreement, self-counts) is not re-derived.

## VERDICT: **FAIL — 0 BLOCKING, 1 MAJOR, 0 MINOR**

Derivation: this report's own `# FINDINGS` section contains one `### MAJOR` heading and zero `### BLOCKING` / `### MINOR` headings (`grep -c` against the section below: BLOCKING 0, MAJOR 1, MINOR 0).

---

# FINDINGS

### MAJOR 1 — "the one PASS in this work package" is false; there were at least three landed PASSes in WP-1.5b before this round was written, and this round's own new text restates the false count twice

**Claim quoted, site 1** (lines 39–41, inside this round's newly-added struck-and-explain parenthetical):

> "...so a head describing this unit as awaiting that review was false from `f81706a` onward and named **the one PASS in this work package** nowhere at all."

**Claim quoted, site 2** (line 58, this round's newly-added U1-A table row):

> "It carried no row for them at all until u-rev 4, which is how **the one PASS in this work package** went uncited by the unit that earned it (`docs/decisions.md` D-342)."

**Contradicting evidence.** At least three REVIEW verdicts inside WP-1.5b were landed PASS before `1964026` (this round's own commit) was authored, not one:

```
$ grep -m1 -oE "VERDICT[: ]*\*{0,2}(PASS|FAIL)" docs/experiments/wp15b_U1_REVIEW_urev2.md
VERDICT: **PASS

$ head -1 docs/experiments/wp15b_trackC_REVIEW_impl.md
# WP-1.5b Track C — REVIEW-impl
$ sed -n '20,23p' docs/experiments/wp15b_trackC_REVIEW_impl.md
**Verdict summary:** both fixes hold when driven directly, both tests were confirmed
to fail against the pre-fix sources, and both control halves were confirmed to bind by
mutation. Three MINOR findings, all in the *prose and seam* that `b067d47` newly added
around its guard, none in the guards themselves. **0 BLOCKING, 0 MAJOR, 3 MINOR.**

$ sed -n '153,155p' docs/experiments/wp15b_trackC_R19_REVIEW_impl.md
## Verdict

**PASS.**
```

Ordering, confirmed by `git merge-base --is-ancestor`:

```
$ git merge-base --is-ancestor 84ff8d7 1964026 && echo before   # Track C PASS
before
$ git merge-base --is-ancestor d59f0de 1964026 && echo before   # Track C R19 PASS
before
```

Both Track C PASSes (`84ff8d7`, `d59f0de`) are ancestors of `1964026`, and both are also ancestors of `a3bdb12` (the commit landing D-342 itself):

```
$ git merge-base --is-ancestor 84ff8d7 a3bdb12 && echo "predates D-342"
predates D-342
$ git merge-base --is-ancestor d59f0de a3bdb12 && echo "predates D-342"
predates D-342
```

So "the one PASS" was already false when D-342 first used the phrase, and this round's text imports the same falsehood twice rather than checking it. (This matches the later, independent correction recorded in `docs/decisions.md` D-345, landed *after* this round: *"M11 measures V6's transcribed 'zero PASS at close' FALSE — three landed PASSes, and U1's last landed review is one of them."* — three, not one, and U1's is only one of the three.)

**Why it breaks.** This is a numeric/uniqueness claim asserted as fact in surviving prose, false at the moment it was written and still false, checkable in seconds with `grep`/`git merge-base`, in a document whose whole reason for existing this round is to correct exactly this species of defect (a stale self-referential claim never re-checked against the tree). It is also a restatement (D-331 rule 3: lineage/status tables may say a fact exists, not assert unverified content) of a claim whose home is `docs/decisions.md` D-342 — and the restatement carries the error forward rather than merely pointing at it.

**Fix scope:** at both sites, drop the cardinality claim ("the one PASS") or replace it with a verified one ("U1's own PASS," or, if worth keeping the broader point, "one of at least three landed PASSes in WP-1.5b" with a citation) — a one-clause edit at each of the two sites, no restructuring required.

---

# VERIFIED WITH NO FINDING

**1. The head clause replacing "the counts are D-309's and are not restated here" with "and is that review's home."** The struck clause was false: `U1-A`'s own table (line 57, unchanged by this round) already restates the revision-7 review's counts verbatim ("7 BLOCKING, 7 MAJOR, 9 MINOR"), so "not restated here" was false in the same document. The replacement clause, "D-309 ... is that review's home," is accurate under `D-331`'s own rule ("where a claim has landed in `docs/decisions.md`... THAT is its home") — D-309 states the revision-7 verdict verbatim.

**2. The lineage-table restatement of "7 BLOCKING, 7 MAJOR, 9 MINOR" itself (line 57).** Considered as a possible D-331 rule-3 violation (a lineage table restating verdict counts rather than pointing at their home). Rejected as a *new* finding: it is pre-existing text, unchanged by this round, and the identical pattern (`REVIEW-design | revision 7, 6feb40a | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR`) appears verbatim in `U2_node_protocol.md`, `U3_tier_t.md` and (by count) `U4_soundness_instrument.md`, none of which any post-D-331 review (`wp15b_U2_REVIEW_urev5.md`, `wp15b_U3_REVIEW_urev6.md`, `wp15b_U4_REVIEW_urev8.md`) flagged. It reads as an accepted project convention (bare verdict-count identification of a round, not "finding content"), not a live defect this round introduced or could have caught.

**3. Whether the strike was executed as an actual deletion-and-pointer or merely a cosmetic "mark."** Compared against `D-346`'s own instance base and the established in-tree convention (e.g. `U4_soundness_instrument.md`'s `U4-Z` lead-in: `*~~...~~ **STRUCK AT u-rev 9 AND REPLACED WITH NOTHING**...*`). U1's u-rev 4 does the same thing at the same shape: the false paragraph is removed from its original site and replaced there by a genuine pointer ("live in that round's report... which is their home under D-331"); the strikethrough-quote-plus-explanation is a separate, established convention for recording *what* was struck and *why*, not a restoration of the false claim as live text. This matches D-346 and is not the "C-prime-mark" shape D-345 killed (which was about an ongoing marking convention governing unmarked text project-wide, not a one-time disposal footnote).

**4. §4.1/§4.2's MEASURED test counts, re-taken independently against `1964026`** (unchanged by this round, but named in the dispatch as the class this unit failed on at u-rev 1):

```
$ cargo test -p pistol-cli --test solver_edge_check_tests -- --list 2>&1 | tail -3
the_shipped_workspace_has_no_normal_edge_on_the_solver: test

11 tests, 0 benchmarks

$ cargo test -p pistol-cli --test solver_link_check_tests -- --list 2>&1 | tail -3
no_solver_source_reaches_any_shipped_binary_of_this_workspace: test

19 tests, 0 benchmarks

$ grep -n "repo_root()" crates/pistol-cli/tests/solver_edge_check_tests.rs \
                          crates/pistol-cli/tests/solver_link_check_tests.rs
crates/pistol-cli/tests/solver_edge_check_tests.rs:288:    let ran = edge_check(&repo_root(), "pistol-solver");
crates/pistol-cli/tests/solver_link_check_tests.rs:735:    let ran = link_check(&repo_root(), "crates/pistol-solver");
```

11 and 19 confirmed (MEASURED); §4.2 row (a)'s derived (11−1)+(19−1) = **28** confirmed correct. No drift since the u-rev-2 PASS review re-verified the same numbers.

**5. "It carried no row for them at all until u-rev 4"** (line 58, about U1's own review-history row in `U1-A`). Checked against `f81706a`'s (u-rev 3) copy of `U1-A`: the table has four rows (M0 red team, revisions 2–6, revision 7, restructure red team) and none about U1's own u-rev 1/2 reviews. True.

**6. "It was written at u-rev 2 and never re-read"** (line 38). Checked: the struck paragraph was introduced verbatim at `5baea10` (u-rev 2) and is byte-identical through `3543a7f` and `f81706a` (`git diff 5baea10 f81706a` touches no line in that paragraph). True.

**7. Grep sweep for any other stale reference to the struck claim or to "u-rev 2" awaiting-review language elsewhere in the document.** `grep -n "awaiting\|NOT BEEN REVIEWED\|has been reviewed" docs/experiments/U1_gate_supersession.md` returns only the struck quotation itself (inside the `~~...~~` span) and the new head sentence ("NO REVIEW-design HAS RUN AGAINST THIS u-rev"), which is true and current. No other site restates the old, false status.

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

**Candidate: "false from `f81706a` onward" understates when the head paragraph actually became false — it was already false as soon as `wp15b_U1_REVIEW_urev2.md` landed (`e3f0bc3`), two commits earlier than `f81706a`.**

Reproducer:
```
$ git log --oneline --graph 1964026 -- docs/experiments/U1_gate_supersession.md docs/experiments/wp15b_U1_REVIEW_urev2.md
* 1964026 ... U1 reaches u-rev 4 ...
* f81706a ... U1's foot label reaches u-rev 3 ...
* 3543a7f ... the carve-provenance clause said revision 7 was NEVER REVIEWED ...
* e3f0bc3 ... U1's u-rev 2 re-review lands and it is the session's first PASS ...
* 5baea10 ... U1 reaches u-rev 2 ...
```
`e3f0bc3` (the PASS landing) precedes both `3543a7f` and `f81706a`, and the head paragraph is unchanged across all three. So the paragraph was false starting at `e3f0bc3`, not first at `f81706a`.

Rejected as a reportable finding, not because it's untrue, but because "false from `f81706a` onward" is literally true (it does not assert `f81706a` was the *first* point of falsity, only that falsity held from there on) — it is the same shape of loose-but-not-false phrasing the u-rev-2 PASS review (`wp15b_U1_REVIEW_urev2.md`, "Rejected with attempted reproducer" section) already declined to charge for an analogous ambiguity ("stale at 9/19"). No sentence in the document claims `f81706a` is when the falsity began, and no downstream claim depends on that date being the origin rather than a later still-false checkpoint.

**Candidate: "a fourth hand-kept copy of a history the reports already hold" (line 42–43) — is "fourth" a checkable, and possibly wrong, count?**

Attempted reproducer: tried to enumerate candidate "copies" of this history (the review reports themselves, D-342's ADR line, the now-struck head paragraph, U1-A's table) to see whether a literal count of 3 existing copies + 1 hypothetical new one = "fourth" holds up mechanically. No stable enumeration is defined anywhere in the document or in D-346 for what counts as a "copy" in this rhetorical construction, so there is no falsifiable referent to check it against — it reads as rhetorical emphasis ("yet another"), not a numbered claim. Rejected: not a checkable claim, therefore not a finding.
