use std::{collections::VecDeque, time::Instant};
use rayon::prelude::*;
use mpi::traits::*;

use crate::{
    arrange_tiles::arrange,
    resolve_placement::place,
    score::score_endgame,
    types::{GameProgression, PatternLines},
};

fn timed<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    println!("Time elapsed: {:.2?}", start.elapsed());
    result
}

/// Hierarchical parallelism: MPI for first level, Rayon inside each MPI process
pub fn get_all_scores_mpi(n_rounds: usize) -> Vec<GameProgression> {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    timed(|| {
        let mut res: Vec<GameProgression> = Vec::new();

        // Start with a single empty PatternLines
        let initial_patterns: Vec<PatternLines> = vec![PatternLines::new()];

        // Generate first level BFS nodes
        let mut first_level_nodes: Vec<GameProgression> = initial_patterns
            .into_iter()
            .map(|pattern| GameProgression::new(0, vec![pattern], [[false; 5]; 5]))
            .collect();

        // Split first level across MPI ranks
        let local_first_level: Vec<GameProgression> = first_level_nodes
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % size as usize == rank as usize)
            .map(|(_, node)| node)
            .collect();

        // BFS with Rayon for subsequent levels
        let mut unfinished: VecDeque<GameProgression> = VecDeque::from(local_first_level);

        while !unfinished.is_empty() {
            let current_level: Vec<GameProgression> = unfinished.drain(..).collect();

            // Parallel expansion using Rayon
            let mut next_level: Vec<GameProgression> = current_level
                .into_par_iter()
                .flat_map(|game| {
                    // If we reached final round
                    if game.patterns.len() == n_rounds + 1 {
                        let mut finished_game = game.clone();
                        finished_game.score += score_endgame(&finished_game.wall);
                        return vec![finished_game].into_par_iter();
                    }

                    let last_pattern = game.patterns.last().unwrap();
                    let mut new_arrangements: Vec<PatternLines> = Vec::new();
                    arrange(&mut new_arrangements, last_pattern, None, None);

                    new_arrangements
                        .into_iter()
                        .map(move |pattern_lines| {
                            let mut new_game = game.clone();
                            new_game.patterns.push(pattern_lines);
                            place(
                                &mut new_game.score,
                                &mut new_game.patterns,
                                &mut new_game.wall,
                            );
                            new_game
                        })
                        .collect::<Vec<_>>()
                        .into_par_iter()
                })
                .collect();

            // Separate finished games from unfinished
            next_level.retain(|game| {
                if game.patterns.len() == n_rounds + 1 {
                    res.push(game.clone());
                    false
                } else {
                    true
                }
            });

            unfinished.extend(next_level);
        }

        // Gather results at rank 0
        let all_results: Vec<Vec<GameProgression>> = world
            .all_gather_into_vec(&res);

        if rank == 0 {
            // Flatten all results into a single Vec
            all_results.into_iter().flatten().collect()
        } else {
            Vec::new()
        }
    })
}
