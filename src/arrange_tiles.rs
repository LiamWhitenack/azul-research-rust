use crate::types::PatternLines;

pub fn arrange(
    arrangements: &mut Vec<PatternLines>,
    input: &PatternLines,
    colors: Option<[bool; 5]>,
    m: Option<usize>,
) {
    // Use default values if None
    let colors = colors.unwrap_or([true; 5]);
    let m = m.unwrap_or(0);

    let line = input.line(m).clone();

    let mut compatible_colors = [false; 5];
    for i in 0..5 {
        compatible_colors[i] = line.potential_colors[i] & colors[i];
    }

    if line.color != -1 {
        let mut forced_mask = [false; 5];
        let chosen_color = line.color as usize;
        forced_mask[chosen_color] = compatible_colors[chosen_color];
        compatible_colors = forced_mask;
    }

    if !compatible_colors.iter().any(|&b| b) {
        if m == 4 {
            arrangements.push(input.clone());
        } else {
            arrange(arrangements, input, Some(colors), Some(m + 1));
        }
        return;
    }

    let mut mask = compatible_colors;
    while let Some(pos) = mask.iter().position(|&b| b) {
        let mut pattern_copy = input.clone();
        let mut line_copy = line.clone();

        mask[pos] = false; // remove choice

        line_copy.color = pos as i8;
        line_copy.count += 2;
        pattern_copy.set_line(m, line_copy);

        if m == 4 {
            arrangements.push(pattern_copy);
        } else {
            let mut colors_copy = colors;
            colors_copy[pos] = false;

            // roll colors_copy by 1 to the right
            let last = colors_copy[4];
            for i in (1..5).rev() {
                colors_copy[i] = colors_copy[i - 1];
            }
            colors_copy[0] = last;

            arrange(arrangements, &pattern_copy, Some(colors_copy), Some(m + 1));
        }
    }
}
