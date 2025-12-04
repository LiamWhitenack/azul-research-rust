mod score;
mod types;
mod view_progression;
mod resolve_placement;
mod arrange_tiles;
mod dfs;

use types::*;
use view_progression::*;

use dfs::get_all_scores;



fn main() {
    let n_rounds: i8 = 5;
    let progressions: Vec<GameProgression> = get_all_scores(n_rounds);

    // if let Some(best) = progressions.iter().max_by_key(|gp| gp.score) {
    //     println!("{}", progressions.len());
    //     println!("{}", styled_grid_progression(best, false));
    // } else {
    //     println!("No scores available");
    // }
}

