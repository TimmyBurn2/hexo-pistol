# REVIEW-design — `docs/experiments/U4_soundness_instrument.md`, u-rev 7

**Pinned revision reviewed: `0f49c90`.**

```
$ git rev-parse HEAD
0f49c9035bf7c4c694e363d6f926a4cde34751ce
$ git status --porcelain
(no output)
```

**Does it still match HEAD? YES** — `0f49c90` is HEAD, tree clean, at the start and at the end of this review. No `git diff 0f49c90 HEAD -- docs/experiments/U4_soundness_instrument.md` is needed; the document has not moved. **Live tree at exit: `git status --porcelain` → empty.**

**Subject:** `docs/experiments/U4_soundness_instrument.md`, **u-rev 7** — the repair of `docs/experiments/wp15b_U4_REVIEW_urev6.md` (REVIEW-design against u-rev 6, `7358a07`, FAIL, 1 BLOCKING / 2 MAJOR / 4 MINOR) plus the fold of D-329 (MATRIX M4 axis A SELECTS N-E), D-330, D-320 and D-325.

**Context was fresh.** I did not author this unit, its repair, either matrix, any red-team round, or any earlier review. I read `CLAUDE.md` first, then `wp15b_U4_REVIEW_urev6.md` in full, then the whole of U4, then `docs/decisions.md` D-316 through D-330 in full, `matrix_M4_axisA_round4.md`, `matrix_M4_axisA_REDTEAM.md`, `matrix_M4_axisA_selection.md`, `wp15b_trackC_REVIEW_impl.md`, `wp15b_sprt_prereg.md` §7A.2/§10/§11, `tools/SHELL_CHECKLIST.md` items 9 and 11, `U2_node_protocol.md` §5.3, `U3_tier_t.md` §10, `restructure_matrix_15b.md` and `restructure_selection_15b.md`.

**Reproducer discipline.** Every finding below carries a command and its real output. No build was needed; no worktree was created. I edited no repository file and ran no git write command.

---

## VERDICT: **FAIL**

**0 BLOCKING, 4 MAJOR, 4 MINOR.**

---

# ⚠ THE THING THE DISPATCH ASKED FOR EXPLICITLY, FIRST

**YES. THERE ARE THREE MORE STALE-DEPENDENT STATEMENTS IN U4-Z, AND ONE OF THEM IS FALSIFIED BY D-329 — THE ADR LINE THIS VERY u-rev FOLDS.**

The u-rev 7 lead-in to U4-Z says the section was *"RE-DERIVED IN FULL … every claim below whose truth depends on an ADR line was re-read against `docs/decisions.md` as it now stands, item by item"* (U4:1443–1448). The re-derivation did not achieve that. Three separate U4-Z claims are falsified by landed state:

| # | U4-Z claim | Falsified by | Class |
|---|---|---|---|
| **MAJOR 2** | *"reopens `tools/baseline_snapshot.sh` with **eight of twelve** items ENGAGED … plus D-329's four conditions on top"* (U4:1783–1784) | **D-329** (via red-team F13) removes `SHELL_CHECKLIST` **item 11** from that set — and **U4's own §9 says so at 923–925** | ADR line THIS u-rev folds |
| **MAJOR 3** | *"A fifth thing is **recorded by D-329** … the relative-base inconsistency"* (U4:1748) | D-329 records no such thing — zero occurrences of `relative`, `CALLER_PWD`, or the F6 residual | ADR line THIS u-rev folds |
| **MAJOR 4** | *"Travelling item T2 … **is named in no document now in the tree**"* (U4:1739–1740) | `restructure_matrix_15b.md:35` and `restructure_selection_15b.md:50` both name it, both as *"M4 ADR line (B2)"* — and **U4:108 itself resolves T5 from that same list** | tree state; false negative-universal |

**This is the third recurrence of the class, and it recurred inside the section rewritten in full to prevent it.** The u-rev 6 review's BLOCKING 1 was one instance (D-320). The u-rev 7 repair found a second no finding had named (D-325). This round finds three more, one of them against the ADR line the same commit folds. Per the dispatching session's standing rule, **the trigger for an architect restructure of U4's status matter is met**; another patch-and-re-review round is the wrong instrument, and I state that as a determination independent of the severity ratings below.

Two structural reasons, both visible in the text:

1. **U4-Z restates in prose what other sections already state, and the restatements drift.** MAJOR 2's eight-of-twelve, MAJOR 3's fifth condition and the four-conditions bullet are all second copies of content that lives correctly at U4:923–925, U4:930–941 and in the selection record. The re-derivation re-derived the copies; it did not remove the duplication that makes copies necessary.
2. **The re-derivation was scoped to "claims that depend on an ADR line".** MAJOR 4 depends on the tree, not on an ADR line, and fell outside the pass by construction. So did the new false completeness claim at MAJOR 1. A re-derivation whose scope excludes a defect class it has already shipped twice is not a closure.

---

# WHAT THE ROUND GOT RIGHT, STATED FIRST BECAUSE IT IS MOST OF IT

**BLOCKING 1 (D-320) is discharged, completely and accurately.** I checked U4:1590–1637 and U4:1749 clause by clause against `docs/decisions.md:685`. Every load-bearing part is present and correctly stated: the breach acknowledged and not argued away; the retro-matrix **WAIVED** on two grounds *"stated together because neither carries it alone"*, with both grounds correct (proportionality; and the independent verification, which D-320 itself calls *"the part that makes this a waiver rather than an excuse"*, with the scope-1 PASS on the named-gate wiring); the *"what this line does not do"* limits quoted verbatim; the debt recorded as **PAID** *"in the only currency available after the fact, which is disclosure"*; and the **flip clause carried in full**, with the correct observation that this unit is where a gate-naming defect would surface. The OPEN-list bullet no longer says the residual is unclosed — it is struck through with the disposition stated (`~~**Its RESIDUAL is not closed**~~`). The landing commit checks out:

```
$ git show 0af32fb -- docs/decisions.md | grep -o "^+D-3[0-9][0-9]:"
+D-319:
+D-320:
+D-321:
```

**I looked for the gate-naming defect D-320's flip names and did not find one.** §8.3's lookup table (U4:649–655) resolves `(a)`, `(b)`, `(c)` and `(d)` to the four names; §8.7 wires exactly those four; §8.3's config bullet is explicitly *"A CONFIG STATEMENT, NOT A FIFTH GATE"*. No part of the gate is unreached by the four names, no citation resolves only under a retired letter, and no fifth gate is appended as a letter. **D-320's flip has NOT fired.** (MAJOR 1 below is an instrument-identity staleness, not a gate-naming defect, and does not fire it.)

**D-325 is a genuine catch and is folded accurately.** It landed after u-rev 6, no finding named it, and U4 records it correctly at two sites:

```
$ git show 81180b8 -- docs/decisions.md | grep -o "^+D-3[0-9][0-9]:"
+D-325:
```

Its content is stated exactly: the cell *"counted one inside site and missed three others"*, **THE COUNT SIX STANDS** with no seventh, D-316's conclusion untouched, and the remedy taken was the new line the unit had asked for. The one elision in U4's quotation of D-325 is marked with `…` and is faithful.

**MAJOR 3 of the prior report is discharged where it was charged, and the selected-from comparison is untouched.** U4:1500–1517 now records the u-rev 2 EXECUTION and states *"S-E FELL in M3 round 1 and the differential gate's instrument since D-323 is S-M"*. The two-shape comparison is **byte-identical** to u-rev 6:

```
$ sed -n '1139,1169p' <7358a07 copy> > cmp_old.txt
$ sed -n '1468,1498p' docs/experiments/U4_soundness_instrument.md > cmp_new.txt
$ diff -u cmp_old.txt cmp_new.txt && echo "IDENTICAL — no diff"
IDENTICAL — no diff
```

**MAJOR 2 of the prior report is discharged.** The blanket stamp is replaced by a two-sided one (U4:355–374), and all five named NOT-RECORD blocks are current text at the sites named — §8.2's `FOLDED AT u-rev 6` (504), §8.3's four-gate table (642–647), §8.3's live **U3** §10 sentence (674–676), §8.4's `RE-READ AT u-rev 6` block and its M3/M4/M6 marks (778–795), §8.7's wiring sentence and fold (839–861). I tested the other direction too: the RECORD enumeration's scope is now narrow enough that no current-state block falls inside it (see *Rejected* for the two candidates I tried).

**The M4 fold is accurate**, clause by clause, against all three records and D-329/D-330. Details in *Verified with no finding*.

**Rule 9 is clean.** `grep -n "wc -l\|line count\|[0-9]\{3\} lines"` returns only the two sites where U4 states that it asserts none.

**The census gate passes.**

```
$ cargo test -p pistol-solver --test wp15b_census
running 5 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.56s
```

---

# FINDINGS

## MAJOR

### 1. §8.7 still says, in the present tense and in the unit's own voice, that S-E **is** the differential gate — so u-rev 7's own claim "NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS S-E" is false, at the sibling site MAJOR 3's repair did not reach

**The claim reviewed — U4:138, the REVIEW STATUS row for MAJOR 3:**

> **REPAIRED.** … **NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS S-E.**

**The contradicting sentence:**

```
$ sed -n '833,837p' docs/experiments/U4_soundness_instrument.md
> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
> named.** The double-list dies with the letters — S-E is not listed beside the
> letters it was one of, because it IS the differential gate and is named once, in
> §8.2. The two-shape comparison the selection was made from stands unedited in
> U4-Z, with the selection recorded beneath it.
```

**It is the same sentence MAJOR 3 charged, at its sibling site.** The U4-Z copy read *"the S-E double-list dies with the letters, because S-E **is** the differential gate and is named once, in §8.2"*; u-rev 7 repaired it to *"the double-list dies with the letters, because **the differential gate is named ONCE**, in §8.2"* (U4:1503–1504). §8.7's copy was not touched:

```
$ git show 35aab95:docs/experiments/U4_soundness_instrument.md | grep -n "because it IS the differential gate"
448:> letters it was one of, because it IS the differential gate and is named once, in
$ git show 7358a07:docs/experiments/U4_soundness_instrument.md | grep -n "The double-list dies with the letters"
743:> named.** The double-list dies with the letters — S-E is not listed beside the
```

Unchanged from u-rev 5 through u-rev 7.

**It is not protected as RECORD, on the unit's own two rulings.** *(i)* The re-scoped stamp's RECORD enumeration (U4:355–363) is: the u-rev 1 SELECTION OPEN heading, the stub and its five-row table; §8.1; §8.2's body prose from *"The stage under doubt, named"* to the end of §8.2; and *"the prose §8.3 and §8.4 carry from the superseded document"*. §8.7 is not in it. *(ii)* More decisively, the unit **edited text inside this very blockquote at u-rev 6** — MINOR 9's repair of the `ec8f7fb:502` quotation sits at U4:823–826, four lines above — expressly *"because that one is **carve prose** and not the selected-from text"* (U4:161, U4:1539–1540). The unit has itself ruled this block editable carve prose, which is the same category the MAJOR 3 repair used to justify correcting U4-Z's copy in place.

**Why it breaks.** §8.7 is the section U4 designates as *"the live specification `tools/staged_soundness_check.sh` is taken from"* (U4:373–374). A reader arriving there for the script's enumeration meets, in the paragraph that explains why the wiring reads as it does, an unmarked present-tense identification of a fallen row as the gate's instrument. Independently, u-rev 7 added a **completeness claim** to the REVIEW STATUS table asserting that no such sentence exists, and a false completeness claim beside an incomplete pass is precisely the defect this unit deleted from its own BLOCKING 1 row at this u-rev (*"~~across the whole unit~~ — that claim is WITHDRAWN AS FALSE"*, U4:153).

**Why MAJOR and not BLOCKING.** The sentence points at §8.2, and §8.2 states S-M correctly at its head — a reader who follows the pointer is corrected. The damage is the false completeness claim and one unmarked stale identification, not a wrong specification.

**Fix scope (not mine to apply).** One clause at U4:835–836, on the annotation pattern §8.3's table cell and U4-Z's SELECTION block already use; and the REVIEW STATUS row's universal at U4:138 either verified or narrowed.

---

### 2. U4-Z's OPEN list states that N-E's `--config` engages **eight of twelve** SHELL_CHECKLIST items "plus D-329's four conditions on top" — D-329 removes item 11 from that set, and U4's own §9 says so 860 lines above

**The claim reviewed — U4:1781–1785:**

```
$ sed -n '1781,1785p' docs/experiments/U4_soundness_instrument.md
- **The `tools/` changes this unit implies have had no SHELL_CHECKLIST review.**
  §8.7's `staged_soundness_check.sh` is a new script; §9's `--config` — **now a
  SELECTED shape, N-E under D-329** — reopens `tools/baseline_snapshot.sh` with **eight
  of twelve** items ENGAGED by its own count, plus D-329's four conditions on top. Both
  reviews are owed at IMPL, and the coverage rule binds each.
```

The "own count" is §9.1 amendment 2, RECORD carried verbatim:

```
$ sed -n '1299p' docs/experiments/U4_soundness_instrument.md
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
```

**Contradicting evidence — D-329 measures item 11 out, and U4 carries the measurement itself.** D-329 (`docs/decisions.md:703`):

> N-Q's extra lines are required by no rule here, since **item 11's scope is a binding consumed by `rm`, `mv` or a write** and item 9 is discharged by the guard both rows owe.

U4's own §9 head, carrying red-team F13:

```
$ sed -n '922,925p' docs/experiments/U4_soundness_instrument.md
> **N-Q's extra lines are required by no rule in this tree (F13):** item 11's scope is
> *"any binding consumed by `rm`, `mv`, or a write"* and `$CONFIG` is a READ, so item 9
> governs it and is discharged by the whole-path guard **both** rows owe.
```

And the checklist itself:

```
$ sed -n '117,120p' tools/SHELL_CHECKLIST.md
## 11. A CALLER'S PATH THAT FEEDS A DELETE OR AN OVERWRITE IS CONTAINMENT-GUARDED

**Any binding consumed by `rm`, `mv`, or a write is guarded so that its resolved
path is provably under the root the script means.
```

`$CONFIG` is a READ. Item 11 is not engaged.

**Why it breaks.** The bullet is in the OPEN list — the surface an IMPL session reads to learn what it owes — and its subject is explicitly the **post-D-329** state (*"now a SELECTED shape, N-E under D-329"*). It tells that session the reopening engages item 11 and that D-329 only **adds** four conditions on top. D-329 both adds four and subtracts one, and the subtraction was one of the three measured findings the selection turned on. The hedge *"by its own count"* attributes the number to §9.1 but does not qualify the composite claim, which is the one the reader acts on. This is the third consecutive u-rev in which U4-Z carries a claim a landed ADR line falsifies, and here the line is the one this u-rev exists to fold.

**Fix scope.** One clause: state the engaged set as §9.1 counted it *for N-A*, and record that D-329 removes item 11 for N-E's read binding, per the unit's own §9 F13 paragraph.

---

### 3. U4-Z attributes the relative-base residual to D-329 — "a fifth thing is recorded by D-329" — and D-329 records nothing of the kind

**The claim reviewed — U4:1748, the closing parenthetical of the four-conditions bullet:**

> *(A fifth thing is **recorded by D-329** and is not a condition: the relative-base inconsistency — a relative `--config` resolves against `$ROOT` while a relative `--out` resolves against `$CALLER_PWD` — which N-E inherits without making it load-bearing for a refusal. OPEN.)*

**Contradicting evidence — D-329 contains none of it:**

```
$ sed -n '703p' docs/decisions.md | grep -c "CALLER_PWD\|relative-base\|relative"
0
$ grep -n "CALLER_PWD" docs/decisions.md
(no output)
```

The only `ROOT` token in D-329 is in the **F5** clause about N-Q's logical/physical `pwd` mismatch — a different finding about a different row. The relative-base residual is **F6**, and it is recorded in the selection record, not the ADR:

```
$ sed -n '114,117p' docs/experiments/matrix_M4_axisA_selection.md
4. **The relative-base inconsistency is recorded, not fixed** (F6): a relative
   `--config` resolves against `$ROOT` while a relative `--out` resolves against
   `$CALLER_PWD`. N-E inherits it; unlike N-Q it does not make it load-bearing
   for a refusal. It is OPEN.
```

**Why it breaks.** The residual itself is real and correctly described — this is an attribution defect, which is the class D-322 and D-330 have both landed lines about in this same work package. It matters here specifically: the selection record's condition list and D-329's condition list **differ** (the record's items 3 and 4 are the D-324 supersession and the relative-base residual; D-329's are the item-10 test and the item-12 sentence). U4 correctly follows D-329's four, then attributes the record's fifth to D-329 as well. A reader who checks the ADR — which is what "recorded by D-329" instructs — finds nothing, and an ADR-only reader never learns the residual is open. It also directly falsifies the section's own lead-in claim that every ADR-dependent claim was re-read against `docs/decisions.md`.

**Fix scope.** One word: attribute it to `matrix_M4_axisA_selection.md` (condition 4, red-team F6), which D-329 names as the selection record.

---

### 4. U4-Z says travelling item T2 "is named in no document now in the tree" — two tree documents name it, both as exactly the item U4-Z is disposing of, and U4:108 resolves a sibling label from the same list

**The claim reviewed — U4:1738–1741:**

> *(Travelling item T2 is carried under this head. **It is named in no document now in the tree** — it comes from the superseded §16's travelling list — so this unit records the ADR state, which is what a reader can check, and **does not assert a disposition for a label it cannot resolve**.)*

**Contradicting evidence:**

```
$ grep -rn "\bT2\b" docs/ --include=*.md | grep -v U4_soundness_instrument
docs/experiments/restructure_matrix_15b.md:35:T2. M4 ADR line (B2).
docs/experiments/restructure_selection_15b.md:50:T2. M4 ADR line (B2). T3. Gate (b) defined or wiring rewritten, S-E
$ sed -n '35p' docs/experiments/restructure_matrix_15b.md
T2. M4 ADR line (B2).
```

Both are in the tree at HEAD, both under a heading `## Travelling items (bind to every option, cost is common)`, and both give T2 the same content — **"M4 ADR line (B2)"** — which is precisely the head U4-Z carries it under and precisely what D-329 discharges.

**And U4 resolves T5 from the same two documents, in its own head:**

```
$ sed -n '108p' docs/experiments/U4_soundness_instrument.md
**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
$ sed -n '50,51p' docs/experiments/restructure_selection_15b.md
T2. M4 ADR line (B2). T3. Gate (b) defined or wiring rewritten, S-E
     double-list killed (B3). T5. Label bump on any append.
```

T2 and T5 are on the same line of the same file. The unit resolves one and declares the other unresolvable.

**This is new at u-rev 7:**

```
$ git show 7358a07:docs/experiments/U4_soundness_instrument.md | grep -n "named in no document now in the tree"
(no output)
```

**Why it breaks.** The false premise has a consequence the sentence itself names: U4-Z **withholds a disposition** it is in a position to make. T2 is *"M4 ADR line (B2)"*; U4-Z has just established, correctly and at length, that B2 is ANSWERED by D-329 with a named residual. That is the disposition T2 wants, and it is withheld on the ground that the label "cannot be resolved" — when it resolves in two tree documents to the exact item under discussion. A travelling item is precisely the thing a carve must not silently drop, and this is the document that owes it.

**Fix scope.** Delete the false premise and state T2's disposition: named at `restructure_matrix_15b.md:35` and `restructure_selection_15b.md:50` as *"M4 ADR line (B2)"*, DISCHARGED by D-329 with the assembled-not-quoted attack recorded as its residual.

---

## MINOR

### 5. U4-M item 1's parenthetical mis-describes one of the four `--config` occurrences it counts — the fourth is in a comment, not on an invocation line

```
$ sed -n '1386,1389p' docs/experiments/U4_soundness_instrument.md
     BUILT.** MEASURED at `8690ad6`: `tools/baseline_snapshot.sh:182` still reads
     `CONFIG="configs/instrument_v0.toml"` and the script takes no `--config` argument
     of its own (the four `--config` occurrences in it are the ENGINE's flag on
     invocation lines, not the script's).
$ grep -n -- "--config" tools/baseline_snapshot.sh
465:printf 'pistol\nquit\n' | timeout "$HANDSHAKE_TIMEOUT" "$BINARY" --config "$CONFIG" >"$WORK/hs" 2>/dev/null || HANDSHAKE_RC=$?
481:	# caller-named and the guard above does not cover it. If a `--config` flag is
514:timeout "$CORPUS_TIMEOUT" "$BINARY" --config "$CONFIG" <"$WORK/corpus.session" >"$WORK/corpus.out" || CORPUS_RC=$?
581:		timeout "$LADDER_CAP_S" "$BINARY" --config "$CONFIG" >"$out" 2>/dev/null || rc=$?
$ grep -o -- "--config" tools/baseline_snapshot.sh | wc -l
4
```

The count of four is right; three are the engine's flag on invocation lines and the fourth (481) is inside the comment `` # caller-named and the guard above does not cover it. If a `--config` flag is / # ever added, this line joins that guard in the same commit ``. The conclusion — the script takes no `--config` of its own — is **strengthened** by line 481, which says so explicitly (*"no flag sets it"*), so the finding is an enumeration defect and not a wrong answer. It is charged because the parenthetical is a **MEASURED** claim added at u-rev 7 (`git diff 7358a07 0f49c90` line 611), and because the misdescribed line is the one comment in the tree that binds the next `--config` change — a fact worth citing correctly in the unit that owes that change.

### 6. The REVIEW STATUS row for MINOR 7 asserts "Every abbreviated citation now reads 'S-M SELECTED at `af8082a` (taken at `809b5db`)'" — one live abbreviated citation does not

```
$ grep -on "af8082a[^)]\{0,45\}" docs/experiments/U4_soundness_instrument.md
29:af8082a`**; the record is `docs/experiments/matrix_M
142:af8082a` (taken at `809b5db`
182:af8082a` (taken at `809b5db`
182:af8082a`; **D-323**
217:af8082a`**
387:af8082a`.
395:af8082a`. The five
854:af8082a` is **S-M**
1745:af8082a` (taken at `809b5db`, the revision carrying
1745:af8082a`
1886:af8082a` (taken at `809b5db`; D-323
```

The four sites the prior report named (U4:157–158 → now 29/217, U4-A's round-2 row 182, the OPEN list's B1 bullet 1745, the closing line 1886) are all repaired to one convention, and that is the substance of MINOR 7. But U4:854 — inside §8.7's `FOLDED AT u-rev 6` paragraph, which the re-scoped stamp lists as **NOT RECORD**, item (v) — reads *"the instrument selected at `af8082a` is **S-M** (D-323)"* with no `809b5db`. 387 and 395 are inside stamped u-rev 6 record parentheticals and I do not charge them. The finding is the universal, not the citation: *"Every abbreviated citation"* is a completeness claim of exactly the shape this u-rev deleted from its own BLOCKING 1 row.

### 7. `91 test lines` is added at u-rev 7 at two sites with no MEASURED / ESTIMATED mark, and it is measurable in one command

```
$ sed -n '937,939p' docs/experiments/U4_soundness_instrument.md
> 3. **AN ITEM-10 DRIVING TEST IS OWED for both new refusal classes**, in two halves
>    with a control, per the coverage rule. The precedent is one commit old: `b067d47`
>    paid 91 test lines for ONE guard arm.
$ grep -n "91 test lines" docs/experiments/U4_soundness_instrument.md | cut -c1-60
939:>    paid 91 test lines for ONE guard arm.
1748:- **THE FOUR CONDITIONS RIDING WITH N-E ARE UNPAID, AND EACH BINDS IMPL**
```

The number is correct —

```
$ git show b067d47 --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
91	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
54	4	tools/baseline_snapshot.sh
```

— and the red team pastes exactly that numstat at F7. Both U4 sites are new at u-rev 7 and neither carries a mark, in a unit whose head states that *"a mark added at u-rev 6 names the command that took it and pastes its complete output."* The same applies, more weakly, to *"2 classes … against 5 classes plus an unpinned normalisation"* at U4:909–910, which is not covered by the *"fewest MEASURED added lines"* mark governing the line counts in the same sentence. (I do not charge `22 / 7 / 32 / 12 / 4 / 5`: those are inside the MEASURED clause and reproduce exactly — see *Verified with no finding*.)

### 8. The assembled strongest-surviving-attack blockquote drops the selection record's closing sentence with no elision mark

```
$ python3  # normalised comparison, U4:957-964 against matrix_M4_axisA_selection.md:127-138
U4 is prefix of record: True
```

U4's blockquote is **character-identical** to the record for everything it quotes, and stops at *"only left where it was."* The record continues: *"N-E therefore ships a seam whose provenance guarantee is exactly as strong as the caller, and this round chose it knowing that and having measured that the alternative bought nothing for its five extra lines except a new false refusal."* The omission is **in the attack's favour** — the dropped sentence is the round's defence — so nothing is softened, which is why this is MINOR and not MAJOR. It is charged because the unit's own MINOR 9 discipline is that *"text presented as a verbatim quotation should be one"*, and because D-323's fold in the same document marks its elisions with `…` where D-329's quotation of the same paragraph does.

---

# MY OWN U4-Z CLAIM INVENTORY, BUILT INDEPENDENTLY

I did not take the repair's "34 checked / 22 held / 11 changed / 1 deleted" on trust. Below is every U4-Z claim whose truth depends on an ADR line, on another U4 section, or on tree state, with my own verdict. **Result: 34 claims examined, 31 hold, 3 fail** (MAJOR 2, 3, 4 above).

### Lead-in (U4:1443–1454)

| # | Claim | Verdict |
|---|---|---|
| Z1 | U4-Z's B3 section was not in u-rev 6's enumerated fold sites — BLOCKING 1 | **HOLDS** — the enumeration at U4:153 lists ten sites and U4-Z's B3 section is not among them |
| Z2 | *"every claim below whose truth depends on an ADR line was re-read against `docs/decisions.md` as it now stands, item by item"* | **FAILS** — MAJOR 2 and MAJOR 3 |
| Z3 | D-320 landed at `0af32fb`, in the same commit as D-321 | **HOLDS** — `git show 0af32fb` adds D-319, D-320, D-321 |
| Z4 | D-325 landed at `81180b8`, after u-rev 6 | **HOLDS** — `git show 81180b8` adds D-325; `7358a07` precedes it |

### B3 section (U4:1456–1637)

| # | Claim | Verdict |
|---|---|---|
| Z5 | Heading: *"SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320"* | **HOLDS** |
| Z6 | The two-shape comparison is left UNEDITED | **HOLDS** — `diff` against `7358a07` is empty |
| Z7 | SELECTION block records the u-rev 2 execution; the gate named once in §8.2 was then S-E | **HOLDS** |
| Z8 | *"S-E FELL in M3 round 1 and the differential gate's instrument since D-323 is S-M"* | **HOLDS** — D-323, `docs/decisions.md:691` |
| Z9 | The block is carve prose recorded AFTER the comparison, so the unedited discipline does not reach it — *"the same reading applied to §8.7's copy under MINOR 9"* | **HOLDS**, and is the ruling that makes MAJOR 1 a finding |
| Z10 | *"The shape-2 selection … decided how the gate is ADDRESSED, never which instrument it carries"* | **HOLDS** — D-316's subject is the addressing scheme |
| Z11 | `(b)` lost its definition at revision 2 and the document shipped FIVE further revisions | **HOLDS** — consistent with U4:830 |
| Z12 | MINOR 9: `ec8f7fb:502` actually reads *"(a) **the** tactical suite **at 100 % of its** …"* | **HOLDS** — verified by the prior reviewer against `git show ec8f7fb` and consistent with U4:825 |
| Z13 | The cost cell said THREE; executing it the count is **SIX**, six sites tabulated | **HOLDS** — D-325 confirms six and no seventh |
| Z14 | The u-rev 6 diagnosis is FALSE and withdrawn (MINOR 8) — the cell counted one inside site and missed three others | **HOLDS** — D-325 states exactly this |
| Z15 | *"THE SAME FALSE DIAGNOSIS WAS IN THE LANDED D-316"*, quoted | **HOLDS** — `docs/decisions.md:677` contains *"counted only the sites outside the unit and missed three inside it"* verbatim |
| Z16 | D-325 disposed of it, at `81180b8`, by a new line not an edit | **HOLDS** |
| Z17 | D-325 records the COUNT SIX stands, no seventh, D-316's conclusion untouched | **HOLDS** — verbatim in D-325 |
| Z18 | The D-325 quotation *"the paragraph the false sentence sits in is ABOUT THE ACCURACY OF COST CELLS … "* | **HOLDS** — elision marked and faithful |
| Z19 | *"nothing here is owed to the architect any more"*; the OPEN list records it CLOSED | **HOLDS** — U4:1866–1875 |
| Z20 | Shape 2's stated cost is DISCHARGED — §8.3 opens with a letter→gate lookup table so legacy citations resolve | **HOLDS** — U4:642–655 |
| Z21 | D-320's ruling, all six parts (breach acknowledged; waiver on two grounds; proportionality; independent verification with the scope-1 PASS; what it does not do; debt PAID by disclosure) | **HOLDS** — each quoted clause is verbatim from `docs/decisions.md:685` |
| Z22 | D-320's flip clause, quoted, with *"this unit is where it would fire"* | **HOLDS**; and I checked for the defect — none found |
| Z23 | *"D-320's waiver rests on THIS unit's review history"* | **HOLDS** — D-320 cites `wp15b_U4_REVIEW.md`'s scope-1 PASS by name |

### ADR-lines section (U4:1639–1741)

| # | Claim | Verdict |
|---|---|---|
| Z24 | Item 4 is no longer blocked on M3 but on the SEAM, which D-323 records as separate and OPEN | **HOLDS** — D-323 *"(a) The SEAM … is a separate named decision that is OPEN"* |
| Z25 | Item 4's seam ≠ D-329's seam; D-115 bars widening `pistol_search::staged` to `pub` | **HOLDS** — the two seams are distinct decisions and D-329's subject is the snapshot's config path |
| Z26 | Item 15 stays blocked, ground narrowed: seam SELECTED AND NOT BUILT, plus the missing config document, which D-329 says blocks every axis-A row equally | **HOLDS** — D-329: *"it blocks all three rows equally"* |
| Z27 | Item 15's MAJOR 4 disagreement is UNRECONCILED and unchanged by the selection | **HOLDS** |
| Z28 | Numbered item 4: S-M, R1 reused by `#[path]`, DEPENDS-OPEN-THEORY, five conditions, seam still the blocker at u-rev 7 | **HOLDS** — all five conditions and the mark match D-323 |
| Z29 | Numbered item 15: **U3** (u-rev 4) §10 is the one place the count is stated and this line does not restate it | **HOLDS** — U3 is u-rev 4; `U3_tier_t.md:331–336` states **FOUR** and *"This is the one place the count is stated"* |
| Z30 | B2: the revision-7 review found M4 had no ADR line; three rounds stopped (rounds 1–2 under D-318, round 3 under D-324's recorded tie); D-329 is now M4's line and records field / attack / selection / four conditions / what it supersedes | **HOLDS** — every element verified against D-324, D-329 and the three M4 records |
| Z31 | The strongest surviving attack against N-E is ASSEMBLED and not quoted; the red team was dispatched to break N-Q and never asked to break N-E | **HOLDS** — `matrix_M4_axisA_selection.md:119–125` and D-329 both state it |
| Z32 | *"nothing of N-E is built, so the snapshot still has a BEFORE and no AFTER"* | **HOLDS** — measured below |
| Z33 | *"Travelling item T2 … is named in no document now in the tree"* | **FAILS** — MAJOR 4 |

### OPEN list (U4:1743–1882)

| # | Claim | Verdict |
|---|---|---|
| Z34 | B1/M3 CLOSED by S-M (D-323); five residuals each with its own bullet | **HOLDS** — all five residuals have bullets (DEPENDS-OPEN-THEORY, S-N, seam, corpus/per-CI cost, `0 of 3406`, second referent) |
| Z35 | B2/M4 ANSWERED by N-E (D-329); round ran `7866bcf`, attacked `7e0a328`, landed `d56a898`; N-M eliminated on registered ground; rung (a) silent; rung (b) 22/7 vs 32/12 | **HOLDS** — every number re-derived below |
| Z36 | AXIS B IS NOT REOPENED; D-324's flip already fired toward N-K; no ADR line adopts N-K | **HOLDS** — D-324 and D-329 both state it |
| Z37 | N-E has never been attacked in its own right; the assembled paragraph's own point is where to attack | **HOLDS** |
| Z38 | The four conditions riding with N-E, each stated | **HOLDS** — all four match D-329 exactly |
| Z39 | *"A fifth thing is recorded by D-329 … the relative-base inconsistency"* | **FAILS** — MAJOR 3 |
| Z40 | B3 residual DISPOSED OF by D-320, debt PAID, flip clause live | **HOLDS** |
| Z41 | MAJOR 8's residual has two parts; `crates/pistol-search/src/staged.rs` does not exist at `46c58ac` and `8690ad6` | **HOLDS** — `ls crates/pistol-search/src/staged.rs` → *No such file or directory* at HEAD; the file has never existed |
| Z42 | The snapshot's second instrument, agreement criterion, stage under doubt and disagreement consequence are unregistered | **HOLDS** — U4-M registers only the replication |
| Z43 | *"eight of twelve items ENGAGED … plus D-329's four conditions on top"* | **FAILS** — MAJOR 2 |
| Z44 | Both shipped-instrument defects CLOSED at `b067d47` and `a102c6a`; REVIEW-impl PASSED at `84ff8d7`, 0/0/3, on mutation-checked controls | **HOLDS** — `wp15b_trackC_REVIEW_impl.md:709–711` reads **VERDICT: PASS**, *"0 BLOCKING, 0 MAJOR, 3 MINOR (F1, F2, F3)"* |
| Z45 | F1, F2, F3 as described (usage-block universal / `$CONFIG` comment criterion / `Run::refusal()` backing 18 of 19) | **HOLDS** — each matches the report's own text, including the 18-vs-1 counts |
| Z46 | §8.4's M3 witness is NOT BUILT and a legal position is owed | **HOLDS** — U4:792 states it in the cell |
| Z47 | The differential gate's seam is OPEN and is not D-329's seam; no matrix has been authored for it | **HOLDS** |
| Z48 | S-E's second half is neither selected nor rejected; no ADR line since D-323 carries it | **HOLDS** — D-329 decides the snapshot seam only |
| Z49 | The per-CI cost is UNGROUNDED at its dominant term; D-323 does not re-price the gate | **HOLDS** — D-323 lists the corpus/per-CI cost under what it does not decide |
| Z50 | `configs/instrument_staged_v0.toml` does not exist; no row of any M4 revision produces it, round 4's three included; D-329 corrects D-324's N-M framing | **HOLDS** — measured below |
| Z51 | The seam is SELECTED AND NOT BUILT; `tools/baseline_snapshot.sh:182` still reads the literal and the script takes no `--config` of its own | **HOLDS** on the claim; the enumerating parenthetical at U4-M is MINOR 5 |
| Z52 | Item 15's blockage is unreconciled between a landed ADR and a landed review, and the selection did not resolve it | **HOLDS** |
| Z53 | D-316's false diagnosis CLOSED by D-325 | **HOLDS** |
| Z54 | No REVIEW-design has run at this u-rev; u-rev 6 FAILED 1/2/4 and u-rev 5 FAILED 3/3/5 | **HOLDS** — matches both reports' own headers |

---

# VERIFIED WITH NO FINDING

- **The M4 axis-A fold is accurate, clause by clause.** I checked U4:867–987 and U4:1746 against `matrix_M4_axisA_round4.md`, `matrix_M4_axisA_REDTEAM.md`, `matrix_M4_axisA_selection.md`, D-329 and D-330. Correct on: the field (`7866bcf`, three rows {N-E, N-M, N-Q}, axis A alone, N-Q authored in, authored under the D-328 split); the attack (`7e0a328`, **thirteen findings** — F1…F13 — and **nine of ten facts reproducing**, FACT 5 being the partial); the selection (`7e0a328` taken, `d56a898` landed); N-M's elimination on registered ground *before* the ladder; rung (a) silent across the field with all three of its grounds; rung (b)'s cells; rung (c) not reached; the three substance findings (F4's gitignored `.bin`, F5's logical/physical `pwd`, F13's item-11 scope); the four conditions; and D-330's three-part amendment.
- **Every rung-(b) number reproduces against the red team's own re-derivation.**
  ```
  $ sed -n '65,68p' docs/experiments/matrix_M4_axisA_REDTEAM.md
  --- N-E ---  numstat: 22  8   added CODE: 7   added COMMENT: 15   blank added: 0   bash -n: OK
  --- N-M ---  numstat: 21  6   added CODE: 10  added COMMENT: 11   blank added: 0   bash -n: OK
  --- N-Q ---  numstat: 32  8   added CODE: 12  added COMMENT: 20   blank added: 0   bash -n: OK
  ```
  and *"N-E's 7 = `CONFIG=""` + flag arm + `[ -n ]` + **4 guard lines**; N-Q's 12 = those 7 + exactly **5 containment lines**"* — so U4's *"22 added / 7 CODE against N-Q's 32 / 12; both owe the same 4 whole-path guard lines; N-Q owes 5 containment lines on top"* is exact. F7's table gives **2** new refusal classes for N-E against **5** plus the unpinned normalisation for N-Q — exact.
- **N-E is stated as SELECTED and NOT BUILT everywhere it appears, and nothing in the unit reads as though the Staged snapshot were unblocked.** The disclaimer is carried at six independent sites (head bullet 53–55; §9 head 971–977; §9's re-derived blocker paragraph 1187–1198; U4-M item 1 1381–1391 and 1417–1420; U4-Z items 15 and the OPEN bullets 1662–1670, 1849–1854; the closing line 1886). It is true at HEAD:
  ```
  $ ls configs/instrument_staged_v0.toml
  ls: cannot access 'configs/instrument_staged_v0.toml': No such file or directory
  $ sed -n '182p' tools/baseline_snapshot.sh
  CONFIG="configs/instrument_v0.toml"
  ```
  The script has no `--config` flag of its own; its argument loop takes none.
- **D-324 is recorded as superseded in COUNT and in KIND, correctly, at both sites.** U4:943–949 and U4:1150–1154 both give *"wrong in COUNT — four, measured at `b067d47`"* and *"wrong in KIND: the line-289 loop guards a BASENAME while the record writes a WHOLE PATH"*, which is D-329's own wording. The `configs/instrument_staged_v0.toml` framing correction (not an N-M cost; blocks all rows equally) is carried at U4:947–949, 1400–1402 and 1845–1848.
- **D-329's four riding conditions are carried accurately and completely** at U4:930–941 and U4:1748: `$3` not `$4` with the three-field reasoning and the `corpus`-line cross-contamination; the guard-not-a-basename-loop spelling with the `configs/spaced dir/…` witness; the item-10 driving test in two halves with a control against the `b067d47` precedent; and the item-12 FAIL sentence for a script with no void class. Nothing is added and nothing is dropped.
- **The registered ground for N-M's elimination is real.** `docs/experiments/wp15b_sprt_prereg.md:354` opens §7A.2; the invocation `tools/baseline_snapshot.sh --config configs/gate_v0.toml` appears there; §10 registers the flip *"if `tools/baseline_snapshot.sh` lands `--config` in a shape the §7A.2 criterion cannot be taken under"* (`:750–751`); and §11 records *"This document has never passed a review, and it does not claim to"* (`:757`).
- **All six `(u-rev N)` cross-unit citations are current.** `grep -on "\*\*U[0-9]\*\* (u-rev [0-9])"` returns U3 (u-rev 4) at 70, 155, 674, 756, 1702 and U2 (u-rev 3) at 259. `U3_tier_t.md:15` is **u-rev 4**; `U2_node_protocol.md:15` is **u-rev 3**; `U1_gate_supersession.md:15` is **u-rev 2** and U4 makes no u-rev-bearing U1 citation. Each cited section supports the claim: U3 §10 carries the config table and the *"FOUR … one place the count is stated"* ruling (`:331–336`); U2 §5.3 evaluates at `StonesLeft::from_state(state)` and `HitBudget::from(left)` (`:259–264`), which is the phase-derived budget U4 attributes to it.
- **MINOR 4 is repaired at both ends.** U4:1406 now reads **ABOVE**, and §9.1 amendment 4 carries its own marked note at U4:1316–1324, so a reader meeting the amendment first meets the misattribution with it.
- **MINOR 5 is repaired by both remedies.** U4:674–676 narrows to *"states no such number **as a live claim**"* and the quotation is replaced by a description (*"gave that count as a cardinal and, in the same breath, cited **U3** §10 as the only place …"*). No cardinal survives in the description.
- **MINOR 6 is repaired in the shape that cannot go stale.** U4:141 and U4:165 record the STANDING DUTY rather than a size or a u-rev, and state that this unit asserts no line count of itself.
- **Rule 9 holds.** `grep -n "wc -l\|line count\|[0-9]\{3\} lines"` returns only U4:141 and U4:165, both of which are the statement that no count is asserted.
- **The two-sided record stamp's NOT-RECORD list names five blocks and all five are current text at the sites named**, and each carries its own marker at its site: §8.2:504 (`FOLDED AT u-rev 6`), §8.3:642–647 (the four-gate table, whose differential row states S-M), §8.3:674–676 (the live **U3** §10 sentence), §8.4:778–786 plus the u-rev 6 marks in the M3/M4/M6 cells at 792/793/795, §8.7:839–861 (the wiring sentence and its fold).
- **The census gate passes** — output pasted above.
- **The tree is clean at exit.**

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **"The re-scoped RECORD stamp still sweeps up §8.3's legacy-citation lookup paragraph (U4:649–655), which is current state and is not among the five NOT-RECORD blocks."** Attempted: the paragraph is carve-authored at u-rev 2, is load-bearing now (U4-Z:1583–1588 relies on it to discharge shape 2's stated cost), and is not listed at U4:365–374. **Rejected** — the RECORD enumeration's §8.3 clause is scoped to *"the prose §8.3 and §8.4 **carry from the superseded document**"*, and this paragraph is not carried from it, so it is not swept up; and it sits immediately beneath the four-gate table and is fairly read as part of NOT-RECORD item (ii), which names that table and its function. No over-reach in the direction MAJOR 2 charged.
- **"§8.3's sentence at U4:747 — 'the differential gate (§8.2 — S-M since D-323, S-E until u-rev 6)' — is a u-rev 6 retarget inside prose the stamp classifies as RECORD."** Attempted: the host sentence is carried from the superseded document and the parenthetical is not. **Rejected** — the clause is self-dating (*"S-M since D-323"*), so reading it as record does not mislead about the current instrument, and the defect MAJOR 2 named was a stamp asserting that current specifications are superseded. This one asserts nothing false about the state.
- **"The u-rev 7 fold overstates D-329 by writing 'every axis-A row' where D-329 says 'all three rows'."** Attempted: U4:1667–1668 and 1847–1848 against D-329's *"it blocks all three rows equally"*. **Rejected** — round 4's field is exactly the three axis-A rows {N-E, N-M, N-Q}, so the two phrasings are coextensive over the field D-329 governs, and U4 states the field explicitly at 870–872.
- **"D-320's flip clause has fired — a gate-naming defect is present, in MAJOR 1's S-E sentence."** Attempted: read D-320's three named defect shapes against §8.3 and §8.7. **Rejected** — MAJOR 1 is a stale INSTRUMENT identity, not a gate-naming defect. All four gate names are defined (§8.3), all four are wired (§8.7:839–849), every retired letter resolves through the lookup table (§8.3:649–655), and the one unlabelled §8.3 bullet is expressly *"A CONFIG STATEMENT, NOT A FIFTH GATE"*. The retro-matrix is not owed.
- **"The N-K axis-B numbers at U4:881–882 are unmarked."** Attempted: *"measured at 8 added lines, 0 removed, ~2× wall (2 × 33 s)"*. **Rejected** — the mark is present in the sentence (`measured at`), carried from D-324's own *"the red team then measured it at 8 added lines, 0 removed, no re-indentation … ~2× wall (2 × 33 s …)"*, which is the mark that text carried.
- **"The M4 stop record's ten-versus-twelve tension is misreported."** Attempted: as in the prior round, `matrix_M4_stop_round3.md` and D-324 disagree on how many lines fact 4's paste omitted. **Rejected as U4's** — U4:1220–1222 follows D-324's *"truncated by one line"* exactly; the tension is between the stop record and D-324 and is owned there.
- **"U4 cites other units without their u-rev at ~20 sites, against its own LABEL DISCIPLINE rule at U4:113."** Attempted: `grep -on "\*\*U[123]\*\* §[0-9.]*"` returns bare citations at 137, 370, 379, 380, 495, 556, 559, 612, 677, 698, 763, 794, 796, 820, 1472, 1490, 1505, 1548, 1549. **Rejected as out of scope and not a u-rev 7 regression** — the pattern predates this u-rev, most of the hits are inside carried-verbatim or record text, no prior round charged it, and the dispatch's item 6 scopes this review to the citations that *do* carry a u-rev. Noted for the architect rather than charged.

---

**Cross-unit items noted and not charged to U4:** `docs/experiments/section_owner_table.md` §11 records U4 at its u-rev 6 size (`1413`), which u-rev 7 supersedes; U4 correctly records this as a standing duty owed to the owner table rather than restating a size, so nothing here is U4's. `matrix_M4_axisA_selection.md`'s condition list and D-329's condition list differ in content while both numbering four — that divergence is the two records', and MAJOR 3 is charged only for U4's misattribution across it.

---

*REVIEW-design of `docs/experiments/U4_soundness_instrument.md` u-rev 7, at pinned revision `0f49c90` (matches HEAD, tree clean at entry and exit). Fresh context; not the author of the unit, its repair, either matrix, any red team, or any earlier review. Every finding reproduced before reporting; every rejected charge recorded with its attempted reproducer. No build and no worktree were required. This report modifies no repository file, ran no git write command, and is not committed.*

**VERDICT: FAIL — 0 BLOCKING, 4 MAJOR, 4 MINOR.**
