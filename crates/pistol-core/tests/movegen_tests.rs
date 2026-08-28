mod common;

use common::perft_positions::perft_case;
use pistol_core::{
    Board, Coord, CoreError, GameState, LEGAL_RADIUS, Outcome, Player, Turn, generate_turns,
    legal_placements,
};

/// The position of a fixture case, replayed through the state machine.
fn fixture_position(name: &str) -> GameState {
    let case = perft_case(name);
    GameState::from_plies(&case.plies).expect("a legal fixture game")
}

/// A cell, from the token the fixtures and the protocol both use.
fn cell(token: &str) -> Coord {
    token.parse().expect("a stone token")
}

#[test]
fn pair_legal_iff_some_ordering_legal() {
    // One stone on the board, P2 to move: the legal region is the radius-8
    // ball around the origin, and a pair may reach outside it through the ball
    // its own first stone opens (docs/decisions.md D-6).
    let state = GameState::from_plies(&[Coord::ORIGIN]).expect("the first stone");
    let turns = generate_turns(&state).expect("a turn boundary");

    let anchor = cell("8,0");
    let far = cell("16,0");
    assert_eq!(Coord::ORIGIN.distance(anchor), LEGAL_RADIUS);
    assert_eq!(Coord::ORIGIN.distance(far), 2 * LEGAL_RADIUS);
    assert_eq!(anchor.distance(far), LEGAL_RADIUS);

    let reachable = Turn::pair(anchor, far).expect("two distinct cells");
    assert!(
        turns.contains(&reachable),
        "{reachable} is legal as anchor first, so it is a turn"
    );

    // Neither cell of this pair may go down first, so no ordering plays it —
    // and it is not a turn, however close the two cells are to each other.
    let neighbour_of_far = cell("16,1");
    assert_eq!(far.distance(neighbour_of_far), 1);
    assert!(Coord::ORIGIN.distance(neighbour_of_far) > LEGAL_RADIUS);
    let unreachable = Turn::pair(far, neighbour_of_far).expect("two distinct cells");
    assert!(
        !turns.contains(&unreachable),
        "{unreachable} has no legal ordering: both cells are outside the region"
    );
    let mut position = state.clone();
    assert_eq!(
        position.make_turn(unreachable),
        Err(CoreError::OutsideLegalRegion { at: far })
    );
    assert_eq!(position, state, "a refused turn changed the position");

    // The same pair mirrored, so that the cell that has to go down first is the
    // canonically SECOND one: generation still has it, and making it plays the
    // reverse ordering.
    let mirrored_anchor = cell("-8,0");
    let mirrored_far = cell("-16,0");
    let mirrored = Turn::pair(mirrored_anchor, mirrored_far).expect("two distinct cells");
    assert_eq!(
        mirrored,
        Turn::Pair(mirrored_far, mirrored_anchor),
        "canonical order"
    );
    assert!(turns.contains(&mirrored));

    let mut position = state.clone();
    assert_eq!(position.make_turn(mirrored), Ok(Outcome::Ongoing));
    let played: Vec<Coord> = position.played().map(|(at, _)| at).collect();
    assert_eq!(
        played,
        vec![Coord::ORIGIN, mirrored_anchor, mirrored_far],
        "the anchor had to be played first"
    );

    // Where both orderings are legal, the canonical one is played: the same
    // turn always leaves the same ply history behind, whatever order the caller
    // was thinking of (CLAUDE.md rule 4).
    let (near, next) = (cell("1,0"), cell("2,0"));
    let either_way = Turn::pair(next, near).expect("two distinct cells");
    let mut position = state.clone();
    assert_eq!(position.make_turn(either_way), Ok(Outcome::Ongoing));
    let played: Vec<Coord> = position.played().map(|(at, _)| at).collect();
    assert_eq!(played, vec![Coord::ORIGIN, near, next]);
}

#[test]
fn winning_first_stone_truncates_pair_in_movegen() {
    // P1 to move with 0,0 through 4,0: both ends of the five complete it.
    let state = fixture_position("a_win_the_mover_can_take");
    let turns = generate_turns(&state).expect("a turn boundary");
    let (low, high) = (cell("-1,0"), cell("5,0"));

    let singles: Vec<Turn> = turns
        .iter()
        .copied()
        .filter(|turn| matches!(turn, Turn::Single(_)))
        .collect();
    assert_eq!(
        singles,
        vec![Turn::Single(low), Turn::Single(high)],
        "rule 4: exactly the two cells that complete the line are turns of one stone"
    );

    // Neither ordering of the two winning cells plays a pair: whichever goes
    // down first ends the turn.
    let both_win = Turn::pair(low, high).expect("two distinct cells");
    assert!(!turns.contains(&both_win), "{both_win} is not a turn");

    // A stone that completes the line as a turn's SECOND stone is an ordinary
    // pair, and is a turn.
    let quiet = cell("0,-1");
    let wins_second = Turn::pair(quiet, high).expect("two distinct cells");
    assert!(turns.contains(&wins_second), "{wins_second} is a turn");

    // Making the truncated turn ends the game on the stone that completed the
    // line — the turn owes a second stone and never gets to play it.
    let mut position = state.clone();
    let turn_number = position.turn();
    assert_eq!(
        position.make_turn(Turn::Single(high)),
        Ok(Outcome::Win {
            winner: pistol_core::Player::P1,
            turn: turn_number
        })
    );
    assert_eq!(
        position.board().stone_count(),
        state.board().stone_count() + 1
    );
    assert_eq!(
        position.stones_owed(),
        0,
        "rule 4: nothing follows the line"
    );
    assert_eq!(
        generate_turns(&position).expect("a decided position"),
        Vec::new(),
        "a decided position has no turns"
    );

    // And a pair whose canonically first cell wins is played the other way
    // round, so that the pair is played rather than truncated.
    let quiet_high = cell("6,0");
    let ordered_around = Turn::pair(high, quiet_high).expect("two distinct cells");
    assert_eq!(
        ordered_around,
        Turn::Pair(high, quiet_high),
        "canonical order"
    );
    assert!(turns.contains(&ordered_around));
    let mut position = state.clone();
    assert!(matches!(
        position.make_turn(ordered_around),
        Ok(Outcome::Win { .. })
    ));
    let played: Vec<Coord> = position.played().map(|(at, _)| at).collect();
    assert_eq!(
        &played[played.len() - 2..],
        &[quiet_high, high],
        "the stone that wins had to be played second"
    );
}

#[test]
fn generation_and_the_turn_level_api_refuse_a_half_played_turn() {
    // Two plies is turn 1's stone and the first stone of turn 2: a position
    // that owes one stone is not at a turn boundary.
    let mut state = GameState::from_plies(&[Coord::ORIGIN, cell("1,0")]).expect("a legal game");
    let refusal = CoreError::TurnInProgress { turn: 2 };
    assert_eq!(generate_turns(&state), Err(refusal));
    assert_eq!(state.make_turn(Turn::Single(cell("2,0"))), Err(refusal));
    assert_eq!(state.unmake_turn(), Err(refusal));
    assert_eq!(pistol_core::perft(&mut state, 1), Err(refusal));
    // Depth 0 too. `perft(0)` is 1 at a turn boundary (docs/decisions.md D-54),
    // and here it would be the one unlocked door into an API D-50 defines only
    // there — a count for a position perft has no meaning on.
    assert_eq!(pistol_core::perft(&mut state, 0), Err(refusal));
}

#[test]
fn legal_placements_are_the_cells_the_cell_probe_calls_legal() {
    // Two forms of rule 5 — enumerate the balls, and probe one cell — have to
    // agree, or the search and the rules layer are answering different
    // questions (CLAUDE.md rule 2).
    for name in [
        "one_stone_at_the_origin",
        "two_lobes_joined_by_a_bridge",
        "a_win_the_mover_can_take",
    ] {
        let state = fixture_position(name);
        let board = state.board();
        let enumerated = legal_placements(board);
        assert_eq!(enumerated, probe_over_box(board), "case `{name}`");

        let mut sorted = enumerated.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, enumerated, "case `{name}`: ascending and unique");
    }

    // An empty board is the origin alone (rule 3), not an empty region.
    assert_eq!(legal_placements(&Board::empty()), vec![Coord::ORIGIN]);
}

#[test]
fn legal_placements_stop_at_the_edge_of_the_addressable_lattice() {
    // A synthetic position no game reaches (docs/decisions.md D-35): a stone
    // near the far corner of the i16 lattice, where part of its ball has no
    // cells at all. A cell that cannot be addressed is not a cell, so it is not
    // a placement either (D-47) — and enumerating the region must answer that
    // rather than overflow a coordinate (D-34).
    let corner = Coord::new(i16::MAX - 2, i16::MAX - 2);
    let mut board = Board::empty();
    board.apply(corner, Player::P1).expect("an empty cell");

    let cells = legal_placements(&board);
    assert!(
        cells
            .iter()
            .all(|&at| at.distance(corner) <= LEGAL_RADIUS && at != corner),
        "every placement is in the ball, and the stone's own cell is not one"
    );
    assert!(
        cells.contains(&Coord::new(i16::MAX, i16::MAX - 2))
            && cells.contains(&Coord::new(i16::MAX - 2, i16::MAX)),
        "the cells that do exist at the edge are still placements"
    );
    let ball_at_the_origin = legal_placements(&{
        let mut board = Board::empty();
        board
            .apply(Coord::ORIGIN, Player::P1)
            .expect("an empty cell");
        board
    });
    assert!(
        cells.len() < ball_at_the_origin.len(),
        "the lattice edge clips the ball"
    );
}

#[test]
fn generation_is_deterministic() {
    // Same position, same turns, in the same order — twice in one process, and
    // again from a state replayed from scratch (CLAUDE.md rule 4).
    let case = perft_case("tight_cluster_at_a_turn_boundary");
    let state = GameState::from_plies(&case.plies).expect("a legal fixture game");
    let once = generate_turns(&state).expect("a turn boundary");
    let twice = generate_turns(&state).expect("a turn boundary");
    assert_eq!(once, twice);

    let replayed = GameState::from_plies(&case.plies).expect("a legal fixture game");
    assert_eq!(
        generate_turns(&replayed).expect("a turn boundary"),
        once,
        "a second state over the same move list generated a different order"
    );
}

/// The legal placements a box scan finds, asking the board one cell at a time.
///
/// The box reaches a cell beyond the far side of every ball, so a region that
/// ran wider than the rule allows would show up here as a disagreement rather
/// than as a cell nobody looked at.
fn probe_over_box(board: &Board) -> Vec<Coord> {
    let pad = i16::try_from(LEGAL_RADIUS).expect("the rule radius fits a coordinate") + 1;
    let stones: Vec<Coord> = board.stones().map(|(at, _)| at).collect();
    let min_q = stones.iter().map(|at| at.q).min().expect("some stone") - pad;
    let max_q = stones.iter().map(|at| at.q).max().expect("some stone") + pad;
    let min_r = stones.iter().map(|at| at.r).min().expect("some stone") - pad;
    let max_r = stones.iter().map(|at| at.r).max().expect("some stone") + pad;
    let mut cells = Vec::new();
    for q in min_q..=max_q {
        for r in min_r..=max_r {
            let cell = Coord::new(q, r);
            if board.is_legal_placement(cell) {
                cells.push(cell);
            }
        }
    }
    cells
}
