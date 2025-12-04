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
    let progressions: Vec<GameProgression> = get_all_scores(5);

    if let Some(best) = progressions.iter().max_by_key(|gp| gp.score) {
        println!("{}", progressions.len());
        println!("{}", styled_grid_progression(best, true));
    } else {
        println!("No scores available");
    }
}

