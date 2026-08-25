//! The anchor report: results, interval, compute — written, not argued.
//!
//! RULE9-JUSTIFICATION: this module renders ONE result set three ways — the
//! seat tallies the walk computes, the JSON the ADR digests, and the plain-
//! text summary the operator reads first — over a shared tally walk that a
//! split would have to duplicate or hoist behind a boundary nobody else
//! needs; the three renderings are one responsibility.
//!
//! What the numbers are allowed to mean is fixed by the pre-registration
//! (docs/experiments/sealbot_anchor_prereg.md): wins by line are the sample;
//! capped games and forfeits are reported separately and excluded from the
//! interval, which is a Wilson 95% on decided games only. No Elo, no SPRT,
//! no strength claim beyond the anchor (docs/research/sealbot_notes.md,
//! D-197).

use pistol_core::Player;

use crate::referee::{GameResult, GameSummary};

/// Wilson score interval at 95% for `wins` out of `decided`.
///
/// `None` when nothing was decided — an interval over an empty sample is
/// not an interval, and the report says so instead.
pub fn wilson_95(wins: u64, decided: u64) -> Option<(f64, f64)> {
    if decided == 0 {
        return None;
    }
    let z = 1.96_f64;
    let n = decided as f64;
    let p = wins as f64 / n;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let centre = (p + z2 / (2.0 * n)) / denominator;
    let half = (z / denominator) * ((p * (1.0 - p) + z2 / (4.0 * n)) / n).sqrt();
    Some((0.0f64.max(centre - half), 1.0f64.min(centre + half)))
}

/// One engine's seat-split tallies.
#[derive(Debug, Default)]
pub struct SeatTally {
    pub win: u64,
    pub loss: u64,
    pub capped: u64,
    pub forfeit: u64,
    /// Wins that arrived by the opponent's forfeit — visible, not mixed in.
    pub win_by_opponent_forfeit: u64,
}

impl SeatTally {
    fn record(&mut self, summary: &GameSummary, a_perspective: bool) {
        match &summary.result {
            GameResult::Win { winner, .. } => {
                let a_won = (*winner == Player::P1) == summary.a_is_p1;
                if a_won == a_perspective {
                    self.win += 1;
                } else {
                    self.loss += 1;
                }
            }
            GameResult::Capped { .. } => self.capped += 1,
            GameResult::Forfeit { loser, .. } => {
                let a_lost = (*loser == Player::P1) == summary.a_is_p1;
                if a_lost == a_perspective {
                    self.forfeit += 1;
                } else {
                    self.win_by_opponent_forfeit += 1;
                }
            }
        }
    }
}

/// The whole match, tallied.
pub struct MatchReport {
    pub games: u32,
    pub turn_cap: u32,
    pub a_label: String,
    pub b_label: String,
    pub a_as_p1: SeatTally,
    pub a_as_p2: SeatTally,
    pub b_as_p1: SeatTally,
    pub b_as_p2: SeatTally,
    pub a_nodes_total: Option<u64>,
    pub a_wall_ms_total: u64,
    pub b_wall_ms_total: u64,
    pub interval: Option<(f64, f64)>,
    pub decided: u64,
    pub a_wins_decided: u64,
    pub summaries: Vec<GameSummary>,
}

impl MatchReport {
    /// Tally a finished match.
    pub fn assemble(
        games: u32,
        turn_cap: u32,
        a_label: &str,
        b_label: &str,
        summaries: Vec<GameSummary>,
    ) -> MatchReport {
        let mut a_as_p1 = SeatTally::default();
        let mut a_as_p2 = SeatTally::default();
        let mut b_as_p1 = SeatTally::default();
        let mut b_as_p2 = SeatTally::default();
        let mut a_nodes_total = Some(0u64);
        let mut a_wall_ms_total = 0u64;
        let mut b_wall_ms_total = 0u64;
        let mut decided = 0u64;
        let mut a_wins_decided = 0u64;
        for summary in &summaries {
            (if summary.a_is_p1 {
                &mut a_as_p1
            } else {
                &mut a_as_p2
            })
            .record(summary, true);
            (if summary.a_is_p1 {
                &mut b_as_p2
            } else {
                &mut b_as_p1
            })
            .record(summary, false);
            a_nodes_total = match (a_nodes_total, summary.a_nodes) {
                (Some(total), Some(delta)) => Some(total + delta),
                _ => None,
            };
            a_wall_ms_total += summary.a_wall_ms;
            b_wall_ms_total += summary.b_wall_ms;
            if let GameResult::Win { winner, .. } = &summary.result {
                decided += 1;
                if (*winner == Player::P1) == summary.a_is_p1 {
                    a_wins_decided += 1;
                }
            }
        }
        MatchReport {
            games,
            turn_cap,
            a_label: a_label.to_string(),
            b_label: b_label.to_string(),
            a_as_p1,
            a_as_p2,
            b_as_p1,
            b_as_p2,
            a_nodes_total,
            a_wall_ms_total,
            b_wall_ms_total,
            interval: wilson_95(a_wins_decided, decided),
            decided,
            a_wins_decided,
            summaries,
        }
    }

    /// The report as JSON. Transcripts are per-game files; this is the match.
    pub fn to_json(&self) -> serde_json::Value {
        let seat = |tally: &SeatTally| {
            serde_json::json!({
                "win": tally.win,
                "loss": tally.loss,
                "capped": tally.capped,
                "forfeit": tally.forfeit,
                "win_by_opponent_forfeit": tally.win_by_opponent_forfeit,
            })
        };
        let interval = match self.interval {
            None => serde_json::json!({"decided": 0, "note": "no decided games; no interval"}),
            Some((low, high)) => serde_json::json!({
                "decided": self.decided,
                "a_wins": self.a_wins_decided,
                "wilson_95_low": round4(low),
                "wilson_95_high": round4(high),
            }),
        };
        serde_json::json!({
            "anchor": true,
            "games": self.games,
            "turn_cap": self.turn_cap,
            "engines": { "a": self.a_label, "b": self.b_label },
            "a_as_p1": seat(&self.a_as_p1),
            "a_as_p2": seat(&self.a_as_p2),
            "b_as_p1": seat(&self.b_as_p1),
            "b_as_p2": seat(&self.b_as_p2),
            "interval": interval,
            "compute": {
                "a": {
                    "nodes_total": self.a_nodes_total,
                    "wall_ms_total": self.a_wall_ms_total,
                },
                "b": { "wall_ms_total": self.b_wall_ms_total },
            },
            "games_detail": self.summaries.iter().map(game_json).collect::<Vec<_>>(),
        })
    }

    /// The report as plain text, plain language first.
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        let a_total_win = self.a_as_p1.win + self.a_as_p2.win;
        let a_total_loss = self.a_as_p1.loss + self.a_as_p2.loss;
        let a_total_capped = self.a_as_p1.capped + self.a_as_p2.capped;
        let a_total_forfeit = self.a_as_p1.forfeit + self.a_as_p2.forfeit;
        let a_by_forfeit = self.a_as_p1.win_by_opponent_forfeit + self.a_as_p2.win_by_opponent_forfeit;
        text.push_str("ANCHOR match (not SPRT, not paired, not an Elo claim)\n");
        text.push_str(&format!(
            "{} vs {}: {} games, turn cap {}\n\n",
            self.a_label, self.b_label, self.games, self.turn_cap
        ));
        text.push_str(&format!(
            "{:^12} {:>5} {:>5} {:>7} {:>8} {:>9}\n",
            "seat", "win", "loss", "capped", "forfeit", "byOppFf"
        ));
        text.push_str(&format!(
            "{:^12} {:>5} {:>5} {:>7} {:>8} {:>9}\n",
            format!("{} p1", self.a_label),
            self.a_as_p1.win,
            self.a_as_p1.loss,
            self.a_as_p1.capped,
            self.a_as_p1.forfeit,
            self.a_as_p1.win_by_opponent_forfeit
        ));
        text.push_str(&format!(
            "{:^12} {:>5} {:>5} {:>7} {:>8} {:>9}\n",
            format!("{} p2", self.a_label),
            self.a_as_p2.win,
            self.a_as_p2.loss,
            self.a_as_p2.capped,
            self.a_as_p2.forfeit,
            self.a_as_p2.win_by_opponent_forfeit
        ));
        text.push_str(&format!(
            "\nA totals: {} W / {} L / {} capped / {} forfeited ({} wins arrived by the \
             opponent's forfeit, counted separately)\n",
            a_total_win, a_total_loss, a_total_capped, a_total_forfeit, a_by_forfeit
        ));
        match self.interval {
            None => text.push_str("No decided games; no interval.\n"),
            Some((low, high)) => text.push_str(&format!(
                "Wilson 95% on decided games: A won {} of {} => [{:.3}, {:.3}]\n",
                self.a_wins_decided, self.decided, low, high
            )),
        }
        text.push_str(&format!(
            "\nCompute: A {} nodes, {:.1}s wall; B {:.1}s wall.\n",
            self.a_nodes_total.map(|n| n.to_string()).unwrap_or_else(|| "n/a".into()),
            self.a_wall_ms_total as f64 / 1000.0,
            self.b_wall_ms_total as f64 / 1000.0,
        ));
        text.push_str("\nPer game:\n");
        for summary in &self.summaries {
            let seat = if summary.a_is_p1 { "p1" } else { "p2" };
            text.push_str(&format!(
                "  game {:>3} (A {}): {} — {}\n",
                summary.game,
                seat,
                summary.kind(),
                game_detail(&summary.result),
            ));
        }
        text
    }
}

/// One game's detail line, shared with the transcript writer.
pub fn game_detail(result: &GameResult) -> String {
    match result {
        GameResult::Win {
            winner,
            turn,
            first_stone_win,
        } => format!(
            "winner {} at turn {}{}",
            seat_of(*winner),
            turn,
            if *first_stone_win { " (first-stone win)" } else { "" }
        ),
        GameResult::Capped { turn } => format!("no decision within {turn} turns"),
        GameResult::Forfeit { loser, why } => format!("{} forfeited: {}", seat_of(*loser), why),
    }
}

/// One game's JSON entry.
fn game_json(summary: &GameSummary) -> serde_json::Value {
    serde_json::json!({
        "game": summary.game,
        "a_is_p1": summary.a_is_p1,
        "kind": summary.kind(),
        "detail": game_detail(&summary.result),
        "turns_played": summary.turns.len(),
        "a_nodes": summary.a_nodes,
        "a_wall_ms": summary.a_wall_ms,
        "b_wall_ms": summary.b_wall_ms,
    })
}

/// p1/p2 for a player.
fn seat_of(player: Player) -> &'static str {
    if player == Player::P1 {
        "p1"
    } else {
        "p2"
    }
}

/// Four decimals, for stable diffs.
fn round4(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}
