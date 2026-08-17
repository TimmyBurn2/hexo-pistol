//! `pistol-eval` — position evaluation behind one trait.
//!
//! This crate will own the `Eval` trait and its implementations: the v0
//! handcrafted three-axis line-window pattern tables first, an incremental
//! codebook net later. The contract is incremental by construction —
//! apply/undo per placed stone — so that swapping the backend never becomes a
//! search change.
//!
//! The v0 weight table is committed configuration, not an artifact: it is a
//! handful of integers in `configs/eval_v0_weights.toml`, the file named by
//! `eval.weights_file` (docs/decisions.md D-11). The Stage-2 codebook net is
//! the artifact, and that one is never committed (CLAUDE.md rule 8). Either
//! way, a missing or malformed weights file is a loud load-time error raised
//! here and not a config parse error — config validation stays pure and
//! offline (docs/decisions.md D-21).
//!
//! Operator-confirmed Stage-0 values for that file, which WP-05 writes:
//! `table[1..=5] = 2 / 12 / 60 / 300 / 1500`. A length-6 window holding both
//! colours is dead and scores 0; every other window scores
//! `table[own_count] - table[opp_count]`. These are sanity values, tuned by
//! SPSA/Texel in Stage 4.
//!
//! WP-01 is workspace scaffold, config, errors and CI; it writes no eval logic.
