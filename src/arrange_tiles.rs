use crate::types::PatternLines;

pub fn arrange(
    arrangements: &mut Vec<PatternLines>,
    input: &PatternLines,
    colors: Option<[bool; 5]>,
    m: Option<usize>,
) {
    let colors = colors.unwrap_or([true; 5]);
    let m = m.unwrap_or(0);

    // Stack holds tuples of (current_pattern, line_index, available_colors)
    let mut stack: Vec<(PatternLines, usize, [bool; 5])> = Vec::new();
    stack.push((input.clone(), m, colors));

    while let Some((pattern, m, colors)) = stack.pop() {
        let line = pattern.line(m).clone();

        // Determine compatible colors
        let mut compatible_colors = [false; 5];
        for i in 0..5 {
            compatible_colors[i] = line.potential_colors[i] & colors[i];
        }

        // If line.color is forced, mask all other colors
        if line.color != -1 {
            let mut forced_mask = [false; 5];
            let chosen_color = line.color as usize;
            forced_mask[chosen_color] = compatible_colors[chosen_color];
            compatible_colors = forced_mask;
        }

        // If no compatible colors, either push the arrangement or move to next line
        if !compatible_colors.iter().any(|&b| b) {
            if m == 4 {
                arrangements.push(pattern);
            } else {
                stack.push((pattern, m + 1, colors));
            }
            continue;
        }

        // For each compatible color, create a new state and push to stack
        let mut mask = compatible_colors;
        while let Some(pos) = mask.iter().position(|&b| b) {
            mask[pos] = false;

            let mut pattern_copy = pattern.clone();
            let mut line_copy = line.clone();

            line_copy.color = pos as i8;
            line_copy.count += 2;
            pattern_copy.set_line(m, line_copy);

            if m == 4 {
                arrangements.push(pattern_copy);
            } else {
                let mut colors_copy = colors;
                colors_copy[pos] = false;

                // Roll colors_copy by 1 to the right
                let last = colors_copy[4];
                for i in (1..5).rev() {
                    colors_copy[i] = colors_copy[i - 1];
                }
                colors_copy[0] = last;

                stack.push((pattern_copy, m + 1, colors_copy));
            }
        }
    }
}
