//! The two documents `corpus-extract` writes.
//!
//! [`super::emit`] owns the header and digest *discipline* — what a fixture
//! header is made of and where its payload begins. This file owns what these two
//! particular fixtures say: the prose a reader meets, which parameters each
//! states, and the shape of a payload line. It is a separate file for size
//! discipline (CLAUDE.md rule 9), and the split is a real one — the discipline is
//! reusable and the prose is not.
//!
//! Both documents carry the same provenance block, because both are claims about
//! the same corpus and a reader who has one should not need the other to know
//! which document it came from.

use super::bench;
use super::emit::Fixture;
use super::openings::{self, ELO_GAP_CEILING, K_TURNS, OPENING_STONES};
use super::stats::Stats;

/// The header every output shares: what the corpus was, and what was left out.
pub fn provenance(fixture: &mut Fixture, digest: &str, stats: &Stats, excluded: &[String]) {
    fixture.gap();
    fixture.note("The corpus is an external artifact and is NEVER committed (CLAUDE.md rule 8).");
    fixture.note("It is identified here by its digest and by nothing else — not by a path, which");
    fixture.note("is machine-specific, and not by a timestamp, so the same bytes give the same");
    fixture.note("file anywhere (CLAUDE.md rule 4).");
    fixture.derived("corpus_sha256", digest);
    fixture.derived("games_read", stats.games_read);
    fixture.derived("games_excluded", stats.excluded());
    fixture.gap();
    fixture.note("Every game excluded from every output, by hash, with the flat move index it");
    fixture.note("failed at:");
    fixture.exclusions(excluded.to_vec());
}

/// `openings_v1.txt`.
pub fn openings_fixture(
    digest: &str,
    stats: &Stats,
    selection: &openings::Selection,
    excluded: &[String],
) -> String {
    let mut fixture = Fixture::new(&[
        "pistol opening corpus, v1 — the positions the arena plays from.",
        "",
        "Pinned by SHA-256 in crates/pistol-cli/tests/corpus_document_tests.rs",
        "(OPENINGS_V1_SHA256). Editing this file without updating that constant is a red",
        "test, which is the point.",
        "",
        "One position per line, in the canonical move-list encoding (docs/decisions.md",
        "D-6) — the exact tail the `position` verb takes. Everything from ` #` onward is",
        "commentary a reader may strip:",
        "",
        "  src    the corpus game this spelling came from",
        "  elo    the lower of that game's two ratings",
        "  games  how many corpus games reach this opening up to a lattice symmetry",
        "  p1     how many of those the FIRST player won",
        "",
        "`games` and `p1` are the corpus's own evidence about how balanced an opening is.",
        "They are emitted rather than acted on: a threshold is the runner's to choose, and",
        "filtering by an engine's own eval would put the engine under test into its own",
        "book (docs/decisions.md D-145). Coverage is partial and that is not hidden — most",
        "openings occur once, so most lines read `games 1`.",
        "",
        "There is NO CAP. The whole pool is here, in game-hash order, so a runner takes a",
        "prefix and gets a sample rather than the rating tail (docs/decisions.md D-143).",
    ]);
    fixture.gap();
    fixture.note("How a game had to qualify, and how a position was chosen:");
    fixture.param("k_turns", K_TURNS);
    fixture.param("opening_stones", OPENING_STONES);
    fixture.param("elo_gap_ceiling", ELO_GAP_CEILING);
    fixture.param("elo_floor_rule", "lower_median_of_min_elo_over_candidates");
    fixture.param("dedupe", "canonical_form_over_the_12_lattice_symmetries");
    fixture.param("representative", "min_elo_desc,game_hash_asc");
    fixture.param("emission_order", "game_hash_asc");
    fixture.derived("elo_floor", selection.floor);
    fixture.derived("candidate_games", selection.candidates);
    fixture.derived("eligible_games", selection.eligible);
    fixture.derived("distinct_positions", selection.distinct_positions);
    fixture.derived("canonical_openings", selection.openings.len());
    provenance(&mut fixture, digest, stats, excluded);
    for opening in &selection.openings {
        fixture.line(&format!(
            "{} # src {} elo {} games {} p1 {}",
            opening.position,
            opening.game_hash,
            opening.min_elo,
            opening.class_games,
            opening.class_p1_wins
        ));
    }
    fixture.render()
}

/// `bench_positions_v1.txt`.
pub fn bench_fixture(
    digest: &str,
    stats: &Stats,
    positions: &[bench::BenchPosition],
    excluded: &[String],
) -> String {
    let mut fixture = Fixture::new(&[
        "pistol bench positions, v1 — two stone counts for a like-for-like bench.",
        "",
        "Pinned by SHA-256 in crates/pistol-cli/tests/corpus_document_tests.rs",
        "(BENCH_POSITIONS_V1_SHA256).",
        "",
        "Same line form as openings_v1.txt: a `position` verb tail, then commentary from",
        "` #` onward. `stones` is stated per entry because a band does not always reach",
        "its centre — a game that ends early contributes the largest turn boundary it has.",
        "",
        "A position sits at a TURN boundary, so its stone count is odd: turn t carries",
        "2t-1 stones. The rule is the largest odd count at or below the band centre and no",
        "lower than the centre less the width (docs/decisions.md D-146).",
        "",
        "No game appears in both bands: twenty-four games measure twenty-four position",
        "shapes, where twelve measured twice would measure twelve.",
        "",
        "Consumer: ROADMAP WP-1.3(c)'s fixed-node runs at two stone counts. `bench` itself",
        "stays unimplemented until a change justifies one (CLAUDE.md rule 5, D-14).",
    ]);
    fixture.gap();
    for band in bench::BANDS {
        fixture.param(
            "band",
            format!("centre {} width {}", band.centre, band.width),
        );
    }
    fixture.param("per_band", bench::PER_BAND);
    fixture.param("selection_order", "game_hash_asc");
    fixture.derived("positions", positions.len());
    provenance(&mut fixture, digest, stats, excluded);
    let mut current = None;
    for position in positions {
        if current != Some(position.centre) {
            fixture.body_note(&format!("band centre {}", position.centre));
            current = Some(position.centre);
        }
        fixture.line(&format!(
            "{} # src {} stones {}",
            position.position, position.game_hash, position.stones
        ));
    }
    fixture.render()
}
