use pistol_core::TURN_STONES;
use pistol_core::window::{WINDOW_LEN, Window};

use crate::state::THREAT_DESYNC;

/// How many classes are maintained per side.
pub(crate) const CLASS_COUNT: usize = 5;

/// The own-stone count at which a window is HOT: a whole turn's stones short of
/// the win length.
///
/// D-243's counting identity, not a tuned threshold — a window at this count
/// has at most `TURN_STONES` empties and a turn places `TURN_STONES` stones, so
/// its owner on the move fills it.
const HOT_MIN: u32 = WINDOW_LEN - TURN_STONES;

/// The own-stone count at which a SINGLE stone completes the window.
const WIN_IN_ONE_PLY: u32 = WINDOW_LEN - 1;

// Three claims about the CLASSES, not restatements of the derived constants
// above: the greatest live count stays below hot, exactly one count separates
// hot from a win in one ply, and a turn's stones fill a hot window's slack
// exactly. The third guards the DERIVATION where the first two guard the
// values, and it is the one that fires when D-243's flip clause does.
// docs/decisions.md D-262 carries the argument for all three, including why
// the third is honestly vacuous at the constants as they stand.
const _: () = assert!(
    3 < HOT_MIN && WIN_IN_ONE_PLY == HOT_MIN + 1 && HOT_MIN + TURN_STONES == WINDOW_LEN,
    "hot must sit above every maintained live count, one stone below the win, and a turn's \
     stones short of it"
);

/// One maintained class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Class {
    /// Live with exactly two own stones.
    LiveTwo,
    /// Live with exactly three own stones.
    LiveThree,
    /// Live with at least [`HOT_MIN`] own stones — D-243's HOT.
    Hot,
    /// Live with exactly [`WIN_IN_ONE_PLY`] own stones — D-243's WIN-IN-ONE-PLY.
    WinInOnePly,
    /// Every cell own: the window is a completed run of [`WINDOW_LEN`].
    Completed,
}

impl Class {
    /// Every class, in a fixed order. Iterating this is deterministic, which is
    /// why nothing here iterates a set of classes instead.
    pub(crate) const ALL: [Class; CLASS_COUNT] = [
        Class::LiveTwo,
        Class::LiveThree,
        Class::Hot,
        Class::WinInOnePly,
        Class::Completed,
    ];

    /// This class's slot in a [`WindowSets`].
    const fn slot(self) -> usize {
        match self {
            Class::LiveTwo => 0,
            Class::LiveThree => 1,
            Class::Hot => 2,
            Class::WinInOnePly => 3,
            Class::Completed => 4,
        }
    }

    const fn bit(self) -> u8 {
        1u8 << self.slot()
    }
}

/// The classes one window falls into for one side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ClassSet(u8);

impl ClassSet {
    /// The classes a window with these counts is in **for the side those counts
    /// are read from**.
    ///
    /// The liveness conjunct is first and applies to every class: a window
    /// holding an opponent stone is dead for this side whatever it holds of
    /// this side's, so it is in no set at all.
    pub(crate) fn of(own: u32, opp: u32) -> ClassSet {
        if opp != 0 {
            return ClassSet(0);
        }
        let mut bits = 0u8;
        if own == 2 {
            bits |= Class::LiveTwo.bit();
        }
        if own == 3 {
            bits |= Class::LiveThree.bit();
        }
        if own >= HOT_MIN {
            bits |= Class::Hot.bit();
        }
        if own == WIN_IN_ONE_PLY {
            bits |= Class::WinInOnePly.bit();
        }
        if own == WINDOW_LEN {
            bits |= Class::Completed.bit();
        }
        ClassSet(bits)
    }

    /// Whether this window is in `class`.
    pub(crate) fn contains(self, class: Class) -> bool {
        self.0 & class.bit() != 0
    }
}

/// One side's five sorted window sets.
///
/// Each is an independently maintained sorted `Vec<Window>`: there is no shared
/// position field carried in the store, and none is possible, because the sets
/// OVERLAP — a window at own == 5 is in both `hot` and `win_in_one_ply`, and one
/// position per window cannot locate it in two sets at once. Insertion and
/// removal are `binary_search` then `insert`/`remove`, which at the measured
/// sizes (at most 21 at count 2, 6 at count 3, 4 hot) is a small memmove beside
/// a lookup that would cost the same search anyway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WindowSets {
    sets: [Vec<Window>; CLASS_COUNT],
}

impl Default for WindowSets {
    fn default() -> Self {
        WindowSets {
            sets: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl WindowSets {
    /// The windows in `class`, sorted by `(axis, start)`.
    ///
    /// Sorted BY CONSTRUCTION, not by sorting here: this is the ordering the
    /// determinism law cares about, and it is what lets the store underneath be
    /// hashed (CLAUDE.md rule 4).
    pub(crate) fn windows(&self, class: Class) -> &[Window] {
        &self.sets[class.slot()]
    }

    /// Move `window` from the classes it was in to the classes it is in now.
    ///
    /// At most two operations for the reason the module doc gives; nothing here
    /// depends on that bound being two, and a wrong bound would cost time
    /// rather than correctness.
    pub(crate) fn transition(&mut self, window: Window, before: ClassSet, after: ClassSet) {
        for class in Class::ALL {
            match (before.contains(class), after.contains(class)) {
                (true, false) => self.remove(window, class),
                (false, true) => self.insert(window, class),
                _ => {}
            }
        }
    }

    fn insert(&mut self, window: Window, class: Class) {
        let set = &mut self.sets[class.slot()];
        match set.binary_search(&window) {
            Err(at) => set.insert(at, window),
            // Reached only if this state and the stones it was fed disagree
            // about what is already recorded, which is a bug here and not an
            // answer to anyone's question (CLAUDE.md rule 3).
            Ok(_) => panic!("{THREAT_DESYNC}: {window:?} is already in {class:?}"),
        }
    }

    fn remove(&mut self, window: Window, class: Class) {
        let set = &mut self.sets[class.slot()];
        match set.binary_search(&window) {
            Ok(at) => {
                set.remove(at);
            }
            Err(_) => panic!("{THREAT_DESYNC}: {window:?} is not in {class:?} to remove"),
        }
    }
}
