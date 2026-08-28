mod common;

use common::repo;
use pistol_cli::corpus::distance::PlacementDistances;
use pistol_cli::sha256::sha256_hex;
use pistol_core::{Coord, LEGAL_RADIUS};

/// The SHA-256 of `corpus/puzzles/hexo_discord_v1.jsonl` (D-37: the pin fires in
/// the test that reads the fixture, so `cargo test` alone catches drift).
///
/// D-438: this pins WHAT WAS EXTRACTED and is never a truth pin — the mapping
/// behind these coordinates is verified only up to a point-group element.
const PUZZLES_SHA256: &str = "56a632e3c830403b414d47a13af0da9cd658b6039a8d40d52c4267d2a4816dd1";

/// Every puzzle's cells, in the payload's placement order.
///
/// Split on the exact spelling the pinned bytes carry. That is sound precisely
/// because the SHA above is asserted first: the file cannot re-space without
/// the pin firing.
fn puzzle_games() -> Vec<Vec<Coord>> {
    let path = repo("corpus/puzzles").join("hexo_discord_v1.jsonl");
    let bytes = std::fs::read(&path).expect("the corpus is committed");
    assert_eq!(
        sha256_hex(&bytes),
        PUZZLES_SHA256,
        "corpus/puzzles/hexo_discord_v1.jsonl has drifted from the pin in this test"
    );
    let text = String::from_utf8(bytes).expect("utf-8");

    text.lines()
        .map(|line| {
            let cells = line
                .split_once("\"cells\": [")
                .expect("every record carries cells")
                .1;
            let cells = cells.split_once("}]").expect("the array closes").0;
            cells
                .split("{\"q\": ")
                .skip(1)
                .map(|cell| {
                    let (q, rest) = cell.split_once(", \"r\": ").expect("q then r");
                    let r = rest.split(',').next().expect("r is followed by player");
                    Coord::new(q.parse().expect("q is an integer"), r.parse().expect("r"))
                })
                .collect()
        })
        .collect()
}

/// The turn-mate-inclusive criterion (D-218, D-219) over the puzzle corpus.
fn shipped() -> PlacementDistances {
    let mut distances = PlacementDistances::new();
    for game in puzzle_games() {
        distances.add_game(&game);
    }
    distances
}

#[test]
fn the_puzzle_corpus_holds_no_placement_beyond_the_legal_radius() {
    let distances = shipped();
    assert_eq!(
        distances.unrescuable_beyond(LEGAL_RADIUS),
        0,
        "an order-independent placement beyond LEGAL_RADIUS is a rules-truth \
         escalation to the operator (D-101, D-218), never a code change"
    );
    assert_eq!(
        distances.max(),
        Some(LEGAL_RADIUS),
        "the furthest placement sits exactly at the radius the rule pins"
    );
}

#[test]
fn the_criterion_measures_against_turn_mates_and_not_the_pre_turn_board() {
    // WP-P1's F-4 measured each stone against the board as it stood BEFORE its
    // own turn, which is D-218's order-sensitive artifact: it reported twelve
    // placements at distance 9 over this same corpus, and zero once turn-mates
    // were included. Pinning the two apart is what stops a third rediscovery
    // (D-440) — if `PlacementDistances` ever drifts toward the variant, this
    // fails rather than quietly reporting a rules-truth escalation that is not
    // there.
    let variant = pre_turn_board_variant();
    let shipped = shipped();

    assert_eq!(shipped.max(), Some(8));
    assert_eq!(variant.max(), Some(9), "the artifact is reproduced");
    assert_ne!(
        shipped.max(),
        variant.max(),
        "the two criteria genuinely disagree on this corpus, so the test can fail"
    );
    assert_eq!(
        shipped.unrescuable_beyond(LEGAL_RADIUS),
        0,
        "no escalation under the criterion that is sound"
    );
    assert_eq!(
        variant.beyond(LEGAL_RADIUS),
        12,
        "and twelve under the one that is not"
    );
}

/// The WRONG measurement, written out so the test above can contrast with it.
///
/// Deliberately not a call into `PlacementDistances`: the point is that this
/// shape does not exist in shipped code, and a copy here is what keeps it from
/// being written into shipped code again.
struct Variant {
    counts: Vec<(u32, usize)>,
}

impl Variant {
    fn max(&self) -> Option<u32> {
        self.counts.iter().map(|&(distance, _)| distance).max()
    }

    fn beyond(&self, radius: u32) -> usize {
        self.counts
            .iter()
            .filter(|&&(distance, _)| distance > radius)
            .map(|&(_, count)| count)
            .sum()
    }
}

fn pre_turn_board_variant() -> Variant {
    let mut counts: Vec<(u32, usize)> = Vec::new();
    for game in puzzle_games() {
        let mut placed: Vec<Coord> = Vec::new();
        let mut index = 0usize;
        let mut turn = pistol_core::FIRST_TURN;
        while index < game.len() {
            let owed = (pistol_core::stones_in_turn(turn) as usize).min(game.len() - index);
            if !placed.is_empty() {
                for stone in &game[index..index + owed] {
                    let nearest = placed
                        .iter()
                        .map(|earlier| earlier.distance(*stone))
                        .min()
                        .expect("the board is not empty");
                    match counts.iter_mut().find(|(key, _)| *key == nearest) {
                        Some((_, count)) => *count += 1,
                        None => counts.push((nearest, 1)),
                    }
                }
            }
            placed.extend_from_slice(&game[index..index + owed]);
            index += owed;
            turn += 1;
        }
    }
    Variant { counts }
}
