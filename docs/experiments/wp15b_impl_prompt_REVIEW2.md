# WP-1.5b IMPL governing prompt — REVIEW 2 (fresh context, scoped repair check)

## Header

**Pinned revision.** `f3752c393d49e9e446cf78eb6b81493553619a8a`.

```
$ git rev-parse HEAD            # at entry
f3752c393d49e9e446cf78eb6b81493553619a8a
$ git status --porcelain        # at entry
(empty)
```

**At exit:**

```
$ git rev-parse HEAD
f3752c393d49e9e446cf78eb6b81493553619a8a
$ git status --porcelain | wc -l
0
```

**Match with HEAD:** the pinned revision still matches HEAD at entry and exit;
the tree is clean at both. This session wrote exactly one file — itself.

**Scope, as given.** Not a fresh full re-read. A closure check of the 13
findings in `docs/experiments/wp15b_impl_prompt_REVIEW.md`
(pinned `cc6046121e23592454cd0e4f55d038dc8857518e`) against the repair landed
at the pinned revision above (`git diff cc6046121e23592454cd0e4f55d038dc8857518e
f3752c393d49e9e446cf78eb6b81493553619a8a -- docs/wp15b_impl_prompt.md`), plus a
scope-creep check on that diff and a D-346 self-state-claim check. The ~15
"VERIFIED WITH NO FINDING" items and the 4 "REJECTED" items from the first
review are out of scope and were not re-litigated.

---

## VERDICT

**PASS**

All 13 findings (MAJOR 1–6, MINOR 1–7) are closed against their primary
sources. The MAJOR 2 hotspot-substitution check was run directly:
`grep -c pistol-solver crates/pistol-search/Cargo.toml` → `0`;
`ls crates/pistol-search/src/` contains no `staged.rs`; `grep -n "D-263"
docs/decisions.md` shows D-263 unamended; no ADR line names a "Tier-T cell
extraction" hotspot. The substitution is genuinely NOT landed, and the
document's current §4 text says exactly that (`IMPL's registered hotspot is
therefore the cover arithmetic, per D-263, as landed`), matching reality
rather than asserting the opposite. The full diff since the reviewed revision
introduces no normative claim beyond what closing the 13 findings required.
D-346 compliance holds: the struck universal and its three cited siblings are
gone, and no new self-state claim was introduced.

---

## Per-finding results

MAJOR 1 — CLOSED. §5's `SearchInfo.stages` bullet no longer asserts "WP-1.6
(quiescence) blocks on this seam existing." It now reads: "Stage Q's own
widening-rate counters DEFER with stage Q (`WPQ_seed.md` §7.2); a later WP
(WP-1.5c, which owns the deferred quantities) reads them through this same
seam, so the seam itself is not deferred with them — this seam carries no
WP-1.6 dependency (`U2_node_protocol.md` §U2-M item 2; no source names one)."
Re-checked against U2:652–660 ("the counter seam described above is what a
later WP reads them through, so the seam is not deferred") and
`docs/ROADMAP.md`:125–126 (WP-1.5c "does not block WP-1.6") — both confirm
"a later WP" is WP-1.5c, matching the current text.

MAJOR 2 — CLOSED. §4's heading dropped "corrected by `U3_tier_t.md` §U3-M item
4." The lead no longer says "both already landed and neither IMPL's to redo";
it now states the substitution is unamended, cites the same MEASURED tree
checks the first review ran (`grep -c pistol-solver crates/pistol-search/Cargo.toml`
→ `0`, no `staged.rs`, `docs/ROADMAP.md`:89–92 still names the cover
arithmetic), and concludes "IMPL's registered hotspot is therefore the cover
arithmetic, per D-263, as landed." M5-E is now described correctly as "an
adopted design decision that deletes the redundant query pair ... IMPL writes
this deletion ... it is not a separate hotspot" (not a second landed thing).
A new paragraph, "ADR debt, owed and not IMPL's to close silently," names
U3-Z item 7 as MEASURED/OWED/NOT LANDED, states the document does not decide
whether the re-measurement replaces D-263's hotspot, and gives IMPL an
explicit fallback rule (bench cover arithmetic unless the ADR lands first).
§6 gained a new bullet naming this ADR debt, and §8 item 3 now reads "§4's
cover-arithmetic hotspot (or, if the §4 ADR debt has landed by then, the
substituted Tier-T-extraction hotspot)." I independently ran the two
prescribed commands and confirm the substitution genuinely is NOT landed:
`grep -c pistol-solver crates/pistol-search/Cargo.toml` → `0`; `ls
crates/pistol-search/src/` → `candidates.rs error.rs fallback.rs info.rs
lib.rs ordering.rs params.rs position.rs pv.rs pvs.rs score.rs search.rs
stop.rs tt` (no `staged.rs`); `grep -n "D-263" docs/decisions.md` shows the
line unamended; a grep for "Tier-T cell extraction" in `docs/decisions.md`
returns nothing. The document's text matches this reality.

MAJOR 3 — CLOSED. §8's "Finish does not require" list no longer lists the
SEAM decision as "not blocking." A new paragraph, "Finish DOES require
§3.2/§3.3's differential-gate SEAM decision to land, with no excusing clause,"
now states explicitly: "If the SEAM is still open, IMPL is not finished —
full stop." §3.3 gained a matching sentence: "Because §8's finish policy
requires all four gates wired and green with no excusing clause, IMPL's
finish is itself blocked on the SEAM decision landing — this is not a defect
in either §3.3 or §8, it is the binding consequence of requiring the fourth
gate unconditionally." This is the finding's second offered resolution
("hold item 2") rather than the first (rescope item 2 to three gates); that
is a valid alternative per the review's own instruction that the finding's
fix scope allows either. The direct textual contradiction the finding found
(exclusion list said "not blocking" while item 2 required all four) is gone.

MAJOR 4 — CLOSED. §3.4's pre-registered consequence is now explicitly "two
branches." Branch 1 is the original C→B replacement text. Branch 2 is new:
"If the instrument is GREEN while mutation M7 (`U4_soundness_instrument.md`
§8.4; Tier T at ≥3 for the mover — option A) also SURVIVES, the instrument
has demonstrated it cannot tell A from C, C's entire ground is unmeasured,
and that is recorded as such in the results rather than read as a
confirmation of C." The referent for "the soundness instrument" is now "§3.3
above, U4 §8 in full" (previously wrongly "(§3.2)"), matching U3 §6.5's own
referent (U4 §8, the whole four-part gate).

MAJOR 5 — CLOSED. §2 (build order) gained a second binding clause, promoted
from §6's pointer list: "rule 5 is undischarged for the node protocol itself
... This is a rule-5 registration the architect must place before U2's IMPL
starts, not a repair IMPL or a carve may write. IMPL does not begin U2's
shape until this registration lands." §6's U2 bullet was trimmed to the
remaining M5-E figures item, with a pointer: "The node-protocol rule-5
registration is promoted to §2's binding order, not carried here."

MAJOR 6 — CLOSED. The struck universal ("Every claim below cites its primary
source; a reader who wants a finding's content or a design's argument reads
it there, per D-331's fold law") is gone from the header — confirmed absent
by grep. Its three cited siblings are also gone: ":9–10" ("this document does
not repeat that mapping") is now just "... part of the superseded pre-carve
design." (full stop, no universal); ":52" ("None is restated beyond what
IMPL needs to build it") is replaced by the specific, checkable §3 lead now
verified under MINOR 4; ":226–227" ("Pointer-only, per unit's own OPEN list")
is replaced by "The argument for each item below is at its own unit's
citation, not here." A grep across the current document for "restated",
"universal", "self-state", "cites its primary source", and "fold law" finds
only line 6 ("REFERENCE material under D-351, cited here and not restated" —
pre-existing, not the struck sentence, not challenged by the original review)
and line 286 (a U3 OPEN-list bullet describing U3's own gap, not a claim
about this document). The two concrete claims MAJOR 6 named as falsified by
the universal (§4's "already landed", §5's WP-1.6 clause) are independently
fixed under MAJOR 2 and MAJOR 1 above.

MINOR 1 — CLOSED. "legality-agreement figure" → "cover-enumeration agreement
figure," matching D-323 condition (2)'s unadorned "agreement" and the
underlying figure's actual subject (cover enumeration, not legality).

MINOR 2 — CLOSED. §3.3's header parenthetical is now "(§8.3 and §8.2; wired
at §8.7)" — exactly the finding's prescribed text, correctly naming §8.2 as
the differential gate's specification site.

MINOR 3 — CLOSED. §3.1 no longer reads rung (a)'s silence as support for N-E.
It now states the shape (required `--config`, no default, whole-path guard),
that rung (a) was silent "because all three rows already satisfy it
(MEASURED: none of the three has a default, all refusing an absent `--config`
by name at exit 1)," that rung (a) therefore does not distinguish N-E, and
that "N-E's actual ground is rung (b), at D-329" — matching D-329's own
correction.

MINOR 4 — CLOSED. §3's lead now reads: "Three carry a landed ADR line (§3.1
D-329, §3.2 D-323, §3.3 D-316) with their argument at U4's own §8/§9; §3.4's
is GATED (`U3_tier_t.md` §U3-Z item 2, not yet written) and its argument
lives at `U3_tier_t.md` §6.1/§6.5" — the finding's prescribed text almost
verbatim.

MINOR 5 — CLOSED. Both debt pointers changed from §5 to §6: §3.2's SEAM
sentence now reads "carried as debt in §6 below," and §3.4's gated-ADR
sentence now reads "carried as debt in §6 below."

MINOR 6 — CLOSED. §3.4 now reads "Config keys `tier_t_own_count = 2`,
`tier_t_opponent_count = 3` (the threshold reading — own windows qualify at
count ≥ 2, opponent windows at ≥ 3; not the exact reading — `U3_tier_t.md`
§6.1)" — the literal key spelling (`= 2`, `= 3`) is now stated, with the ≥
semantics separated out as required by the finding.

MINOR 7 — CLOSED. §5's fourth bullet no longer claims the four documents are
what "§9's OPERATOR-CONFIRM slots need" in general. It now reads: "These four
documents are what `docs/experiments/wp15b_sprt_prereg.md` §9.4's
soundness-gate slot and the snapshot's AFTER measurement need ... §9.7 is a
separate slot — the revision at which `tools/baseline_snapshot.sh` accepts
`--config` (§3.1 above) — a REVISION, not a config-document need."

---

## Part (b) — scope creep check

Every hunk in `git diff cc6046121e23592454cd0e4f55d038dc8857518e
f3752c393d49e9e446cf78eb6b81493553619a8a -- docs/wp15b_impl_prompt.md` maps to
exactly one of the 13 findings (mapping enumerated during review: header →
MAJOR 6; new §2 clause → MAJOR 5; §3 lead → MINOR 4; §3.1 → MINOR 3; §3.2
item 2 → MINOR 1; §3.2/§3.4 debt pointers → MINOR 5; §3.3 header → MINOR 2;
§3.3 finish-blocked sentence → MAJOR 3; §3.4 rewrite → MINOR 6 + MAJOR 4; §4
full rewrite → MAJOR 2; §5 fourth bullet → MINOR 7; §5 stages bullet → MAJOR
1; §6 heading → MAJOR 6 sibling; §6 U2 bullet → MAJOR 5; §6 new ADR-debt
bullet → MAJOR 2; §8 item 3 → MAJOR 2; §8 exclusion list and new "Finish DOES
require" paragraph → MAJOR 3). No hunk touches unrelated document sections.

One elaboration worth flagging, not as scope creep but for the record: the
reverted §4 EXPECTED GAIN BRACKET / ABORT THRESHOLD / INSTRUMENT text is new
prose (D-263 itself registers no bracket or threshold, so reverting to the
cover arithmetic as hotspot required the document to say what IMPL derives
instead). The INSTRUMENT line's "taken at both stone counts the arena plays
from" is not literally prescribed by MAJOR 2's fix scope, but it is the
project's standing bench convention (D-114's "fixed-node runs at two stone
counts," reused at D-146, D-255) rather than an invented fact, and it is a
necessary consequence of correctly reverting the hotspot substitution (the
old, now-removed text specified "BATCHED nodes only," which was Tier-T's
population, not cover arithmetic's). This is required to keep §4 internally
executable after the revert, not an independent new normative claim.

## D-346 compliance

The struck universal at the header and its three cited siblings (`:9–10`,
`:52`, `:226–227` in the reviewed revision) are all gone from the current
text, confirmed by grep for their exact language. No new self-state claim
(a claim about this document's own completeness, citation practice, review
status, or revision history) was introduced. Two new sentences that discuss
the document's own internal consistency were checked and are not the flagged
class: §3.3/§8's "this is not a defect in either §3.3 or §8, it is the
binding consequence of requiring the fourth gate unconditionally" is an
editorial gloss on a specific, checkable design interaction (parallel in kind
to the pre-existing, previously-VERIFIED §2 sentence "Neither ordering is a
defect in either unit"), not a universal about the document's citation
practice, completeness, or review history — the class D-346 and MAJOR 6 target.

---

*Review complete. Pin `f3752c393d49e9e446cf78eb6b81493553619a8a`, matching
HEAD, tree clean, at both entry and exit.*
