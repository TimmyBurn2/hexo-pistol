use std::collections::BTreeMap;

use pistol_core::window::{WINDOW_LEN, Window, windows_through_indexed};
use pistol_core::{Axis, Coord, Player};

use crate::line::{Chunk, LinePos, LineStore};
use crate::sets::{Class, ClassSet, WindowSets};
use crate::table::{FULL_MASK, WindowMasks};

/// Named invariant: the stones this state was fed and the stones a caller
/// believes it holds have drifted apart.
///
/// The token appears verbatim in the panic message, so a test can pin it and a
/// log can be grepped for it — the same contract `EVAL_DESYNC` carries in
/// `pistol-eval` (CLAUDE.md rule 3, docs/decisions.md D-45).
pub const THREAT_DESYNC: &str = "THREAT_DESYNC";

/// An axis's slot in a per-axis array, in `Axis::ALL` order.
const fn axis_slot(axis: Axis) -> usize {
    match axis {
        Axis::ConstQ => 0,
        Axis::ConstR => 1,
        Axis::ConstS => 2,
    }
}

/// Which side's sets a player's are.
const fn slot(side: Player) -> usize {
    match side {
        Player::P1 => 0,
        Player::P2 => 1,
    }
}

/// The incremental threat state: per-axis line bitboards, read window by
/// window, plus the ten sorted sets every query answers out of.
///
/// `undo` takes back THE LAST STONE APPLIED and refuses any other: the store
/// is restored from a log rather than recomputed, and a log can only be
/// replayed in reverse. Every caller unwinds in that order already; a caller
/// that cannot rebuilds through [`ThreatState::new`] and `apply`.
///
/// # The store is not exported, and that is CHECKED
///
/// The packed key, its hasher, the line store and the class sets are
/// `pub(crate)` and their modules are private. That is not tidiness: the whole
/// ground for the store being its own file is that a different store replaces
/// exactly that file and nothing else, and every internal name a consumer can
/// reach is a commitment that replacement would have to unwind
/// (docs/decisions.md D-254, D-261). It has happened once — the per-window
/// hashed table D-254 adopted became per-axis line bitboards behind the same
/// queries.
///
/// A doc sentence about visibility is falsified by any commit that
/// re-publishes what it names, silently and with every gate green. The first
/// example goes through the public door and compiles; the two after it reach
/// for the store's own and must not.
///
/// ```
/// use pistol_core::window::Window;
/// use pistol_core::{Axis, Coord};
/// let window = Window::new(Axis::ConstR, Coord::new(0, 0)).unwrap();
/// let state = pistol_solver::ThreatState::new();
/// let _ = state.masks(window);
/// ```
///
/// ```compile_fail
/// use pistol_core::window::Window;
/// use pistol_core::{Axis, Coord};
/// let window = Window::new(Axis::ConstR, Coord::new(0, 0)).unwrap();
/// let state = pistol_solver::ThreatState::new();
/// let _ = pistol_solver::table::empty_cells(window, state.masks(window));
/// ```
///
/// ```compile_fail
/// let _ = pistol_solver::line::unpack(0);
/// ```
///
/// A bare `compile_fail` passes on ANY compilation error, and the error-code
/// form does not repair it: this toolchain accepts `compile_fail,E0999` on code
/// whose real error is `E0603`. What makes the second example non-vacuous is
/// the FIRST: every line of it appears there and compiles, so the only line it
/// can fail on is the one that differs. WHAT THIS DOES NOT COVER: a re-export
/// of the same item under another path leaves both examples failing, so that
/// door is judged at the `pub use` list and is not mechanized (D-261).
#[derive(Debug, Clone, Default)]
pub struct ThreatState {
    table: LineStore,
    sets: [WindowSets; 2],
    /// Per touched window of every applied stone, what it held before and the
    /// class transitions the stone caused — what `undo` replays in reverse.
    log: Vec<Touched>,
    /// Per applied stone, the three chunks it flipped and what each held before.
    chunks: Vec<(u64, Chunk)>,
    /// The applied stones, newest last, each with its `log` entry count.
    applied: Vec<Applied>,
}

/// How far a window through a cell reaches along its line on either side.
const REACH: u32 = WINDOW_LEN - 1;

/// One window a stone touched, with each side's class set before and after.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Touched {
    window: Window,
    was: [ClassSet; 2],
    now: [ClassSet; 2],
}

/// One applied stone and how many `Touched` entries it pushed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Applied {
    at: Coord,
    player: Player,
    touched: u8,
}

// The log is bookkeeping for `undo`, not state: two states that hold the same
// windows in the same classes are the same state whatever path built them.
impl PartialEq for ThreatState {
    fn eq(&self, other: &ThreatState) -> bool {
        self.table == other.table && self.sets == other.sets
    }
}

impl Eq for ThreatState {}

impl ThreatState {
    /// An empty state: no stones, no entries, no set members.
    pub fn new() -> ThreatState {
        ThreatState::default()
    }

    /// One stone down.
    ///
    /// # Panics
    ///
    /// With [`THREAT_DESYNC`] if any window through `at` already holds a stone
    /// there. That means this state and the board it is supposed to mirror have
    /// drifted apart, which is a caller bug rather than operator input.
    pub fn apply(&mut self, at: Coord, player: Player) {
        // Per axis: the eleven positions around the stone before and after it,
        // read once and set once. The windows come from `pistol-core`'s own
        // enumeration below, never from a loop of this crate's (CLAUDE.md
        // rule 2), and each reads its masks out of its axis's run by shift.
        let mut runs = [(0u64, 0u64, 0u64, 0u64); Axis::ALL.len()];
        for axis in Axis::ALL {
            let here = LinePos::of(axis, at);
            let (p1, p2) = self
                .table
                .run(axis, here.line, here.pos - REACH as i32, 2 * REACH + 1);
            assert!(
                ((p1 | p2) >> REACH) & 1 == 0,
                "{THREAT_DESYNC}: {player} stone on {at} lands on a cell of its {axis:?} line \
                 that already holds one"
            );
            let (key, before) = self.table.place(axis, at, player);
            self.chunks.push((key, before));
            let (p1_after, p2_after) = match player {
                Player::P1 => (p1 | (1 << REACH), p2),
                Player::P2 => (p1, p2 | (1 << REACH)),
            };
            runs[axis_slot(axis)] = (p1, p2, p1_after, p2_after);
        }
        let mut touched = 0u8;
        for (window, index) in windows_through_indexed(at) {
            let (p1, p2, p1_after, p2_after) = runs[axis_slot(window.axis)];
            let shift = REACH - u32::from(index);
            let masks = |p1: u64, p2: u64| WindowMasks {
                p1: ((p1 >> shift) as u8) & FULL_MASK,
                p2: ((p2 >> shift) as u8) & FULL_MASK,
            };
            let before = masks(p1, p2);
            let after = masks(p1_after, p2_after);
            let mut was = [ClassSet::default(); 2];
            let mut now = [ClassSet::default(); 2];
            for side in [Player::P1, Player::P2] {
                let s = slot(side);
                was[s] = ClassSet::of(before.own_count(side), before.opp_count(side));
                now[s] = ClassSet::of(after.own_count(side), after.opp_count(side));
                if was[s] != now[s] {
                    self.sets[s].transition(window, was[s], now[s]);
                }
            }
            self.log.push(Touched { window, was, now });
            touched += 1;
        }
        self.applied.push(Applied {
            at,
            player,
            touched,
        });
    }

    /// The last stone applied, back up.
    ///
    /// # Panics
    ///
    /// With [`THREAT_DESYNC`] unless `player`'s stone at `at` is the most
    /// recently applied stone still down: a stone that was never applied, the
    /// other player's stone, or a stone applied earlier than the last are all
    /// refused. The first two are the drift the incumbent refused too; the
    /// third is the contract stated on the type.
    pub fn undo(&mut self, at: Coord, player: Player) {
        let last = self.applied.pop();
        let frame = match last {
            Some(frame) if frame.at == at && frame.player == player => frame,
            _ => panic!(
                "{THREAT_DESYNC}: taking back a {player} stone at {at}, which is not the last \
                 stone applied ({last:?})"
            ),
        };
        for _ in 0..frame.touched {
            let touched = self.log.pop().unwrap_or_else(|| {
                panic!("{THREAT_DESYNC}: the undo log is shorter than its own frame")
            });
            for s in [slot(Player::P1), slot(Player::P2)] {
                if touched.was[s] != touched.now[s] {
                    self.sets[s].transition(touched.window, touched.now[s], touched.was[s]);
                }
            }
        }
        for _ in Axis::ALL {
            let (key, before) = self.chunks.pop().unwrap_or_else(|| {
                panic!("{THREAT_DESYNC}: the chunk log is shorter than its own frame")
            });
            self.table.restore(key, before);
        }
    }

    /// What `window` holds, for both sides.
    pub fn masks(&self, window: Window) -> WindowMasks {
        self.table.masks(window)
    }

    /// The windows `side` holds in `class`, sorted by `(axis, start)`.
    pub(crate) fn class_windows(&self, side: Player, class: Class) -> &[Window] {
        self.sets[slot(side)].windows(class)
    }

    /// How many windows hold a stone. Enumerates the store: diagnostics only,
    /// never a choice path.
    pub fn window_count(&self) -> usize {
        self.table.snapshot().len()
    }

    /// Whether no window holds a stone.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Every window holding a stone, sorted by window — for oracles and
    /// diagnostics.
    ///
    /// Never on a choice path: see `LineStore::snapshot`, whose doc says why
    /// the store underneath may be hashed at all.
    pub fn table_snapshot(&self) -> BTreeMap<Window, WindowMasks> {
        self.table.snapshot()
    }
}
