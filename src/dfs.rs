use std::{collections::VecDeque, time::Instant};

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

        // Unfinished now stores GameProgression structs directly
        let mut unfinished: VecDeque<GameProgression> = VecDeque::new();
        unfinished.push_back(GameProgression::new(0, initial_patterns, [[false; 5]; 5]));

        while let Some(game) = unfinished.pop_front() {
            // If we have reached the final round
            if game.patterns.len() == n_rounds + 1 {
                let mut finished_game = game.clone();
                finished_game.score += score_endgame(&finished_game.wall);
                res.push(finished_game);
                continue;
            }

            let last_pattern = game.patterns.last().unwrap();

            // Generate new arrangements from the last pattern
            let mut new_arrangements: Vec<PatternLines> = Vec::new();
            arrange(&mut new_arrangements, last_pattern, None, None);

            // For each new arrangement, create a new GameProgression and place it
            for pattern_lines in new_arrangements {
                let mut new_game = game.clone();
                new_game.patterns.push(pattern_lines);
                place(&mut new_game.score, &mut new_game.patterns, &mut new_game.wall);
                unfinished.push_back(new_game);
            }
        }

        res
    })
}


