use std::{collections::VecDeque, time::Instant};
use rayon::prelude::*;

use crate::{
    arrange_tiles::arrange,
    resolve_placement::place,
    score::score_endgame,
    types::{GameProgression, PatternLines},
};

// Timing helper similar to Python @timed decorator
fn timed<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    println!("Time elapsed: {:.2?}", start.elapsed());
    result
}


pub fn get_all_scores(n_rounds: usize) -> Vec<GameProgression> {
    timed(|| {
        let mut res: Vec<GameProgression> = Vec::new();

        // Start with a single empty PatternLines
        let initial_patterns: Vec<PatternLines> = vec![PatternLines::new()];

        // Unfinished now stores GameProgression structs
        let mut unfinished: VecDeque<GameProgression> = VecDeque::new();
        unfinished.push_back(GameProgression::new(0, initial_patterns, [[false; 5]; 5]));

        // Process BFS level by level
        while !unfinished.is_empty() {
            // Take the current level frontier
            let current_level: Vec<GameProgression> = unfinished.drain(..).collect();

            // Parallel expansion of the current level
            let mut next_level: Vec<GameProgression> = current_level
                .into_par_iter()
                .flat_map(|game| {
                    // If we have reached the final round
                    if game.patterns.len() == n_rounds + 1 {
                        let mut finished_game = game.clone();
                        finished_game.score += score_endgame(&finished_game.wall);
                        return vec![finished_game].into_par_iter();
                    }

                    let last_pattern = game.patterns.last().unwrap();
                    let mut new_arrangements: Vec<PatternLines> = Vec::new();
                    arrange(&mut new_arrangements, last_pattern, None, None);

                    // Create new GameProgressions for each arrangement
                    new_arrangements
                        .into_iter()
                        .map(move |pattern_lines| {
                            let mut new_game = game.clone();
                            new_game.patterns.push(pattern_lines);
                            place(&mut new_game.score, &mut new_game.patterns, &mut new_game.wall);
                            new_game
                        })
                        .collect::<Vec<_>>()
                        .into_par_iter()
                })
                .collect();

            // Separate finished games from unfinished games
            next_level.retain(|game| {
                if game.patterns.len() == n_rounds + 1 {
                    res.push(game.clone());
                    false // remove from next_level
                } else {
                    true // keep for next iteration
                }
            });

            // Push remaining unfinished games into the queue
            unfinished.extend(next_level);
        }

        res
    })
}



