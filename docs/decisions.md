# Decisions

Append-only. One line per non-obvious design choice, in the form

    D-n: choice — reason — what flips it

Lines are never edited or deleted. A decision that changes is superseded by a
new line that says which one it replaces. Silent architecture drift is a breach
(CLAUDE.md rule 10); if the code and a line here disagree, one of them is wrong
and it is usually the code.

---

D-1: Crate cut = CLAUDE.md map verbatim, all 8 crates at workspace init, solver/arena/api as doc-only stubs in Stage 0 — churn-free dependency graph from day 1, stubs pin the seams at zero cost — flips if workspace overhead measurably hurts CI (it won't).

D-2: Engine trait is synchronous, no stop verb in v0 (new_game / set_position / go(Budget) -> SearchOutcome) — at <=0.5 s request/response a stop verb buys nothing and forces async plumbing through the one seam the future API wraps; stop/ponder arrive with Lazy SMP (Stage 4) as an additive trait extension + protocol verb — flips when the API spec requires cancellation.

D-3: Score = Eval(i32) | MateIn(u16) | MatedIn(u16), distance in TURNS; internally MATE = 30_000, MateIn(t) = 30_000 - t, eval clamped to +/-16_000; MateIn(1) means the side to move completes >=6 with a stone placed this turn (first or second stone, same turn count per rule 4); a turn cap scores static eval at the horizon, never 0/draw (rule 6) — flips if solver integration needs proof/disproof score kinds (additive enum variants).

D-4: Budget is the closed enum DepthTurns(u32) | Nodes(u64) | MovetimeMs(u64); absent budget = BudgetMissing; MovetimeMs under mode=instrument = InstrumentBudgetUnsupported — all three budgets exist in play mode, but determinism/strength claims only ever come from depth or nodes — flips never (the schema shape is the decision; values are operator config).

D-5: Line protocol v0 mirrors the Engine trait 1:1; stone token "q,r" signed decimal, pair token "q,r/q,r" canonical lexicographic by (q, then r); verbs pistol/newgame/position/go/quit; every malformed or illegal input yields one line "error <NamedError>: <why>" and the engine stays alive — this protocol IS the future API's semantic contract (Hard Rule 11) — flips if the API spec demands a different position encoding (the move-list form stays canonical regardless).

D-6: The move list is the canonical position encoding (unbounded board, no FEN analog); position set exists for fixtures and carries tomove + phase, with the phase-1 stone identified as the last listed stone of the mover; a set position already containing a >=6 run = IllegalPosition; a pair is legal iff SOME ordering of its two placements is legal under rule 5; movegen and perft count UNORDERED pairs — flips if a fixture format needs richer state (extend set, never relax validation).

D-7: Determinism design: no randomized-state maps on any choice path (fixed-seed hasher or sorted structures), stable sorts with lexicographic (q, then r) final tie-break, deterministic TT replacement (depth-preferred + generation counter, never wall time), integer-only eval, instrument mode = 1 thread enforced by config validation; CI gate = two-process CLI runs over sha-pinned fixtures x budgets {depth_turns 4, nodes 200000}, diffing bestmove + nodes + full pv, any diff = fail; a same-process double-run unit test additionally catches state bleed across newgame — Hard Rule 4 made executable — flips never.

D-8: Zobrist keys are 128-bit, computed as a pure function of (q, r, color, FIXED_SEED) via two independent SplitMix64 streams (caching optional, never value-affecting), XORed with side-to-move and intra-turn phase keys; the search TT indexes from low bits and stores the high 64 bits as verification (~24 B packed entries, 4-entry buckets, deterministic replacement); the solver TT (Stage 1+) stores the full 128 bits plus GHI history hooks; coordinates are i16 with loud overflow asserts — deliberate narrowing of the report's blanket 128-bit line: 64 verification bits are ample for heuristic search, full 128 is reserved for proof soundness where the report's argument actually applies — flips on any observed TT-verification collision in the determinism fixtures, or if solver and search ever share a TT (they won't).

D-9: A turn is represented internally as two sequential same-side plies with the intra-turn phase bit in state and zobrist key; all external accounting (depth, mate distance, perft) is in TURNS over unordered pairs; win check runs after each ply and a phase-0 win ends the turn — the alpha-beta cutoff between the two plies is the main pruning lever (report Section A) and rule 4 falls out structurally — flips never at this design point.

D-10: One EngineError enum in pistol-engine wrapping crate-local errors: Config{key, why}, IllegalMove{turn, why}, IllegalPosition{why}, Protocol{line, why}, BudgetMissing, InstrumentBudgetUnsupported, InternalInvariant{what}; validation = serde deny_unknown_fields on every struct PLUS a validate() pass for cross-field constraints, each failure a named Config error carrying the exact editable key — fail loud (Hard Rule 3) — flips never.

D-11: Eval v0 = handcrafted, integer-only, incremental: for each of the 3 axes every length-6 window through a cell; a window containing both colors is dead (0), else table[own_count] - table[opp_count] with all table values explicit in configs/eval_v0_weights.toml; one stone updates <=18 windows (3 axes x 6 offsets); the Eval trait contract (apply/undo per placed stone, side-relative integer value) is the SAME trait the Stage-2 codebook net implements — cheapest eval that produces sane depth-4-6 play; sophistication is Stage 2's job — flips if depth-6 play is pathological in obvious ways (adjust weights, not architecture).

D-12: Test strategy: an independently written brute-force reference generator in the test tree is THE movegen oracle (full radius-8 region by scan, ordered enumeration, unordered dedupe, rule-4 truncation; exact count equality at depths 1-3 turns over origin + hand-built midgame fixtures incl. a two-cloud union case); sha-pinned golden boards pin rules 2 and 4 (exact-six per axis, overline-7, five-is-not-win, first-stone-win truncation, second-stone win); tactical-v0 = ~20 sha-pinned mate/must-block positions with a pass threshold pre-registered in the fixture header before the first run — Hard Rule 7 — flips never.

---

D-13: One cargo workspace, members `crates/*`, edition 2024, `publish = false`, and `unsafe_code` denied workspace-wide — one toolchain surface for eight crates, and unsafe becomes a visible, per-crate opt-out rather than something that arrives unannounced in a hot loop — a measured hotspot that needs unsafe, which then documents the opt-out in its own line here.

D-14: No `[profile.release]` tuning in WP-01 — codegen settings are a perf-sensitive change and rule 5 wants a pre-registered hotspot, an expected gain bracket and an IQR-gated bench; there is no code to bench yet — the first benched work package, which sets them with numbers attached.

D-15: Config is TOML, parsed by serde with `deny_unknown_fields` on every struct, no serde field defaults and no `Default` implementation anywhere — rule 1; a mistyped key must be an error, and a value exists because an operator wrote it — an explicitly optional section, which would need its own line here first.

D-16: `schema_version` is a required top-level integer checked against `SCHEMA_VERSION` — configs outlive binaries, and version skew must fail at load rather than at the first divergent behaviour — a specified migration path, which would replace the equality check with a range.

D-17: Parsing and validation are separate calls and the parse-only entry point is named `parse_unvalidated` — cross-field rules cannot be expressed in serde, and the blunt name makes a caller that skips validation visible at the call site — a typestate wrapper that makes an unvalidated config unrepresentable instead.

D-18: `MIN_TT_BYTES` and `MAX_CANDIDATE_RADIUS` are rejection bounds, not values — code never completes a config, it only refuses an impossible one, so a config that omits the key is still an error — a bound that becomes a tunable itself, at which point it moves into the schema.

D-19: `search.tt_bytes` must be a power of two and at least 1 MiB — the table will index by masking, and a size that is not a power of two would have to be rounded; rounding a stated value silently is the failure mode this project forbids — a cluster-based table with non-power-of-two bucket counts.

D-20: `search.candidate_policy.radius` is validated against its own ceiling and is never compared with the rules' radius-8 legal region — they are different concepts, one a search knob and one a game rule, and conflating them is the mistake CLAUDE.md rule 2 names explicitly — nothing; a coupling would need its own line here and a very good reason.

D-21: `eval.weights_file` is validated for shape, not existence — config validation stays pure, offline and deterministic, and a missing or corrupt weights file is pistol-eval's loud error at load time — a deployment gate that must catch a missing artifact before a match starts.

D-22: `Budget` is a closed enum, an absent budget is `EngineError::BudgetMissing`, a zero budget is a named config error, and instrument mode refuses `MovetimeMs` with `EngineError::InstrumentBudgetUnsupported` — wall-clock is not reproducible and instrument mode is the source of every strength claim (rules 1, 4, 6) — a deterministic simulated clock.

D-23: `EngineError` implements `Display` and `Error` by hand rather than taking a `thiserror` dependency — the enum is small and closed, and its text is part of the operator contract, so it is written rather than generated — enough variant growth that hand-written text starts to drift.

D-24: Deserialization failures become `EngineError::Config { key, why }` with the key recovered by `serde_path_to_error` and spliced with the member named in the message — every rejection has to name a key an operator can go and edit, and serde alone reports the container — a schema layer with native path reporting.

D-25: Config validation ships as the `pistol-engine` example `validate_config`, not a pistol-cli subcommand — pistol-cli's surface mirrors the `Engine` trait one to one (rule 11), and config checking is a tools-side gate — a CLI that grows a general maintenance surface.

D-26: `tools/ci.sh`'s fresh-build gate builds a checkout of the git index, not a clone of `HEAD` — the gate exists to catch a build that depends on an untracked file, and the index is the tracked set in every state: it equals `HEAD` on a fresh checkout and equals the about-to-be-committed tree when work is staged, so a clone of `HEAD` would pass a working tree that is about to break CI — anything the build needs that lives outside the index, such as a submodule or a generated file, which would make a clone the honest thing to build.

D-27: `tools/artifact_check.sh` applies rule 7's 10 MB per-file ceiling to every tracked file, not only to fixtures — no tracked file has any business being that large, and one rule is easier to obey than two — a legitimately large tracked file, which would need its own line here.

D-28: pistol-core carries no code in WP-01, the radius-8 constant included — a rules constant ships with its pinning test in the rules work package, and an untested constant that other crates start reading is worse than no constant at all — the rules work package landing.

D-29: The no-code-side-default guarantee is enforced by a `compile_fail` doctest on `Config` plus a source scan over `config.rs`, `validate.rs` and `budget.rs` in `config_rejects_code_side_default_probe` — a rule that only lives in a review checklist rots quietly; this makes reintroducing a default a red test — a lint or macro that can express the same guarantee directly.

D-30: `tools/ci.sh` runs `cargo fmt --all --check` as a gate — added post-WP-01 by operator instruction; style drift is review noise, and reviewers are here to flag correctness and requirement gaps — flips never.
