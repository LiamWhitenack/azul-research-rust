use crate::types::PatternLines;
use crate::score::score_placement;

// The subroutine modifies score, path, and wall in place
pub fn place(score: &mut i8, path: &mut Vec<PatternLines>, wall: &mut [[bool; 5]; 5]) {
    if let Some(last_step) = path.last_mut() {
        for (m, line) in last_step.iter_mut().enumerate() {
            if (line.count as usize) > m {
                // Update score based on a copy of the wall
                *score += score_placement(&wall.clone(), m, line.color as usize);
                // Update the wall
                wall[m][line.color as usize] = true;
                // Update the line
                if line.color >= 0 && line.color < 5 {
                    line.potential_colors[line.color as usize] = false;
                }
                line.color = -1;
                line.count = 0;
            }
        }
    }
}
