use pistol_core::window::{WINDOW_LEN, Window};
use pistol_core::{Axis, Board, Coord, Player};

/// One plan: the empty cells of one open window holding at least four own
/// stones, sorted. Size is at most two, because six cells less four own stones
/// leaves at most two.
pub type Plan = Vec<Coord>;

/// The plan family of `side`, deduplicated and sorted.
///
/// DEDUPLICATED because DEF-T is a hitting set over a FAMILY: two windows whose
/// empties coincide state one constraint, not two, and a repeated plan would
/// otherwise be visible in the fixture as noise nobody can read.
pub fn plan_family(board: &Board, side: Player) -> Vec<Plan> {
    let mut family: Vec<Plan> = open_windows(board, side)
        .into_iter()
        .filter(|&(_, own, _)| own >= 4)
        .map(|(window, _, _)| empties(board, window))
        .collect();
    family.sort();
    family.dedup();
    family
}

/// The plan family of `side` with the window each plan came from, for a caller
/// that decomposes the family — the crossing-fours counterexample splits by
/// axis. Not deduplicated: a window is the identity here.
pub fn plans_by_window(board: &Board, side: Player) -> Vec<(Window, Plan)> {
    let mut family: Vec<(Window, Plan)> = open_windows(board, side)
        .into_iter()
        .filter(|&(_, own, _)| own >= 4)
        .map(|(window, _, _)| (window, empties(board, window)))
        .collect();
    family.sort();
    family
}

/// DEF-SUPPORT: the largest own-stone count over the OPEN windows of `side`.
///
/// The positive shape a record with no plans still has. A four-stone rhombus
/// and an isolated open three both have an empty plan family; they are
/// different positions and this is the number that says so.
pub fn support(board: &Board, side: Player) -> u32 {
    open_windows(board, side)
        .into_iter()
        .map(|(_, own, _)| own)
        .max()
        .unwrap_or(0)
}

/// DEF-T: the size of a minimum hitting set of `family`, computed EXACTLY and
/// with no ceiling, by trying sizes smallest first — which is the definition.
///
/// An empty family has `t = 0`: nothing to hit. A family containing an empty
/// plan is unhittable, and that is a position where the side to move has
/// already completed six, so it cannot arise from a legal position this pack
/// uses; it panics rather than returning a number nobody defined.
pub fn threat_number(family: &[Plan]) -> usize {
    if family.is_empty() {
        return 0;
    }
    assert!(
        family.iter().all(|plan| !plan.is_empty()),
        "a plan with no empty cell is a completed six, not a threat"
    );
    // Picking one cell out of each plan hits every plan, so the minimum is at
    // most the family size. The loop is therefore total.
    let universe = universe(family);
    for size in 1..=family.len() {
        if subsets(&universe, size)
            .into_iter()
            .any(|candidate| hits_all(family, &candidate))
        {
            return size;
        }
    }
    unreachable!("one cell per plan hits every plan")
}

/// The sub-family of `plans_by_window` running along `axis` — the decomposition
/// the crossing-fours counterexample is stated over.
pub fn along(axis: Axis, plans: &[(Window, Plan)]) -> Vec<Plan> {
    let mut family: Vec<Plan> = plans
        .iter()
        .filter(|(window, _)| window.axis == axis)
        .map(|(_, plan)| plan.clone())
        .collect();
    family.sort();
    family.dedup();
    family
}

/// Every window that is OPEN for `side` — at least one own stone and no
/// opponent stone (DEF-WINDOW) — with its own count and its empty count.
///
/// Enumerated BY POSITION over the stones' bounding box grown by [`WINDOW_LEN`],
/// never by stepping back from a stone. The box is wide enough by construction:
/// a window holding a stone starts at most `WINDOW_LEN - 1` steps back along its
/// axis, and one step moves each coordinate by at most one.
fn open_windows(board: &Board, side: Player) -> Vec<(Window, u32, u32)> {
    let Some((low, high)) = bounds(board) else {
        return Vec::new();
    };
    let grow = i32::try_from(WINDOW_LEN).expect("six fits");
    let mut found = Vec::new();
    for q in (i32::from(low.q) - grow)..=(i32::from(high.q) + grow) {
        for r in (i32::from(low.r) - grow)..=(i32::from(high.r) + grow) {
            let (Ok(q), Ok(r)) = (i16::try_from(q), i16::try_from(r)) else {
                continue;
            };
            for axis in Axis::ALL {
                let Some(window) = Window::new(axis, Coord::new(q, r)) else {
                    continue;
                };
                let (own, opp, empty) = census(board, window, side);
                if own >= 1 && opp == 0 {
                    found.push((window, own, empty));
                }
            }
        }
    }
    found.sort();
    found
}

/// What one window holds for `side`: own, opponent, empty.
fn census(board: &Board, window: Window, side: Player) -> (u32, u32, u32) {
    let (mut own, mut opp, mut empty) = (0, 0, 0);
    for cell in window.cells() {
        match board.get(cell) {
            Some(player) if player == side => own += 1,
            Some(_) => opp += 1,
            None => empty += 1,
        }
    }
    (own, opp, empty)
}

/// The empty cells of `window`, sorted.
fn empties(board: &Board, window: Window) -> Vec<Coord> {
    let mut cells: Vec<Coord> = window
        .cells()
        .into_iter()
        .filter(|&cell| board.get(cell).is_none())
        .collect();
    cells.sort_unstable();
    cells
}

/// The corners of the smallest axis-aligned `(q, r)` box holding every stone.
fn bounds(board: &Board) -> Option<(Coord, Coord)> {
    let mut stones = board.stones().map(|(at, _)| at);
    let first = stones.next()?;
    let (mut low, mut high) = (first, first);
    for at in stones {
        low = Coord::new(low.q.min(at.q), low.r.min(at.r));
        high = Coord::new(high.q.max(at.q), high.r.max(at.r));
    }
    Some((low, high))
}

/// Every cell in some plan, sorted and deduplicated.
fn universe(family: &[Plan]) -> Vec<Coord> {
    let mut cells: Vec<Coord> = family.iter().flatten().copied().collect();
    cells.sort_unstable();
    cells.dedup();
    cells
}

/// Every `size`-cell subset of `universe`, in sorted order. Any size, because
/// the ceiling this module exists to remove was the parameter that stopped at
/// two.
fn subsets(universe: &[Coord], size: usize) -> Vec<Vec<Coord>> {
    if size == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (index, &first) in universe.iter().enumerate() {
        for mut rest in subsets(&universe[index + 1..], size - 1) {
            let mut candidate = vec![first];
            candidate.append(&mut rest);
            out.push(candidate);
        }
    }
    out
}

/// Whether `cells` meets every plan.
fn hits_all(family: &[Plan], cells: &[Coord]) -> bool {
    family
        .iter()
        .all(|plan| cells.iter().any(|cell| plan.contains(cell)))
}

/// How a plan is written in the fixture and in a failure message: `{q,r}` or
/// `{q,r q,r}`.
pub fn plan_token(plan: &Plan) -> String {
    let cells: Vec<String> = plan.iter().map(|cell| cell.to_string()).collect();
    format!("{{{}}}", cells.join(" "))
}

/// How a whole family is written: `-` when it is empty, so that "no plans" is
/// spelled rather than blank.
pub fn plan_list(family: &[Plan]) -> String {
    if family.is_empty() {
        return String::from("-");
    }
    family.iter().map(plan_token).collect::<Vec<_>>().join(" ")
}
