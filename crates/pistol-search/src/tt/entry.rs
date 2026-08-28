use pistol_core::Coord;

/// Bytes per packed entry.
pub const ENTRY_BYTES: usize = 24;

/// Generations before the counter wraps. Six bits, packed beside the bound.
pub const GENERATIONS: u8 = 64;

/// Named invariant: a value handed to the table does not fit the slot it packs
/// into.
pub const TT_FIELD_OUT_OF_RANGE: &str = "TT_FIELD_OUT_OF_RANGE";

/// Whether a record was written by a quiescence-regime node
/// (`crate::quiescence`) rather than the main search — a claim about SEARCH
/// WIDTH, orthogonal to `depth_plies` (docs/wp16_quiescence_design.md §6).
/// Stored at `depth_plies: 1` (the smallest non-empty value, D-393): depth
/// alone cannot distinguish a quiescence record from a genuine depth-1
/// full-width one, which is exactly why this flag exists.
pub const FLAG_FROM_QUIESCENCE: u8 = 0b0000_0001;

/// What a stored score says about the true value of the position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    /// The score is the value.
    Exact,
    /// The value is at least the score: the node failed high.
    Lower,
    /// The value is at most the score: the node failed low.
    Upper,
}

impl Bound {
    const fn index(self) -> u8 {
        match self {
            Bound::Exact => 0,
            Bound::Lower => 1,
            Bound::Upper => 2,
        }
    }

    fn from_index(index: u8) -> Bound {
        match index {
            0 => Bound::Exact,
            1 => Bound::Lower,
            2 => Bound::Upper,
            other => panic!(
                "pistol-search invariant {TT_FIELD_OUT_OF_RANGE}: {other} is not a bound index"
            ),
        }
    }
}

/// What the search stores about a position, and gets back.
///
/// The score is **root-relative** on both sides of the seam: the table re-bases
/// it on the way in and out ([`crate::score::to_table`]), so nothing outside the
/// table handles a node-relative distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    /// How deep the search that produced this score looked, in plies.
    pub depth_plies: u32,
    /// The score, from the point of view of the side to move at that position.
    pub score: i32,
    /// The static evaluation of that position, kept for the ordering and
    /// pruning decisions Stage 1 adds.
    pub static_eval: i32,
    /// What the score means.
    pub bound: Bound,
    /// The ply that scored best there — the first move the ordering tries.
    pub best: Coord,
    /// Whether a quiescence-regime node wrote this record. `false` for every
    /// full-width store, unchanged from before this field existed.
    pub from_quiescence: bool,
}

/// One packed entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    verification: u64,
    best_q: i16,
    best_r: i16,
    score: i16,
    static_eval: i16,
    depth_plies: u8,
    bound_age: u8,
    flags: u8,
}

/// A slot nothing has been written to.
pub const EMPTY: Entry = Entry {
    verification: 0,
    best_q: 0,
    best_r: 0,
    score: 0,
    static_eval: 0,
    depth_plies: 0,
    bound_age: 0,
    flags: 0,
};

const _: () = assert!(
    size_of::<Entry>() == ENTRY_BYTES,
    "the packed entry must be the size the table is sized in"
);

impl Entry {
    /// Pack a record whose score has already been re-based onto this node.
    ///
    /// # Panics
    ///
    /// With [`TT_FIELD_OUT_OF_RANGE`] if a field does not fit its slot — a depth
    /// past the ply horizon, or a score outside the band. Both are bugs in the
    /// search rather than anything a caller could hand in.
    pub fn packed(verification: u64, record: Record, node_score: i32, generation: u8) -> Entry {
        Entry {
            verification,
            best_q: record.best.q,
            best_r: record.best.r,
            score: fits(node_score, "score"),
            static_eval: fits(record.static_eval, "static_eval"),
            depth_plies: depth_fits(record.depth_plies),
            bound_age: (generation << 2) | record.bound.index(),
            flags: if record.from_quiescence {
                FLAG_FROM_QUIESCENCE
            } else {
                0
            },
        }
    }

    /// Unpack, leaving the score exactly as it was stored: re-basing it is the
    /// table's job, because only the table knows which node is asking.
    pub fn record(self) -> Record {
        Record {
            depth_plies: u32::from(self.depth_plies),
            score: i32::from(self.score),
            static_eval: i32::from(self.static_eval),
            bound: Bound::from_index(self.bound_age & 0b11),
            best: Coord::new(self.best_q, self.best_r),
            from_quiescence: self.from_quiescence(),
        }
    }

    /// Whether this slot has ever been written.
    pub fn is_empty(self) -> bool {
        self.depth_plies == 0
    }

    /// Whether this entry names the position `verification` identifies.
    pub fn matches(self, verification: u64) -> bool {
        !self.is_empty() && self.verification == verification
    }

    /// The generation that wrote it.
    pub fn age(self) -> u8 {
        self.bound_age >> 2
    }

    /// Whether a quiescence-regime node wrote this entry.
    pub fn from_quiescence(self) -> bool {
        self.flags & FLAG_FROM_QUIESCENCE != 0
    }

    /// How deep the search that wrote it looked, in plies. Zero means empty.
    pub fn depth_plies(self) -> u8 {
        self.depth_plies
    }
}

/// Narrow a score to the slot it packs into, loudly.
fn fits(value: i32, field: &str) -> i16 {
    i16::try_from(value).unwrap_or_else(|_| {
        panic!("pistol-search invariant {TT_FIELD_OUT_OF_RANGE}: {field} {value} does not fit")
    })
}

/// Narrow a depth to the slot it packs into, loudly. Zero is the empty marker
/// and so is not a storable depth.
fn depth_fits(depth_plies: u32) -> u8 {
    let depth = u8::try_from(depth_plies).unwrap_or_else(|_| {
        panic!(
            "pistol-search invariant {TT_FIELD_OUT_OF_RANGE}: depth {depth_plies} plies is past \
             the packed horizon"
        )
    });
    assert!(
        depth >= 1,
        "pistol-search invariant {TT_FIELD_OUT_OF_RANGE}: a leaf is not worth an entry"
    );
    depth
}
