mod common;

use pistol_cli::corpus::audit::{Audit, CLAIMED_MIN_MOVES, audit_game};
use pistol_cli::corpus::record::Record;
use pistol_core::{Coord, Player};

fn game(hash: &str, moves: &[(i16, i16)]) -> Record {
    Record {
        game_hash: hash.to_string(),
        moves: moves.iter().map(|&(q, r)| Coord::new(q, r)).collect(),
        winner: Player::P1,
        elo: [Some(1500), Some(1500)],
    }
}

/// A decisive game: P1 builds `(0,0)..(0,5)` along ConstQ and the last stone
/// completes the six. Turn 1 is one stone, then two per turn, so P1's stones
/// land at flat indices 0, 3, 4, 7, 8, 11.
fn decisive() -> Vec<(i16, i16)> {
    // P2's stones are scattered rather than adjacent: a filler column would
    // build P2 a six of its own and decide the game before P1's last stone.
    // They sit outside rule 5's region, which `Board` accepts by design (D-35)
    // and which this audit does not ask about: the conjuncts under test are the
    // move floor and decisiveness, not reachability.
    vec![
        (0, 0),   // t1  P1  run 1
        (20, 0),  // t2  P2
        (23, 4),  // t2  P2
        (0, 1),   // t3  P1  run 2
        (0, 2),   // t3  P1  run 3
        (26, 8),  // t4  P2
        (29, 12), // t4  P2
        (0, 3),   // t5  P1  run 4
        (0, 4),   // t5  P1  run 5
        (32, 16), // t6  P2
        (35, 20), // t6  P2
        (0, 5),   // t7  P1  run 6 -> wins on the last stone
    ]
}

#[test]
fn a_decisive_game_is_recognised_by_its_last_stone() {
    let audited = audit_game(&game("aaaa000000000001", &decisive()));
    assert!(audited.decisive, "the last stone completes the six");
    assert!(!audited.decided_early, "nothing won before it");
    assert!(audited.rated);
    assert_eq!(audited.moves, 12);
}

#[test]
fn a_game_whose_last_stone_completes_nothing_is_flagged_indecisive() {
    // The negative control for the `decisive by six-in-a-row` conjunct: a clean
    // pass from an audit that cannot fail is not a pass.
    let mut moves = decisive();
    moves.pop();
    moves.push((7, 7));
    let audited = audit_game(&game("aaaa000000000002", &moves));
    assert!(
        !audited.decisive,
        "the six is never completed, so the record is not decisive"
    );
}

#[test]
fn a_game_that_continued_past_a_win_is_flagged() {
    let mut moves = decisive();
    moves.push((8, 8));
    moves.push((8, 9));
    let audited = audit_game(&game("aaaa000000000003", &moves));
    assert!(
        audited.decided_early,
        "a run completed before the final stone"
    );
}

#[test]
fn each_conjunct_denies_the_filter_on_its_own() {
    // The floor fixture used to be a short game that ALSO completed no run, so
    // `filter_holds` failed through the decisiveness branch and a mutant that
    // dropped the short conjunct survived. Each conjunct is now failed alone.
    let short_but_decisive = built(16, Player::P1, &six());
    assert!(short_but_decisive.len() < CLAIMED_MIN_MOVES);
    let only_short = Audit::of(&[game("baaa000000000001", &short_but_decisive)]);
    assert_eq!(only_short.short.len(), 1);
    assert!(only_short.indecisive.is_empty(), "it IS decisive");
    assert!(only_short.malformed.is_empty());
    assert!(!only_short.filter_holds(), "the move floor alone denies it");

    let mut long_but_indecisive = padded_to(CLAIMED_MIN_MOVES + 2);
    let last = long_but_indecisive.len() - 1;
    long_but_indecisive[last] = (70, 70);
    let only_indecisive = Audit::of(&[game("baaa000000000002", &long_but_indecisive)]);
    assert!(only_indecisive.short.is_empty(), "it is long enough");
    assert_eq!(only_indecisive.indecisive.len(), 1);
    assert!(only_indecisive.malformed.is_empty());
    assert!(
        !only_indecisive.filter_holds(),
        "decisiveness alone denies it"
    );
}

#[test]
fn the_median_is_the_lower_middle_of_an_even_corpus() {
    // M8: `(total-1)/2` becoming `total/2` picks the upper middle. Four games
    // whose two middles differ is the smallest fixture that can tell them apart.
    let lengths = [20usize, 21, 24, 25];
    let records: Vec<_> = lengths
        .iter()
        .enumerate()
        .map(|(index, &length)| {
            game(
                &format!("caaa00000000000{index:x}"),
                &built(length, Player::P1, &six()),
            )
        })
        .collect();
    let audit = Audit::of(&records);
    assert_eq!(audit.total, 4);
    assert_eq!(
        audit.median(),
        Some(21),
        "the lower middle of 20,21,24,25 is 21 and the upper is 24"
    );
    assert!(format!("{audit}").contains("median 21"));
}

#[test]
fn the_rendered_block_lists_every_decile_row_and_the_real_rated_count() {
    // M1 (Display prints one decile row) and M5 (rated printed as total) both
    // survived a suite that checked `deciles()` through the accessor and never
    // through the rendered block the numbers are transcribed from.
    let lengths = [20usize, 21, 24, 25, 28, 29, 32, 33, 36, 37, 40];
    let mut records: Vec<_> = lengths
        .iter()
        .enumerate()
        .map(|(index, &length)| {
            game(
                &format!("daaa00000000000{index:x}"),
                &built(length, Player::P1, &six()),
            )
        })
        .collect();
    // One game with a null rating, so `rated` and `total` differ in the block.
    let mut unrated = game("daaa00000000000f", &padded_to(CLAIMED_MIN_MOVES));
    unrated.elo = [Some(1500), None];
    records.push(unrated);

    let audit = Audit::of(&records);
    let rendered = format!("{audit}");
    // Against the accessor, which `the_deciles_are_bound_at_every_step` pins
    // independently: every row it computes must actually reach the block.
    let deciles = audit.deciles();
    assert_eq!(deciles.len(), 11, "one row per decile");
    for (percentile, length) in deciles {
        let row = format!("  {percentile:<11} {length}");
        assert!(
            rendered.contains(&row),
            "decile row `{row}` missing from:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("rating present both sides   11 of 12"),
        "the rated count is the measured one, not the total:\n{rendered}"
    );
}

#[test]
fn a_game_below_the_move_floor_is_flagged_short() {
    // The negative control for the `>= 20 moves` conjunct.
    let short: Vec<(i16, i16)> = (0..(CLAIMED_MIN_MOVES as i16 - 1))
        .map(|index| (index, 0))
        .collect();
    let audited = audit_game(&game("aaaa000000000004", &short));
    assert!(!audited.meets_floor(), "19 moves is under the floor of 20");

    let audit = Audit::of(&[game("aaaa000000000005", &short)]);
    assert_eq!(audit.short, vec![0], "and the corpus audit names it");
    assert!(!audit.filter_holds(), "so source_filter does not hold");
}

#[test]
fn a_game_missing_a_rating_is_not_counted_as_rated() {
    let mut record = game("aaaa000000000006", &decisive());
    record.elo = [Some(1500), None];
    assert!(!audit_game(&record).rated);
    assert_eq!(Audit::of(&[record]).rated, 0);
}

#[test]
fn the_reported_distribution_is_bound_to_the_games() {
    let long: Vec<(i16, i16)> = (0..40i16).map(|index| (index, 0)).collect();
    let audit = Audit::of(&[
        game("bbbb000000000001", &decisive()),
        game("bbbb000000000002", &long),
        game("bbbb000000000003", &long),
    ]);
    assert_eq!(audit.total, 3);
    assert_eq!(audit.min(), Some(12));
    assert_eq!(audit.max(), Some(40));
    assert_eq!(audit.median(), Some(40));
    let rendered = format!("{audit}");
    assert!(rendered.contains("games audited           3"), "{rendered}");
    assert!(
        rendered.contains("move count  min 12  median 40  max 40"),
        "{rendered}"
    );
    let deciles = audit.deciles();
    assert_eq!(deciles.first(), Some(&(0, 12)));
    assert_eq!(deciles.last(), Some(&(100, 40)));
}

#[test]
fn the_audit_is_deterministic_over_two_runs() {
    let records = vec![
        game("cccc000000000001", &decisive()),
        game("cccc000000000002", &decisive()),
    ];
    let first = Audit::of(&records);
    let second = Audit::of(&records);
    assert_eq!(first, second);
    assert_eq!(format!("{first}"), format!("{second}"));
}

/// Which side owns the placement at flat index `i`: turn 1 is one stone for P1,
/// then two per turn alternating. Spelled out here so a fixture can be built to
/// land a specific stone on a specific side.
fn owner_at(index: usize) -> Player {
    let turn = if index == 0 { 1 } else { index.div_ceil(2) + 1 };
    if turn % 2 == 1 {
        Player::P1
    } else {
        Player::P2
    }
}

/// A game of `total` stones in which `winner`'s stones are exactly `run`, laid
/// down in order, and the other side's are scattered far apart forming no line.
///
/// `run`'s last stone lands on the game's last placement, so the record is
/// decisive on its final stone unless `total` exceeds what the run needs.
fn built(total: usize, winner: Player, run: &[(i16, i16)]) -> Vec<(i16, i16)> {
    // The turn structure decides who owns the last placement, so not every
    // length can end on `winner`'s stone: P1 owns flat indices 0,3,4,7,8,…
    // Round up to the next length that does, rather than silently building a
    // game whose final stone belongs to the other side.
    let mut total = total;
    while owner_at(total - 1) != winner {
        total += 1;
    }
    let mut wanted: Vec<usize> = (0..total).filter(|&i| owner_at(i) == winner).collect();
    assert!(
        wanted.len() >= run.len(),
        "{total} stones give {} placements to {winner:?}, fewer than the run needs",
        wanted.len()
    );
    // Put the run at the END of that side's placements, so its final stone is
    // as late as the turn structure allows.
    let start = wanted.len() - run.len();
    wanted.drain(..start);

    let mut moves: Vec<(i16, i16)> = Vec::with_capacity(total);
    let mut filler = 0i16;
    for index in 0..total {
        match wanted.iter().position(|&i| i == index) {
            Some(slot) => moves.push(run[slot]),
            None => {
                moves.push((200 + filler * 5, 300 + filler * 9));
                filler += 1;
            }
        }
    }
    moves
}

/// Six in a row along ConstQ, the axis `(0, 1)`.
fn six() -> Vec<(i16, i16)> {
    (0..6i16).map(|step| (0, step)).collect()
}

/// A P1 win padded out to exactly `moves` stones, so a fixture can sit ON the
/// move floor rather than beside it.
fn padded_to(moves: usize) -> Vec<(i16, i16)> {
    built(moves, Player::P1, &six())
}

#[test]
fn a_game_sitting_exactly_on_the_move_floor_is_not_short() {
    // B2: the only floor test used a 19-move game, and 19 fails `>= 20` and
    // `> 20` alike, so no test could tell the operators apart. The real corpus
    // has exactly two games at exactly 20 moves, so a `>=` silently becoming a
    // `>` turns the run into a D-456 STOP. This fixture sits ON the boundary.
    let on_floor = padded_to(CLAIMED_MIN_MOVES);
    assert_eq!(on_floor.len(), CLAIMED_MIN_MOVES);
    let audited = audit_game(&game("dddd000000000001", &on_floor));
    assert!(
        audited.meets_floor(),
        "a game of exactly {CLAIMED_MIN_MOVES} moves meets a floor of {CLAIMED_MIN_MOVES}"
    );
    assert!(audited.decisive, "and it is still decisive");

    let audit = Audit::of(&[game("dddd000000000002", &on_floor)]);
    assert!(
        audit.short.is_empty(),
        "so the corpus audit calls it nothing"
    );
    assert!(audit.filter_holds());
}

#[test]
fn the_move_floor_is_the_one_the_metadata_claims() {
    // m1: the constant was unbound, and the floor fixture was built FROM it, so
    // it moved with the constant. `source_filter` says ">=20 moves".
    assert_eq!(
        CLAIMED_MIN_MOVES, 20,
        "dataset_metadata.json's source_filter claims `rated, >=20 moves, \
         decisive by six-in-a-row`"
    );
}

/// A corpus holding one of each defect, so the AGGREGATION is bound and not
/// only the per-game flags.
fn mixed() -> Vec<pistol_cli::corpus::record::Record> {
    let clean = padded_to(CLAIMED_MIN_MOVES + 2);

    // Short: under the floor, and otherwise fine. 16 already ends on P1's
    // stone, so `built` does not round it back up over the floor.
    let short = built(16, Player::P1, &six());
    assert!(
        short.len() < CLAIMED_MIN_MOVES,
        "the fixture is genuinely short"
    );

    // Indecisive: the six is never completed, so the last stone wins nothing.
    let mut indecisive = clean.clone();
    let last = indecisive.len() - 1;
    indecisive[last] = (70, 70);

    // Ran past its win: the six completes, then two more stones follow.
    let mut past_win = clean.clone();
    past_win.push((80, 80));
    past_win.push((80, 81));

    // Does not replay: a cell repeats.
    let mut malformed = clean.clone();
    malformed[5] = malformed[0];

    vec![
        game("eeee000000000001", &clean),
        game("eeee000000000002", &short),
        game("eeee000000000003", &indecisive),
        game("eeee000000000004", &past_win),
        game("eeee000000000005", &malformed),
    ]
}

#[test]
fn every_corpus_level_count_is_bound_to_the_games_that_produced_it() {
    // B1 and MA3: deleting the line that records an indecisive game left all
    // seven tests green while the tool printed `8698 of 8698` over a corpus
    // with 500 non-decisive games. No test ever built an Audit CONTAINING a
    // defect, so every aggregate — and the rendered block every published
    // number is transcribed from — was a channel that could print anything.
    let audit = Audit::of(&mixed());
    assert_eq!(audit.total, 5);
    assert_eq!(audit.short, vec![1], "the short game, by index");
    assert_eq!(
        audit.indecisive,
        vec![2],
        "the one whose last stone wins nothing"
    );
    assert_eq!(
        audit.decided_early,
        vec![3],
        "the one that ran past its win"
    );
    assert_eq!(audit.malformed, vec![4], "the one that does not replay");
    assert_eq!(audit.decisive_games(), 4);
    assert!(!audit.filter_holds(), "so source_filter does not hold");

    // MA3: the rendered block, line by line. Five Display mutants survived a
    // suite that pinned only the header and the min/median/max line.
    let rendered = format!("{audit}");
    for line in [
        "games audited           5",
        ">= 20 moves           4 of 5 (1 short)",
        "last stone completes a run  4 of 5 (1 not)",
        "continued past a win        1",
        "malformed (repeated cell)   1",
        "rating present both sides   5 of 5",
    ] {
        assert!(rendered.contains(line), "missing `{line}` in:\n{rendered}");
    }
}

#[test]
fn a_game_that_ran_past_its_win_is_still_decisive_by_six_in_a_row() {
    // MA1: such a record IS decisive — the six is on the board. Its defect is
    // rule-4 conformance, reported separately. Folding it into the conjunct
    // would announce the dataset's metadata is false on other evidence, and a
    // false D-456 STOP is the expensive direction.
    let mut past_win = padded_to(CLAIMED_MIN_MOVES + 2);
    past_win.push((80, 80));
    past_win.push((80, 81));
    let audit = Audit::of(&[game("ffff000000000001", &past_win)]);
    assert_eq!(audit.decided_early, vec![0], "the rule-4 defect is named");
    assert!(
        audit.indecisive.is_empty(),
        "but it is not reported as failing the decisiveness conjunct"
    );
    assert!(audit.filter_holds(), "so source_filter still holds");
}

#[test]
fn a_record_that_does_not_replay_is_named_malformed_and_not_indecisive() {
    // MA2: the occupied-cell path used to return `decisive: false`, so a record
    // that does not replay announced `the metadata is wrong`. Wrong-shape input
    // gets its own name (rule 3).
    let mut repeated = padded_to(CLAIMED_MIN_MOVES + 2);
    repeated[5] = repeated[0];
    let audited = audit_game(&game("ffff000000000002", &repeated));
    assert!(audited.malformed);
    let audit = Audit::of(&[game("ffff000000000003", &repeated)]);
    assert_eq!(audit.malformed, vec![0]);
    assert!(
        audit.indecisive.is_empty(),
        "a record that does not replay is not a claim about decisiveness"
    );
    assert!(!audit.filter_holds(), "it still denies the audit");
}

#[test]
fn the_deciles_are_bound_at_every_step() {
    // MA4: `seen > want` becoming `seen >= want` survived the suite and moves
    // the 100th percentile of the real corpus from 715 to 657. A live channel,
    // not a hypothetical one.
    //
    // Eleven games, one per decile. The lengths are the ones the turn structure
    // allows to end on P1's stone (0,3,4,7,8,… as flat indices).
    let lengths = [20usize, 21, 24, 25, 28, 29, 32, 33, 36, 37, 40];
    let records: Vec<_> = lengths
        .iter()
        .enumerate()
        .map(|(index, &length)| {
            let moves = built(length, Player::P1, &six());
            assert_eq!(moves.len(), length, "the length is already turn-legal");
            game(&format!("aaab00000000000{index:x}"), &moves)
        })
        .collect();
    let audit = Audit::of(&records);
    assert_eq!(audit.total, 11);
    assert_eq!(
        audit.deciles(),
        lengths
            .iter()
            .enumerate()
            .map(|(step, &length)| (step * 10, length))
            .collect::<Vec<_>>(),
        "eleven games put exactly one at each decile"
    );
    assert_eq!(audit.min(), Some(20));
    assert_eq!(audit.max(), Some(40));
}

#[test]
fn the_owner_walk_alternates_rather_than_giving_every_stone_to_p1() {
    // M13 (owner never alternates) left the headline decisiveness figure at
    // `8698 of 8698` untouched, so nothing in a run evidenced the ownership
    // walk. A game P2 wins does: with every stone assigned to P1 the last
    // stone's owner is P1, and the record's own `winner` then contradicts it.
    let p2_win = built(CLAIMED_MIN_MOVES + 1, Player::P2, &six());
    let mut record = game("aaac000000000001", &p2_win);
    record.winner = Player::P2;

    let audited = audit_game(&record);
    assert!(audited.decisive, "P2's last stone completes the six");
    assert!(
        audited.winner_agrees,
        "and the walk assigns that stone to P2, which is who the record says won"
    );
    assert!(Audit::of(&[record]).winner_disagrees.is_empty());
}

#[test]
fn an_overline_counts_as_decisive_and_is_reported_as_one() {
    // RED-TEAM F7: 157 of 8698 corpus games end on a run of 7+, which game
    // rule 2 scores as a win. Counting them is independent evidence the SOURCE
    // platform scores overlines the same way, against an exact-six variant.
    let seven: Vec<(i16, i16)> = (0..7i16).map(|step| (0, step)).collect();
    let overline = built(CLAIMED_MIN_MOVES + 3, Player::P1, &seven);
    let audited = audit_game(&game("aaad000000000001", &overline));
    assert_eq!(
        audited.final_run,
        Some(7),
        "the last stone completes a seven"
    );
    assert!(audited.decisive, "an overline wins (game rule 2)");
    assert_eq!(
        Audit::of(&[game("aaad000000000002", &overline)]).overlines,
        1
    );
}

#[test]
fn a_record_whose_winner_contradicts_its_play_is_named() {
    // RED-TEAM F1: `winner` was parsed, range-checked and then never read, so a
    // record contradicting its own outcome passed. Measured 0/8698 on the real
    // corpus, and the check is sharp because both values are well represented.
    let clean = padded_to(CLAIMED_MIN_MOVES + 2);
    let mut wrong = game("aaae000000000001", &clean);
    wrong.winner = Player::P2;
    assert!(!audit_game(&wrong).winner_agrees);
    assert_eq!(Audit::of(&[wrong]).winner_disagrees, vec![0]);
    assert!(audit_game(&game("aaae000000000002", &clean)).winner_agrees);
}
