# REVIEW-design (Phase 1', new eyes) — `docs/wp16_quiescence_design.md` revision 3 (WP-1.6)

**Revision reviewed:** `a3b9e3749ccc4e277dc960eaa3d20a47fe76c532` (`a3b9e37`).
**Matches HEAD:** YES — `git rev-parse HEAD` = `a3b9e3749ccc4e277dc960eaa3d20a47fe76c532`;
working tree clean apart from this report, which is untracked.
**Reviewer:** fresh-context REVIEW-design subagent. NOT the reviewer of revision 1
(`wp16_design_REVIEW.md`) and NOT the reviewer of revision 2
(`wp16_design_REVIEW_rev2.md`) — the "new eyes" slot the resume dispatch asked for
after two consecutive FAILs. I read both prior reports ONLY to learn which failure
modes are already on record; this is a full first-principles review of revision 3,
not a diff-review.

**Sources read directly, never through the document's or a prior review's
paraphrase:** `docs/wp16_quiescence_design.md` (all 594 lines);
`docs/research/threat_calculus_v1.md` (whole); `docs/decisions.md` D-3, D-7, D-19,
D-74, D-75, D-104, D-105, D-111, D-117, D-129, D-267, D-287, D-310, D-352, D-374,
D-386, D-388, D-389, D-390 (verbatim);
`crates/pistol-search/src/{pvs.rs,staged.rs,pv.rs,info.rs,candidates.rs,search.rs,position.rs}`;
`crates/pistol-search/src/tt/{mod.rs,entry.rs}`;
`crates/pistol-solver/src/{query.rs,cover.rs}`;
`crates/pistol-engine/src/config.rs`; `crates/pistol-search/src/params.rs`;
`docs/experiments/U3_tier_t.md` §6.1–§6.3 and `crates/pistol-solver/tests/wp15b_census.rs`;
every `configs/*.toml`. The `Entry` layout claim was checked by **compiling it**, not
by reasoning about it (see §6 below).

---

## VERDICT: **FAIL** — STOP. Per the resume dispatch, no revision 4 inside this session.

Four CONFIRMED blocking defects (B1–B4). Two of them — B1 and B2 — are the same
defect *class* that failed revisions 1 and 2: a path IMPL cannot build without
inventing a rule the document does not state, on the branch the design's own narrow
width creates. B1 in particular is not a corner case: it is the **modal defensive
line** the gate is designed to fire on.

What is genuinely good, and should not be re-litigated by whoever resumes: the
revision-2 defect is closed (the move set is stated exactly once and nothing
contradicts it — see §3 below); the `depth_plies: 0` finding is TRUE; the byte-budget
arithmetic §6 asks this reviewer to check independently is **CORRECT and I compiled
it**; D-388 is quoted character-for-character with nothing moved; §9's arithmetic
is right; the D-267 `LAW-RIPOSTE`/`LAW-LEDGER` attribution is right; every `file:line`
citation I spot-checked (roughly forty of them) resolves.

---

## Section-by-section assessment

### §1 Problem — accurate, with one silent deletion (see M3)

The history is stated correctly and the revision-2 failure is characterised the way
`wp16_design_REVIEW_rev2.md` NEW-1 actually characterises it. The claim "This is
stated exactly once, in §3, and nowhere else in this document restates or contradicts
it" is TRUE — verified by reading all 594 lines and by grepping every occurrence of
`staged_candidates`, `within_radius`, `candidate_cells` and `tier_t` in the document.
Every remaining mention is either historical (about revisions 1–2) or an explicit
exclusion. **The defect that sank revision 2 is closed.**

What §1 no longer contains is the `PROTO-NODE` step 5 quotation revision 2 nominated
as "the registered cure". Deleting it is one way to answer C1/NEW-3, but it is not the
way D-389 asked for. See **M3**.

### §2 TURNS invariant — correct; `pvs.rs:199–217` verified verbatim

The `depth_plies == 0` branch is exactly as described: `debug_assert!` on
`Phase::First`, then `return self.position.value()`, with no threat query and no
candidate generation. Stage F's win-now check really does live inside
`depth_plies > 0` only. §2.1's gap is real.

### §3 / Ruling 1 — the move set is stated once, and is genuinely `staged_candidates`-free

Structurally sound. §3's dedicated-function shape (`Run::quiescence` in a new
`crates/pistol-search/src/quiescence.rs`) is implementable against `pvs.rs` as
described, and the argument for abandoning revision 2's in-place rebind — that the
rebind's only motivation was `staged_candidates`'s width, which Ruling 1 forbids — is
correct.

**§3.1 (trigger a)** — query and citations correct. `mate_in(turns_from_root + 1)` is
the right distance (matches `pvs.rs:318`, where the turn number is read after the
stone is placed and both plies of a turn share it). Two omissions, neither blocking:
the design returns the mate score WITHOUT the witness cell, so the reported PV stops
one stone short of the mate it claims (contrast `pvs.rs:311–321`, which promotes the
winning cell); and it does not bump `seldepth_turns`, which `pvs.rs:316–319` bumps
explicitly for exactly this case with a documented reason.

**§3.2 (trigger b, Ruling 2)** — the `Cover` classification is correct against
`cover.rs:201–245`: one-cell covers are enumerated first and are automatically
inclusion-minimal, so `covers.iter().any(matches!(_, MinimalCover::One(_)))` really is
the `t ≤ 1` test and `.all(..)` really would under-fire. The `Cover::cells()` ply-1
set including `MinimalCover::Two` members is sound **and I re-derived the composition
independently**: with families `{a,b}`/`{a,c}`, after ply-1 plays `b` the ply-2
`blocking_covers(us, HitBudget::One)` at the new position returns `Minimal` containing
`One(c)`, so `c` is generated. The two plies compose into a legal cover. Good.

**§3.3 (trigger c)** — the `LiveCount::Two` defect is genuinely gone and the reasoning
from `DEF-PLAN` and `NearHot`'s own closure is correct. Two citation defects: **M4**
(the `23.29/…` figure is attributed to a population it was not measured over) and
**M7** (`live_cells_at_count` is D-352's map entry, not D-267's).

**§3.4** — the gate-node (`Phase::First`) case is sound and its PV argument holds.
Dropping `is_pv` is fine *there*. It is not fine where §3.5 reuses it — see **B2**.

**§3.5 (ply-2)** — **this is where the design breaks.** Two blocking defects, **B1**
and **B2**, plus a precision defect, **M9**. The offensive-trigger walkthrough I ran
is coherent (ply-1 makes one window hot, ply-2's `cells_raising_to_hot` re-query is
exactly the cells that make a *second* window hot, giving the fork §3.3 describes) —
the defect is not in the offensive reading, it is in the defensive one and in the
`Impossible` early return.

**§3.6** — the legality argument is correct and correctly delegated.

**§3.7** — the `LAW-RIPOSTE` argument (quiescence is not a prover, the riposte check
falls out of recursion, budget exhaustion is an admission of ignorance rather than an
unsound proof) is sound, and is the right answer to the calculus's own "truncated
riposte chains" unsoundness-perimeter item (`threat_calculus_v1.md:147–148`). The
`LAW-LEDGER` reading is now the right way round (t=1 banks one free stone; §3.5 is its
spend). **Does the t≤1 gate + ply-2 recovery discharge what D-267 assigns this WP?**
For `LAW-RIPOSTE`, yes as far as an alpha-beta search can — but only if §3.5 actually
generates a ply-2 set, which **B1** shows it often does not. For `LAW-LEDGER`, the
argument is now present and correct in prose, and is expressed in the design by §3.5's
unconditional offense union — which is the same clause **B1** breaks. So the
discharge is contingent on the defect, not independent of it.

**The one case I looked hardest for and did NOT find:** a `t = 2` opponent family the
t≤1 gate refuses and the ply-2 rule cannot recover. The gate refuses `Minimal` with no
`One`, so the extension never starts — but that is a decision not to *extend*, not an
unsound score: the node stands pat on a static read, which is an admission of
ignorance, exactly as `q_budget` exhaustion is. Ruling 2's narrowing does not reopen
a soundness gap. It costs coverage, which is Phase 2's to measure.

### §4 Zones — VERIFIED structurally, independent of the document's prose

`within_radius` is `pub(crate)` with exactly two call sites in the whole workspace:
`candidates.rs:47–48` (both arms of `candidate_cells`) and `staged.rs:283` (the
quiet-ball safety net). `candidate_cells` itself is called from `pvs.rs:247` (the
`Radius` arm only), `fallback.rs:81,104`, `search.rs:346`, and tests. A new
`quiescence.rs` that imports neither `crate::candidates` nor `crate::staged` cannot
reach either. The claim that this now holds STRUCTURALLY rather than as a runtime
suppression is correct and is a real improvement over revisions 1–2. **PASS.**

### §5 Cap — the config list is wrong in both directions. See **B4**.

The schema-home reasoning is right and `deny_unknown_fields` really does apply
(`config.rs:150`, and the parent `SearchSection` at `:136`). The `0`-is-disable
semantics are clear. The list is not.

### §6 TT / Ruling 4 — the load-bearing item, split verdict

**The `depth_plies: 0` finding is TRUE.** `Table::store` (`tt/mod.rs:158–171`) packs
through `Entry::packed` (`entry.rs:113–123`), which calls `depth_fits`
(`entry.rs:167–179`), which ends in

```rust
assert!(
    depth >= 1,
    "pistol-search invariant {TT_FIELD_OUT_OF_RANGE}: a leaf is not worth an entry"
);
```

an **always-on** `assert!` (D-129's correctness class, not `debug_assert!`), reached
before any bucket is touched. `Entry::is_empty` really is literally
`self.depth_plies == 0` (`:138–140`) and the module doc really does say the depth
field doubles as the occupancy flag (`:9–11`). A `Record { depth_plies: 0, .. }`
handed to `Table::store` panics with `TT_FIELD_OUT_OF_RANGE`. Ruling 4's literal
"store at depth 0" is unimplementable. **The document's finding stands, and neither
prior review made it.**

**The byte-budget arithmetic is CORRECT — and I did not take it on the document's
word, I compiled it.** `Entry`'s declared fields are `verification: u64` (8) +
`best_q/best_r/score/static_eval: i16` (8) + `depth_plies: u8` + `bound_age: u8` (2)
= **18 bytes**, and `align_of::<u64>() == 8` forces the size up to **24**, matching
`ENTRY_BYTES` and the const assertion at `entry.rs:100–103`. Six tail-padding bytes
exist. I built the current struct and three variants with `rustc -O --edition 2024`
under `#[repr(Rust)]` (so field reordering is in play, exactly as it is in the real
type):

```
now       size=24 align=8      <- current Entry
withflags size=24 align=8      <- + flags: u8   (the proposal)
plus6     size=24 align=8      <- + six u8 fields
plus7     size=32 align=8      <- the seventh breaks it
bucket now = 96   bucket flg = 96   <- BUCKET_BYTES unchanged
```

So: adding `flags: u8` keeps `size_of::<Entry>() == ENTRY_BYTES` and
`BUCKET_BYTES == 96`, `GENERATIONS` and `bound_age`'s packing are untouched, and
`Table::new`'s sizing arithmetic and D-75's entry-count claim are unaffected. There
is headroom for six such bytes; the seventh trips the existing const assertion, which
is a genuine compile-time gate and not a new one. **`§6`'s single highest-risk
unverified claim VERIFIES.** IMPL will not discover this the hard way.

**The store rule composes soundly.** `victim()` returns the position's own slot when
one matches, so a full-width record and a quiescence record for the same key can never
coexist; a declined quiescence store leaves the full-width entry intact (this closes
`wp16_design_REVIEW_rev2.md` NEW-5 properly); a full-width store still evicts freely.
Determinism is unaffected: the new condition is a pure function of the bucket's
contents and the record, with no clock, no hash-iteration order and no thread — the
`tt/mod.rs` "# Determinism" contract and CLAUDE.md rule 4 both survive. I found no
determinism concern anywhere in §6, §7 or §8.

**What §6 gets wrong: B3** — by its own two read rules, nothing can ever read a
quiescence entry, and §6 never says what one contains. Plus **M2** (the flag has to
live on `Record`, not only on `Entry`).

The "excluded from main-depth node accounting" reading is right, and the refusal to
exclude `self.nodes` is *correct and well argued* — a search doing quiescence work
invisible to its own budget would misreport the compute D-386's SPRT was matched on.
Good.

### §7 Correctness — mostly carried forward correctly; one omission

Win detection, scores-in-turns, determinism and the alpha-beta discipline are all
stated correctly against the code. The determinism paragraph is right that
`can_win_this_turn`, `blocking_covers` and `cells_raising_to_hot` are total,
deterministic and sorted (`query.rs:1–12`), and that reusing `self.should_stop()`
rather than duplicating it keeps a reproducible stop exact.

The PV paragraph is where the omission is: it says `quiescence()` clears and promotes
"at the same points `visit`'s own candidate loop does" and never mentions that those
points index a **fixed-size** table. See **M1**.

### §8 Counters — sound

Fields are well chosen, the exclusion from the existing `StageCounters` fields is now
structural (quiescence cannot reach `StageCounters::record`, which only
`staged_candidates`' caller drives), and the defense/offense mutual exclusivity claim
is right for the right reason (`NothingToBlock` and `Minimal` are disjoint `Cover`
arms). Minor: §6's enumeration of the existing fields omits
`batched_quiet_safety_net` (`info.rs:49–55`).

### §9 Cost derivation — arithmetic correct, disposition honest, three sourcing defects

**D-388 is quoted verbatim with zero drift** — I had the ADR text extracted and
compared character by character against the block-quote: 339 characters, identical,
including `<= 2.0x`, `> 3.0x`, and `Numbers do not move.` Rule 5's "numbers do not
move" is honoured. **The arithmetic checks out**, every step:

| step | claimed | recomputed |
|---|---|---|
| defense per turn | `2.2 × (1+7) ≈ 18` | 17.6 ✓ |
| offense per turn | `4.5 × (1+7) ≈ 36` | 36.0 ✓ |
| corpus roots | ≈24 | `0.250×18 + 0.53×36` = 23.6 ✓ |
| r2 draw | ≈33 | `0.184×18 + 0.83×36` = 33.2 ✓ |
| r8 draw | ≈31 | `0.137×18 + 0.80×36` = 31.3 ✓ |
| playouts | ≈30 | `0.031×18 + 0.82×36` = 30.1 ✓ |
| Poisson `1−e^−λ` | 53/83/80/82 % | 52.8 / 83.1 / 80.0 / 81.9 % ✓ |

Every census value quoted (`:165`, `:168`, `:170`, `:172`, `:181`) reproduces exactly
at the cited line, and the four-column order is right.

The **disposition is honest** and I want that on record: the document reports a figure
that is 8×–17× *outside* its own registered ABORT threshold, says so in plain words,
declines to round it toward comfort, and refuses to move D-388's numbers. That is
exactly what rule 5's registration discipline is for, and it is the right disposition
under Ruling 3. It is not a defect. (It does leave a project-level question the
document does not answer — what WP-1.6 becomes if the ABORT fires as the design's own
model predicts — but that is the architect's, not this review's.)

The sourcing defects are **M4**, **M5** and **M8**.

### §10 Provenance — accurate

---

## CONFIRMED DEFECTS

### B1 (BLOCKING). §3.5's ply-2 candidate set can be EMPTY at `Phase::Second`, on the modal defensive line, and the design specifies no answer. This is D-104's `NO_CANDIDATES_MID_TURN` by name.

**The rule, verbatim (§3.5, lines 237–239):**

> Ply-2 candidates = `Cover2::cells() ∪ cells_raising_to_hot(us, NearHot::Three)`
> at the ply-2 position (deduplicated), unless `Cover2::Impossible` fired the
> early return above.

**The code.** `Cover::cells()` returns `Vec::new()` for **both** non-`Minimal` arms
(`cover.rs:109–112`):

```rust
let mut cells: Vec<Coord> = match self {
    Cover::NothingToBlock | Cover::Impossible => Vec::new(),
    Cover::Minimal(covers) => …
};
```

**The reachable line, walked concretely.** The gate fires trigger (b) — so
`blocking_covers(us, HitBudget::Two)` returned `Minimal` containing some
`MinimalCover::One(a)`, i.e. one cell hits every opponent hot window. Ply-1 plays `a`,
which is the ordinary, best, and usually first-ordered defensive stone. A defender
stone in a window kills its liveness, so `hot_windows(opponent)` is now empty and
`Cover2 = blocking_covers(us, HitBudget::One)` returns `NothingToBlock`
(`cover.rs:203–205`) → `Cover2::cells()` is **empty**. If the mover owns no live
count-3 window, `cells_raising_to_hot(us, NearHot::Three)` is **also empty**
(`query.rs:187–192` fills from `live_windows_at_count(us, LiveCount::Three)`, which is
then `&[]`).

**The union is empty, at `Phase::Second`, with a stone still owed.**

This is not a corner. `U3_tier_t.md:165` (`live-3 own`, MEASURED) gives a mean of
**0.75** live-3 windows per position at corpus roots — the document's own §9 uses that
same number to estimate a ~47 % chance of *zero* such windows. A defensive extension
that plays the single blocking cell and owns no live-3 window is the majority
defensive shape, not an edge.

**Why it is blocking.** `pvs.rs:479–486` is a release-active `assert!` on
`Phase::First` for exactly this state, and D-104 (`decisions.md:236`) is the ADR that
put it there:

> at phase 1 it is not: the node returns with its principal-variation slot cleared,
> the phase-0 parent promotes a line of one ply, and the line that reaches the root
> ends on a lone stone, which `turns_from_plies` refuses with `PV_NOT_PLAYABLE` …
> **There is no score that fixes it, because the missing thing is not a value but the
> mover's second stone.**

D-104's flip clause names this WP's obligation exactly: it "flips when a candidate
policy that can run dry mid-turn arrives, **which must answer inside a turn** and
would replace this assertion with that answer." Revision 3 ships precisely such a
policy and does not answer inside the turn. IMPL is left with three options and the
document licenses none: panic (`NO_CANDIDATES_MID_TURN`, on an ordinary position);
return a static value mid-turn (D-111's `STATIC_EVAL_MID_TURN`, which §2 forbids); or
widen to some fallback — and the only two fallbacks the codebase has are
`staged_candidates`' quiet-ball safety net (`staged.rs:276–285`) and `within_radius`,
both of which Ruling 1 and §4 forbid by name. **This is the revision-2 defect class
exactly: a specification whose available readings differ materially and which
licenses none of them.**

### B2 (BLOCKING). §3.5's `is_pv`-free mate-band return at ply-2 produces a half-turn principal variation → `PV_NOT_PLAYABLE`. §3.4's PV argument is made for the gate node and applied to a node that is not one.

**§3.4, verbatim:** "return `-mate_in(turns_from_root + 2)` directly, **unconditionally**,
with no candidate generation and **no dependence on `is_pv`** … a line ending at a turn
boundary is turn-whole regardless (`pv.rs:76–79` …) — **neither applies at a gate**."

**§3.5, verbatim:** "`Impossible` → **'empty hitting set = losing band, no search'
applies again, symmetrically**: return `-mate_in(turns_from_root + 2)`, no further
generation for this branch".

§3.4's argument is stated *about a gate node*, and it is correct there: a gate node is
at `Phase::First`, so a line ending there replays into whole turns. §3.5 applies the
same return at the **ply-2 node, which is at `Phase::Second`** — a turn half played.
The argument does not transfer, and the document does not re-make it.

**The mechanism, traced in code.** The ply-2 node clears its PV slot on entry and
returns without generating, so `pv.lengths[ply+1] == 0`. Back at the ply-1 node, the
`same_side` child is not negated (`pvs.rs:396–402`), so the score arrives as the
mover's own `-mate_in(k+2)`, and `best_score` starts at `-INFINITY` (`pvs.rs:295`).
Wherever the gate node's incoming `alpha` is `-INFINITY` — which is the root's first
candidate chain and every PV node that has not yet improved — `score > alpha` holds and
`pv.promote(ply, at)` fires (`pvs.rs:339–341`), writing a **one-ply** line. If that
branch is the node's best (and in a genuinely lost position it is), the dangling ply
rides to the root, where `search.rs:217` calls `turns_from_plies(state, run.line())`
and `pv.rs:101–105` panics:

```rust
assert!(
    pending.is_none(),
    "pistol-search invariant {PV_NOT_PLAYABLE}: the line ends with turn {} half played",
    state.turn()
);
```

**The engine avoids this today by the exact mechanism §3.4 discards.**
`staged_candidates` gates `StagedRow::OverloadReturn` on `!is_pv` (`staged.rs:201`),
and `StagedRow::BatchedLost`'s own doc (`staged.rs:85–89`) says why: "the position IS
lost, but a **PV node must return the line that proves its score**, so generation
proceeds rather than returning early." A root PV is composed of PV nodes all the way
down (a null-window child that beats alpha is re-searched with the full window,
`pvs.rs:420–423`), which is precisely what makes the existing `!is_pv` gate sufficient.
Removing it inside quiescence, at a `Phase::Second` node, removes the guard and
reintroduces the panic D-104 was written about.

Revision 2's reviewer independently re-derived §3.4's PV argument and **found it sound
— for the gate**. Revision 3 cites that re-derivation ("PV integrity note carried
forward from the prior review's independent re-derivation") as licence for the ply-2
case, which the prior review never examined because revision 2 had no ply-2 rule.

### B3 (BLOCKING as under-specified). §6's two read rules make every quiescence TT write unreadable, and §6 never says what a quiescence `Record` contains.

**§6 item 4, verbatim:** "a probe returning a record with `from_quiescence: true` is
treated by a FULL-WIDTH caller … exactly as if `probe` had returned `None` — no
cutoff, no move-ordering hint, full stop. `quiescence()` itself **never probes the
table at all** — it only ever WRITES, once per node it visits".

Those are the only two consumers there are. Taken together, **no code path anywhere
can ever read a quiescence entry.** The write is therefore pure cost: a `Table::store`
call per quiescence node (key, pack, four-slot victim scan), permanent occupancy of a
bucket slot that only a full-width store or another quiescence store can reclaim,
`used` inflation and therefore `hashfull_permille` inflation — all of it landing on
`ttd`, which D-388 registers as the PRIMARY metric. §6 states neither consequence; it
presents the probe rule as "the one place this document deliberately gives up a
possible optimization", which understates it: the rule does not forgo an optimization,
it strands the entire store.

The blocking half is narrower and concrete: **§6 never states what a quiescence
`Record` is.** It fixes `depth_plies: 1` and `from_quiescence: true` and says nothing
about `score`, `bound`, `static_eval` or `best`. A stand-pat node has no best move at
all; a node that returned §3.4's mate band expanded no child. `Record::best` is a bare
`Coord` with no absent value, and `Entry::packed` requires all of them
(`entry.rs:113–123`). IMPL cannot write this store without inventing the record.

Either resolution is one sentence — "quiescence does not store" (the honest reading of
what the rules already accomplish, and it deletes the `Entry` change, the `victim()`
change and the probe change with it), or a stated record plus a stated reader. The
document picks neither.

### B4 (BLOCKING). §5's config list is wrong in both directions; a landing commit built to it fails config validation and turns twelve tests red.

§5 claims the list is "unchanged list from revision 2, **re-verified against the tree
at this revision**". It is not.

**Three named files cannot carry the field.** `configs/arena_wp15b_staged_vs_r2.toml`,
`configs/arena_wp15b_staged_vs_r2_confirm.toml` and `configs/arena_wp15b_dryrun.toml`
have **no `[search.candidate_policy]` section at all**. They are dispatched to the
ARENA schema by basename (`tools/config_check.sh:56–63`) and reference engine configs
by path (e.g. `arena_wp15b_staged_vs_r2.toml:94` `config = "configs/instrument_staged_v0.toml"`,
`:103` `config = "configs/instrument_v0.toml"`). Adding `q_depth_turns` to them is
rejected by `deny_unknown_fields`. They inherit the field automatically through the
two engine configs already on the list. This is a correctness error, not redundancy.

**One staged document is omitted.** `crates/pistol-engine/tests/common/mod.rs:43–70`
is `pub const VALID_STAGED: &str`, a complete instrument-mode TOML document with
`kind = "staged"` at `:53` and the five keys at `:54–58`. It is the fixture
`crates/pistol-engine/tests/config_validate_tests.rs` mutates for all twelve staged
validation cases. A required, no-default field lands and every one of them fails on a
missing-field error. It is the only `kind = "staged"` document anywhere outside
`configs/`.

**The `params.rs` half is unaccounted for.** §5 also adds the field to
`StagedParams`, whose construction sites are `crates/pistol-engine/src/instance.rs:181`
plus six `pistol-search` test files (`tests/common/mod.rs:74`, `staged_tests.rs:88`,
`staged_colony_family_tests.rs:122,151`, `staged_differential_gate_tests.rs:126`,
`staged_pattern_fixture_tests.rs:51`, `staged_tier_t_threshold_tests.rs:96`). None
are named.

**The corrected list is five `configs/*.toml`**, not eight:
`gate_staged_v0.toml`, `instrument_staged_v0.toml`, `instrument_v0.toml`,
`play_staged_v0.toml`, `tactical_staged_v0.toml`. (`instrument_v0.toml` really is
staged now, per D-386/`9282dd0` — the document is right to include it.) Plus the
`VALID_STAGED` fixture and the seven `StagedParams` sites above. A judgement call worth
naming separately: `config.rs:159–160` nominates `U3_tier_t.md` §10 (`:412–424`) as
"this document's schema, the one place the count of staged documents and their shape
is stated" — leaving it at five keys while the code has six puts the source of record
out of sync with its own citation.

---

## Non-blocking findings

**M1. `MAX_PLY`, the fixed-size `PvTable`, and `seldepth_turns`' documented contract.**
`Run::new` builds `PvTable::new(MAX_PLY)` (`pvs.rs:131`, `search.rs:204`) with
`MAX_PLY = 2 * MAX_DEPTH_TURNS + 2 = 130` (`search.rs:64,67`). `PvTable::clear`
indexes `lengths[ply]` and `promote` indexes `lengths[ply+1]` on a `Vec` of length
131, with `lines` of `130 × 130` (`pv.rs:34–63`). A granted extension adds up to
`2 × q_depth_turns` plies past the horizon, and §5 validates `q_depth_turns` in
`0..=8` — up to 16 plies. At the deepest iterations a quiescence node indexes past the
table and takes an unnamed `index out of bounds` panic, which is exactly the silent,
symptom-naming failure rule 3 forbids. The document never mentions `MAX_PLY`; IMPL
must either raise it or bound the extension. Separately and in the same place:
`SearchInfo::seldepth_turns`' own doc (`info.rs:105–107`) states "Equal to
`depth_turns` in a completed iteration — **Stage 0 has no extension that passes the
horizon**". WP-1.6 falsifies that sentence and the design never names it; under rule
10 the amendment belongs in the design, not discovered by IMPL.

**M2. `from_quiescence` has to live on `Record`, not only on `Entry`.** §6.2 adds it to
`Entry`; §6.4 speaks of "a probe returning a record with `from_quiescence: true`".
`Table::probe` returns `Option<Record>` and `Entry::packed(verification, record,
node_score, generation)` reads every field from the `Record` it is handed
(`tt/mod.rs:149–171`, `entry.rs:113–123`). So the `pub` `Record` type gains the field
too, and every construction site — including `pvs.rs:363` — is touched. §6's stated
resolution does not typecheck as written.

**M3. `PROTO-NODE` step 5 is never cited, and Ruling 2's gate is its literal opposite.**
The calculus's one normative sentence about this WP is `threat_calculus_v1.md:135`:
"**Quiescence.** Threat-only, zone-bounded (Tier F + Tier T with **t ≥ 2**), never
full-width (was S3)." Ruling 2 gates at **t ≤ 1** only. Revision 1's C1, revision 2's
NEW-3, and D-389's own "what would unstick it" item (3) all demanded the same thing:
"§1's `PROTO-NODE` step 5 `t >= 2` citation is either adopted as the trigger set's
literal reading **or replaced with** the LAW-FORCE/LAW-RIPOSTE citation the triggers
actually rest on". Revision 3 discharges it by deleting the citation — step 5 is not
mentioned anywhere in the document. §3.2 does ground ply-1 on `LAW-FORCE`, which is
the substance of the second option, but the divergence from the design source of
record is never named. CLAUDE.md makes the report and the calculus the prior, and rule
10 makes an unnamed divergence a breach to be answered with an ADR line rather than a
deletion. One sentence closes this.

**M4. §3.3's `23.29 / 31.50 / 30.26 / 48.73` is attributed to a population it was not
measured over, under a MEASURED label and a claim of direct verification.** §3.3 says
the trigger is "reachable only on a `Cover::NothingToBlock` row … which is the
`BATCHED nodes` row — 70.8% / 61.5% / 65.5% / 92.5% — and **on those nodes** Tier T …
averages 23.29 / 31.50 / 30.26 / 48.73 cells", labelled "MEASURED, `U3_tier_t.md` §6.2
census (verified directly against the file at this revision, not through a review's
paraphrase)". `U3_tier_t.md:181` (`option C — Tier T (threshold, ADOPTED)`) is an
**unconditional** mean over every sampled row — `wp15b_census.rs:484` computes it as
`cell(rows, …)` → `mean(rows, …)` on the unfiltered vector. The BATCHED-conditional
figure is the separate row at `:183` (`option C — staged, BATCHED only`) =
37.82 / 47.34 / 45.82 / 60.82. `U3_tier_t.md:235–239` and `:539–543` are that
document's own warning against exactly this blend ("a blend flattered option B by
half"). The values are transcribed correctly; the conditioning is not, and the
sentence claims a verification that would have caught it. Not load-bearing (it is
§3.3's motivation and §9's comparison baseline), and the error is *against* the
document's own argument, but it is inherited verbatim from revision 1's review report
while claiming independence of it.

**M5. §9's `r8 draw` column carries `(SUPERSEDED)` in its own source and is quoted
unmarked.** `U3_tier_t.md:159` heads it `+1..3 turns, r8 draw (SUPERSEDED)`, and
`:154–156` explains why: revision 1 drew from the radius-8 ball while the policy is
radius 2, "which inflated the ball 78.0 → 123.7 by the sampler rather than by depth".
§3.3 and §9 quote it as "r8 draw". Its ≈31 sits inside the headline `~24×–34×` range,
so the conclusion does not move — but a superseded regime quoted as a live input is
the sort of thing rule 5's registration discipline exists to stop.

**M6. §5's "the free checks cost nothing" contradicts D-388's registered HOTSPOT,
quoted in §9 of the same document.** §5: "`quiescence()`'s gate still runs §3.1/§3.4's
free checks (**they cost nothing** and are not extensions)". D-388, quoted verbatim in
§9: "HOTSPOT = trigger evaluation at horizon nodes (`can_win_this_turn` + plan-t
queries per horizon node)". `blocking_covers` runs an `O(|universe|²)` enumeration
(`cover.rs:216–239`). Both sentences cannot be true; the registered one is the
hotspot, and it is exactly what `q_depth_turns == 0` still pays.

**M7. `live_cells_at_count` is D-352's map entry, not D-267's.** §3.3: "equivalently
`live_cells_at_count(us, LiveCount::Three, …)` (`query.rs:206–208`) — **D-267's map
entry**, the two name the same windows." D-267 (`decisions.md:575`) maps eleven
queries and `live_cells_at_count` is not among them — it maps
`live_windows_at_count`, the window-level query. The cell-level query is the twelfth,
added by **D-352** (`decisions.md:749`), which is also where the recorded coincidence
lives verbatim ("at `LiveCount::Three` this query returns exactly what
`cells_raising_to_hot(side, NearHot::Three)` returns — same windows, same empties,
different question"). The substantive claim is right; the ADR is misattributed, in a
document whose §3.7 makes a point of citing D-267 precisely.

**M8. §9's `rate_c` is an unconditional probability used as a rate over a
subpopulation, and the document does not name the assumption.** Trigger (c) fires only
on `Cover::NothingToBlock` rows (§3.3, and §8's disjointness claim). The Poisson
`P(≥1)` heuristic is applied to `live-3 own`, an unconditional mean over all sampled
rows, so `rate_c` implicitly assumes every position owning a live-3 window is also a
`NothingToBlock` position. `P(≥1 live-3)` ≥ `P(NothingToBlock ∧ ≥1 live-3)`, so the
direction is conservative for a worst-case bracket estimate and the disposition does
not change. §9 names two things the census does not carry, honestly and by name; this
third one is the same kind of thing and is not named.

**M9. `Cover::Impossible` does not mean `t ≥ 3` at ply-2, and §3.5 says
"symmetrically".** §3.2's classification table defines `Impossible` as "t ≥ 3,
`LAW-OVERLOAD`", which is right at `HitBudget::Two`. At ply-2 the budget is
`HitBudget::One` (§3.5's own `HitBudget::from(left')`), where `blocking_covers` skips
the pair enumeration entirely (`cover.rs:225`) and returns `Impossible` whenever **no
single cell** hits every opponent hot window — i.e. `t ≥ 2`. The *score* is still
right (the remaining budget cannot hit, so the opponent wins at `k+2`, matching what
`staged_candidates` already does at `Phase::Second` nodes), but §3.4's cited licence
— `LAW-OVERLOAD`'s "t ≥ 3 for the attacker" (`threat_calculus_v1.md:55–59`) — does not
literally cover the ply-2 case, and an IMPL reading §3.2's table would conclude it
does. One sentence.

**M10 (trivial).** §3.1's trigger-(a) early return neither promotes the witness cell
into the PV nor bumps `seldepth_turns`, both of which `visit` does deliberately at
`pvs.rs:311–321` with a stated reason. §6's enumeration of `StageCounters`' existing
fields omits `batched_quiet_safety_net` (`info.rs:49–55`). §9's "~24×" should be
"~25×" against the "~1 node per horizon" baseline it names (24 *extra* nodes on top of
1). None of these change anything.

---

## Explicitly verified, and NOT a defect — do not re-litigate these

1. **The move set is stated exactly once.** All 594 lines read; no section reintroduces
   `staged_candidates`, `within_radius`, `candidate_cells` or `tier_t_union` as a
   quiescence candidate source. Revision 2's NEW-1/NEW-2 are closed.
2. **`Record { depth_plies: 0 }` really does panic.** Always-on `assert!`, not
   `debug_assert!`. The finding is the document's own and neither prior review made it.
3. **The `Entry` byte arithmetic is correct — compiled, not reasoned.** 18 declared
   bytes, align 8, size 24, six padding bytes, `+ flags: u8` → still 24, bucket still
   96, headroom for six. The seventh byte trips the existing const assertion.
4. **The store/probe rules compose soundly and are deterministic.** `victim()` cannot
   produce two entries for one verification; a declined store leaves the full-width
   entry intact; no clock, no hash order, no thread. `wp16_design_REVIEW_rev2.md`
   NEW-5 is properly closed.
5. **§4's zone claim holds structurally.** `within_radius` `pub(crate)`, exactly two
   call sites, neither reachable from a module importing neither `candidates` nor
   `staged`.
6. **D-388 is quoted character-for-character; nothing moved.**
7. **§9's arithmetic is right** at every step, including the Poisson heuristic.
8. **D-267 does assign `LAW-RIPOSTE` and `LAW-LEDGER` to WP-1.6 by name**, at
   `decisions.md:575`, verbatim: "`LAW-RIPOSTE` and `LAW-LEDGER` are WP-1.6's,
   `ZONE-R` and `LAW-DECOMP` are Stage 3's".
9. **Ruling 2's t≤1 narrowing does not reopen a soundness gap.** A refused t=2 gate
   stands pat on a static read, which is an admission of ignorance, not an unsound
   score. It costs coverage, which is Phase 2's to measure.
10. **Reporting an ESTIMATE outside the registered bracket rather than inside it is
    the right disposition.** Rule 5 asks for the number, not for comfort.
11. **Roughly forty `file:line` citations spot-checked; all resolve.** `pvs.rs:32–34`,
    `:79–80`, `:169–182`, `:194`, `:199–217`, `:260–291`, `:311–321`, `:341`,
    `:387–425`, `:441–457`; `staged.rs:186–189`, `:262–266`, `:283`, `:294–334`;
    `tt/entry.rs:9–11`, `:16`, `:100–103`, `:113–123`, `:121`, `:138–140`, `:174–178`;
    `tt/mod.rs:158–171`, `:179–191`, `:193–200`; `cover.rs:60–61`, `:108–116`, `:201`;
    `query.rs:51–58`, `:70–77`, `:101–107`, `:187–192`, `:206–208`, `:231`;
    `pv.rs:76–79`; `info.rs:39–65`, `:69–83`; `config.rs:161–182`; `params.rs:58–70`;
    `position.rs:48,67,143,187–194`; `decisions.md:575`; `threat_calculus_v1.md:29`,
    `:49–53`, `:55–59`, `:74–77`, `:79–83`, `:93`, `:108`, `:137–141`;
    `U3_tier_t.md:165`, `:168`, `:170`, `:172`, `:181`. Citation hygiene in this
    revision is excellent and is not where it fails.

---

## What would close this

Everything blocking is inside §3.5, §5 and §6, and none of it needs a new mechanism.

1. **§3.5 must answer inside the turn (B1).** State what ply-2 generates when
   `Cover2::cells() ∪ cells_raising_to_hot(us, NearHot::Three)` is empty at
   `Phase::Second`. Ruling 1 forbids the two fallbacks the codebase has, so this needs
   the architect: candidates a design could state are the mover's own
   `threat_cells(us)` (the empties of the window ply-1 just made hot — already inside
   the trigger's own query family and inside §4's window-support bound), or an explicit
   "the extension is refused and the gate stands pat when ply-2 would run dry", decided
   at the gate rather than discovered at ply-2. Either is one paragraph. Whichever it
   is, it must also say what happens to D-104's `assert!`.
2. **§3.5 must not return mid-turn without a line (B2).** Either restore the `is_pv`
   condition for the `Phase::Second` case only, or state that a ply-2 early return
   promotes nothing and prove the parent's line stays turn-whole. §3.4's gate-node
   argument stands as written and does not need touching.
3. **§6 must say what a quiescence `Record` is, or say quiescence does not store
   (B3).** The second is one sentence and deletes the `Entry` field, the `victim()`
   condition and the probe change with it; if Ruling 4's literal store is kept, the
   record's `score`/`bound`/`best` must be stated and a reader named.
4. **§5's list must be corrected (B4)** to the five engine configs, plus
   `VALID_STAGED` and the seven `StagedParams` construction sites, minus the three
   arena configs.
5. **One sentence each for M1 (`MAX_PLY` and `seldepth_turns`), M2 (`Record` gains the
   flag), M3 (`PROTO-NODE` step 5's divergence, named), M6 (the "free checks" wording),
   M9 (`Impossible`'s budget-dependent threshold).** M4/M5/M8 are corrections to §3.3
   and §9's sourcing labels; none of them moves a number.

*Report written by the fresh-context REVIEW-design (Phase 1') subagent against
`a3b9e37`. Left uncommitted for the orchestrating session.*
