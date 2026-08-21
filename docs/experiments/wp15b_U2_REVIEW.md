# REVIEW-design (CONFIRMATION PASS) — `docs/experiments/U2_node_protocol.md`

**Pinned revision: `38f21b9`.** Document under review: `docs/experiments/U2_node_protocol.md`,
**u-rev 1**. Source carved from: `docs/experiments/wp15b_design.md` §5 at `6feb40a`
(deleted from the tree; retrieved with `git show 6feb40a:...`).

**HEAD status:** at the time this review ran, HEAD had advanced to `9421d19`
(one commit past the pin — `docs(experiments): B3's repair lands as shape 2 …`,
which lands D-316). **`38f21b9` is NOT current HEAD.** However
`git diff 38f21b9 HEAD --stat` shows only `docs/decisions.md` (+D-316),
`docs/experiments/U3_tier_t.md` and `docs/experiments/U4_soundness_instrument.md`
changed — **`docs/experiments/U2_node_protocol.md` is byte-identical at both
revisions.** The subject of this review did not move, so the findings below hold
against current HEAD as well as against the pin.

**Fresh context:** this session had no prior turns in this conversation; it read
CLAUDE.md, the owner table, the rev7 review and the U2 document cold, per the
dispatch instructions.

**Dispatch type:** CONFIRMATION PASS, per the dispatch prompt — this text is
recorded as "clean on merits after 5 rounds" and the brief was to check three
named things, not to conduct an open-ended review. Proportionality was applied:
no finding below is speculative: check 1's finding is a byte-level diff output,
check 2's finding is verified against `pistol-core` with a throwaway test (run,
confirmed, deleted), and check 3's finding is a `grep` against `docs/decisions.md`.

---

## VERDICT: FAIL

Two verified findings survive proportionality screening. Neither invalidates the
carve mechanism itself (check 1's finding is a citation-pointer correction, not a
design-content change) and neither is new to this text (both pre-date the carve,
carried verbatim from `6feb40a`) — but the confirmation pass's job was precisely
to check for exactly this shape of thing, and F2 in particular is a live,
unrepaired instance of the SAME defect class the rev7 review already named
MAJOR (M4/M6, MAJOR 8), now found a third time, in the section the restructure
relied on being clean. A confirmation pass that reports PASS over an unexamined
instance of a defect class the project has already paid one review round to name
is the vacuity CLAUDE.md's own Process section forbids for pre-registration
criteria, applied here to a review criterion instead.

---

## Check 1 — carve changed only cross-reference retargeting

**Method.** Extracted §5 mechanically from both revisions and diffed byte-for-byte:

```
git show 6feb40a:docs/experiments/wp15b_design.md | sed -n '406,760p' > old_sec5.md   # 355 lines
sed -n '176,530p' docs/experiments/U2_node_protocol.md > new_sec5.md                   # 355 lines
diff -u old_sec5.md new_sec5.md
```

Both extracts are exactly 355 lines, matching the owner table's measured span
(`awk '/^## /{print NR}'` at `6feb40a` reproduced independently and matches the
table's own line numbers for every section, not just §5). The diff produced 15
hunks, 148 diff-lines. Every hunk but one is a straightforward `§n` → `**Unit**
§n` / `WPQ_seed.md §n` retarget with the item number preserved (e.g. `§12.4` →
`**U3** §12 item 4`; `§7.2` → `WPQ_seed.md §7.2`; `§8.2` → `**U4** §8.2`; `§15
item 11` → `U2-Z item 11`; `§6` → `**U3** §6`), which is squarely inside the
declared exception, or is the addition of an item number to a previously-bare
`§15` citation (`(§15)` → `(U2-Z item 8)`) — a necessary consequence of §15 now
being split by item, verified against the owner table (item 8 is U2's and its
content — "LAW-RIPOSTE and LAW-LEDGER hand off to WP-1.6" — is exactly what the
citation names), not flagged.

**One hunk is not a retarget.**

```
-one rule, and §12 item 6 records what it costs `Run::salvage`.
+one rule, and U2-Z item 20 records what it costs `Run::salvage`.
```

`§12` has only 5 items at `6feb40a` (MEASURED: `git show 6feb40a:... | sed -n
'1498,1512p'` shows items 1–5 only; the owner table's own granularity table
confirms `§12 | 1498–1659 | 162 | split by item`, 5 items). **`§12 item 6` never
existed in the superseded document** — it is a broken citation, not a resolvable
one. The topic ("what it costs `Run::salvage`") matches `§15 item 20` exactly
(superseded §15 item 20: "`Run::salvage`'s documented ground does not hold under
`Staged`" — and the SAME paragraph, two sentences later, correctly cites `§15
item 20` for the same topic: "`Staged`. §15 item 20 takes the line, and §11
registers the test"). So the carve silently replaced a pre-existing dangling
citation (`§12 item 6`, pointing nowhere) with the correct one (`U2-Z item 20`,
i.e. what was `§15 item 20`) — a content correction, not a rename of a valid
referent into new namespace, and it is not one of the two exceptions the unit's
own header declares ("§2.2's config-count sentence… and §12 item 2's rate
list…"). The unit's header claims "No sentence of §3, §5 or §14 was rewritten,
extended or re-derived, and no number moved" — a number did move here (a
different section AND a different item number), undisclosed at the point of
occurrence unlike the two declared B5/§12-item-2 exceptions.

**Finding F1 (MINOR).** `docs/experiments/U2_node_protocol.md` §5.4, one
citation ("§12 item 6" → "U2-Z item 20") is a silent content correction, not a
cross-reference retarget, and is undisclosed against the unit's own "verbatim
except two stated exceptions" claim. **Severity note:** the correction is
content-improving (it fixes a pre-existing dangling reference in the superseded
text — `§12 item 6` pointed at nothing) and touches no design claim, matrix,
test, or measured number. Not BLOCKING on that basis, but it is real and should
be disclosed the way the B5 and §12-item-2 exceptions are.

**Result: otherwise clean.** All 14 other hunks are legitimate retargets.

---

## Check 2 — MAJOR 8 status for any witness U2 owns

**MAJOR 8** (rev7 review): "§8.4's M4 and M6 witnesses are not positions a legal
game reaches" — turn 1 is one stone, every later turn is two, so P1's stone
count is odd at every turn boundary and P2's is even (CLAUDE.md rule 3); M4 (P1
= 8 stones) and M6 (P2 = 15 stones + seals) both violate this.

**§8 is U4's in full** (owner table: `§8 | 1007–1286 | 280 | **U4**`) — grepped
`docs/experiments/U2_node_protocol.md` for `M4`, `M6`, `mutation`, `witness`,
`sealed`: no M4/M6 mutation witness appears in U2. The two "witness" hits are
`WinWitness::Pair` (a type name) and M5-B's "promoted WITNESS LINE" (an option
in the M5 matrix, not a board position). **U2 owns none of the rev7 review's
named mutation witnesses.** Confirmed clean on the literal MAJOR-8 subject.

**But the check also asked for "any concrete board position offered as evidence,
or any claim resting on one," and U2 §5.3 has one the grep above does not catch
by name:**

> "**The two-ply realisation — VERIFIED on the shipped solver.** … Two disjoint
> sealed five-stone P1 rows, P2 to move with two stones: `phase0 cover =
> Minimal([Two { first: (4,4), second: (5,0) }])` …"

This is carried verbatim from `6feb40a` (confirmed: no diff hunk touches this
passage in check 1's diff) and illustrates D-257's "`Cover::Minimal` carries
SETS" point. **P1 has exactly 10 stones** (two disjoint five-stone rows).

**Verification (reproducer, run and then removed — not left in the tree):**

```rust
// crates/pistol-core/tests/zzz_throwaway_parity_check.rs (deleted after running)
#[test]
fn p1_stone_count_is_always_odd_at_a_turn_boundary() {
    let mut state = GameState::new_game();
    state.place(Coord::new(0, 0)).expect("turn 1: origin");
    let mut q: i16 = 1;
    for _turn in 2..=13u32 {
        let before_p1 = state.board().stones().filter(|(_, p)| *p == Player::P1).count();
        if state.to_move() == Player::P1 {
            assert!(before_p1 % 2 == 1, "P1 count should be odd before a P1 turn, got {}", before_p1);
        }
        for _ in 0..2 { state.place(Coord::new(q, 0)).expect("legal placement"); q += 1; }
    }
    ...
}
```

Output: `p1_before` sequence over 12 turns is `1, 1, 3, 3, 5, 5, 7, 7, 9, 9, 11,
11` — **always odd**, never once even, confirming CLAUDE.md rule 3's parity
consequence directly against `pistol-core`, not just by arithmetic. `git status
--short crates/pistol-core/tests/` is clean after deleting the throwaway file —
nothing left in the tree.

**P1 = 10 (even) at "P2 to move" cannot occur in any legal game** — the same
defect class MAJOR 8 named, on a third position, uncaught through 5 review
rounds. **Mitigating context, checked and worth recording:** the underlying
design claim this fixture illustrates ("`Cover::Minimal` returns SETS, not a
union, because the union is provably insufficient") is **already independently
established, abstractly, in landed `D-257`** ("three hot windows with empties
{a,b}, {b,d}, {d,e} have no one-cell cover… {a,e} covers nothing in the
middle") — an abstract, position-free example that needs no reachability claim
at all. So this fixture's illegality does not undermine the FILTERED-row
soundness argument or M5-E; it undermines only the honesty of "VERIFIED on the
shipped solver" attached to an unreachable board, which is the exact charge
MAJOR 8 levied at M4/M6's identical phrase.

**Finding F2 (MAJOR, by direct precedent with MAJOR 8).** `docs/experiments/
U2_node_protocol.md` §5.3, the "two disjoint sealed five-stone P1 rows" fixture
has P1 = 10 (even), unreachable at any turn boundary per CLAUDE.md rule 3 —
verified against `pistol-core`. It is a live instance of MAJOR 8's defect class
that MAJOR 8 itself did not catch (MAJOR 8 was scoped to §8.4's mutation
witnesses only) and that the owner table's OPEN carry-forward item ("M4's and
M6's mutation witnesses must become positions a legal game reaches — **U4**,
OPEN") does not cover, because this position is neither M4 nor M6 and is not a
mutation-testing witness.

---

## Check 3 — M5-E intact

**Matrix survives byte-for-byte** except the retargets covered under check 1
(`§12.4` → `**U3** §12 item 4` inside the strongest-attack quote; `§7.2` →
`WPQ_seed.md §7.2` in the M5-D row). Confirmed via the same diff: the 5-row
table (M5-A…M5-E), every cost cell, every failure-mode cell, the ADOPTED line,
and the strongest-surviving-attack blockquote are unchanged.

**MEASURED/ESTIMATED marks intact:** M5-A "**MEASURED** 49–101 ns"; M5-B
"**MEASURED** 22 PV firings… 0.005%"; M5-C "**MEASURED**: −17.27%… −47.25%";
M5-E "**MEASURED** equivalence over 145 158 positions… **ESTIMATED** from **U3**
§12 item 4's own MEASURED figures: −29.1% / −41.3% / −41.5%". All marks present,
none dropped, none added.

**Adopted option is still M5-E**, "supplied by the DECISION-RED-TEAM that killed
M5-A" — unchanged text.

**ADR line — NOT YET LANDED.** `grep -n "M5-E" docs/decisions.md` (both at the
pin and at current HEAD) returns exactly one hit, inside **D-305**, which cites
"M5 to M5-E" only as one of four data points in a base-rate finding about
option-matrix reversals — it does not reproduce or reference the strongest
surviving attack quoted in U2 §5.6, and is not the dedicated ADR line U2-Z item
5 describes. **No D-number exists for M5-E's own adoption record.** This matches
U2's own explicit self-disclosure: U2-Z's lead-in states "the eleven lines below
are this unit's own; none is a correction to a landed line, and **none has
landed**," and the unit's own header states "**THIS UNIT HAS NOT BEEN REVIEWED**
at this u-rev. A WP is not landable while its reviews are outstanding." Per the
architect's ruling that "anything the document marks as OPEN and correctly
scopes is OPEN, not a defect," **this is not a finding against U2** — it is an
accurately and explicitly disclosed OWED state, not a silent gap. Recorded here
only because check 3 asked to confirm the ADR line exists, and the honest answer
is that it does not yet, by the document's own admission.

**Result: matrix content PASS; ADR-line existence UNMET but correctly
self-disclosed as OWED, not a finding.**

---

## Findings summary

| # | Check | Severity | Summary | Status |
|---|---|---|---|---|
| F1 | 1 | MINOR | §5.4's "§12 item 6" silently corrected to "U2-Z item 20" — a content fix, not a retarget, undisclosed as an exception; content-improving, no design/measured claim affected | Verified (mechanical diff) |
| F2 | 2 | MAJOR | §5.3's D-257 pairing fixture ("two disjoint sealed five-stone P1 rows") has P1 = 10, an even count unreachable at any turn boundary per rule 3 — same defect class as MAJOR 8, not covered by the owner table's M4/M6 OPEN item, pre-existing (carried verbatim, not introduced by the carve) | Verified (pistol-core reproducer, run and removed) |
| — | 3 | n/a | M5-E's ADR line has not landed in `docs/decisions.md`; correctly and explicitly self-disclosed by U2 as OWED — not a defect | Confirmed, not a finding |

**Finding count: 2 (1 MINOR, 1 MAJOR).**
