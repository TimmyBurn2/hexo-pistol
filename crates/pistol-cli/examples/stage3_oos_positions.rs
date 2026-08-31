//! Bench-band positions drawn in ROUNDS, so a later round is out-of-sample by
//! construction.
//!
//! `corpus-extract` takes twelve games per band in `game_hash_asc` order and
//! stops. Every number that informed the Stage-3 detector's `opp_hot` threshold
//! was measured on that draw, so validating the threshold needs positions no
//! part of it saw. This continues the SAME loop with the SAME used-set: round 0
//! is byte-identical to `bench_positions_v1.txt`'s body — which is what makes it
//! an instrument rather than a second selection rule — and round 1 is the next
//! twelve per band, disjoint by game hash from round 0 by construction.
//!
//! Usage:
//!   stage3_oos_positions --corpus <path> --round <n>
//! Exit:
//!   0 emitted
//!   1 an argument or a corpus this build refuses
//!   2 THE RUN IS VOID — the corpus is unreadable

use std::process::ExitCode;

use pistol_cli::corpus::openings::Candidate;
use pistol_cli::corpus::verdict::Replayed;
use pistol_cli::corpus::{bench, read, replay};
use pistol_cli::sha256::sha256_hex;
use pistol_engine::PositionSpec;

fn void(why: &str) -> ExitCode {
    eprintln!("stage3_oos_positions: RUN VOID: {why}");
    ExitCode::from(2)
}

fn fail(why: &str) -> ExitCode {
    eprintln!("stage3_oos_positions: FAIL: {why}");
    ExitCode::from(1)
}

struct Drawn {
    centre: usize,
    stones: usize,
    position: PositionSpec,
    game_hash: String,
}

/// Take `rounds + 1` rounds of [`bench::PER_BAND`] positions per band, carrying
/// one used-set across every band of every round, and return the last round.
///
/// The carried used-set is the whole point: it is what makes round `r` disjoint
/// from every earlier round in the same sense `corpus-extract`'s own bands are
/// disjoint from each other — by game, not merely by position.
fn draw(candidates: &[Candidate<'_>], rounds: usize) -> Vec<Drawn> {
    let mut ordered: Vec<&Candidate<'_>> = candidates.iter().collect();
    ordered.sort_by(|a, b| a.record.game_hash.cmp(&b.record.game_hash));

    let mut used: Vec<&str> = Vec::new();
    let mut last = Vec::new();
    for round in 0..=rounds {
        last.clear();
        for band in bench::BANDS {
            let mut taken = 0usize;
            for candidate in &ordered {
                if taken == bench::PER_BAND {
                    break;
                }
                if used.contains(&candidate.record.game_hash.as_str()) {
                    continue;
                }
                let Some(stones) = band.count_for(candidate.record.moves.len()) else {
                    continue;
                };
                let turns_wanted = bench::Band::turn_for(stones);
                if candidate.turns.len() <= turns_wanted {
                    continue;
                }
                if round == rounds {
                    last.push(Drawn {
                        centre: band.centre,
                        stones,
                        position: PositionSpec::Start {
                            moves: candidate
                                .turns
                                .iter()
                                .take(turns_wanted)
                                .map(|grouped| grouped.turn)
                                .collect(),
                        },
                        game_hash: candidate.record.game_hash.clone(),
                    });
                }
                used.push(&candidate.record.game_hash);
                taken += 1;
            }
        }
    }
    last
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    let mut corpus = None;
    let mut round = None;
    let mut index = 0;
    while index < words.len() {
        let key = words[index].as_str();
        let Some(value) = words.get(index + 1) else {
            return fail(&format!("{key} wants a value"));
        };
        match key {
            "--corpus" => corpus = Some(value.clone()),
            "--round" => match value.parse::<usize>() {
                Ok(parsed) => round = Some(parsed),
                Err(why) => return fail(&format!("--round: {why}")),
            },
            other => return fail(&format!("unknown option {other}")),
        }
        index += 2;
    }
    let Some(corpus) = corpus else {
        return fail("--corpus is required");
    };
    let Some(round) = round else {
        return fail("--round is required: a draw without its round is not a draw");
    };

    let bytes = match std::fs::read(&corpus) {
        Ok(bytes) => bytes,
        Err(why) => return void(&format!("cannot read {corpus}: {why}")),
    };
    let digest = sha256_hex(&bytes);
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(why) => return void(&format!("{corpus} is not utf-8: {why}")),
    };
    let records = match read(std::path::Path::new(&corpus), &text) {
        Ok(records) => records,
        Err(why) => return fail(&why.to_string()),
    };
    let replays: Vec<Replayed> = records.iter().map(replay::replay).collect();
    let mut candidates: Vec<Candidate<'_>> = Vec::new();
    let mut excluded = 0usize;
    for (record, replayed) in records.iter().zip(&replays) {
        if replayed.verdict.is_eligible() {
            candidates.push(Candidate {
                record,
                turns: &replayed.turns,
            });
        } else {
            excluded += 1;
        }
    }

    println!("# stage3 out-of-sample bench-band positions — round {round}");
    println!("#");
    println!("# Round 0 reproduces bench_positions_v1.txt's body exactly; every later");
    println!(
        "# round is the next {} games per band in the same `game_hash_asc` order,",
        bench::PER_BAND
    );
    println!("# carrying one used-set, so the rounds are disjoint BY GAME.");
    println!("#");
    println!("# param round {round}");
    println!("# param per_band {}", bench::PER_BAND);
    println!("# param selection_order game_hash_asc");
    for band in bench::BANDS {
        println!("# param band centre {} width {}", band.centre, band.width);
    }
    println!("# derived corpus_sha256 {digest}");
    println!("# derived games_read {}", records.len());
    println!("# derived games_excluded {excluded}");
    let drawn = draw(&candidates, round);
    println!("# derived positions {}", drawn.len());
    let mut current = None;
    for position in &drawn {
        if current != Some(position.centre) {
            println!("# band centre {}", position.centre);
            current = Some(position.centre);
        }
        println!(
            "{} # src {} stones {}",
            position.position, position.game_hash, position.stones
        );
    }
    if drawn.is_empty() {
        return fail(&format!(
            "round {round} drew nothing: the corpus is exhausted"
        ));
    }
    // On stdout and as a COMMENT: the file this prints IS the fixture a census
    // reads, so a receipt living on stderr is one a redirect separates from the
    // bytes it describes.
    println!(
        "# stage3_oos_positions: round {round} drew {} positions",
        drawn.len()
    );
    ExitCode::SUCCESS
}
