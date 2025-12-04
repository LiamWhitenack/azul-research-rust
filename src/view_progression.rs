use colored::*;

use crate::{score::score_placement, types::GameProgression};

const GRID_WIDTH: usize = 13;
const GRID_SPACING: usize = 1;
const COLORS: [&str; 6] = ["red", "green", "yellow", "blue", "magenta", "pink"];
const MARKDOWN_COLOR_CLASSES: [&str; 6] = ["red", "green", "blue", "orange", "purple", "pink"];

fn colored_text(text: &str, color_idx: usize, bold: bool) -> String {
    let color_name = COLORS[color_idx % COLORS.len()];
    if bold {
        text.color(color_name).bold().to_string()
    } else {
        text.color(color_name).to_string()
    }
}

fn md_color(text: &str, color_idx: usize, bold: bool) -> String {
    let class_name = MARKDOWN_COLOR_CLASSES[color_idx % MARKDOWN_COLOR_CLASSES.len()];
    if bold {
        format!(
            r#"<span class="{}" style="font-weight:bold;">{}</span>"#,
            class_name, text
        )
    } else {
        format!(r#"<span class="{}">{}</span>"#, class_name, text)
    }
}

fn empty_grid() -> Vec<Vec<String>> {
    vec![vec![" ".to_string(); 6]; 6]
}

fn mark_placements_stale(grid: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.replace("X", "O"))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn make_grids<F>(
    progression: &GameProgression,
    style_function: F,
) -> (Vec<Vec<Vec<String>>>, Vec<String>)
where
    F: Fn(&str, usize, bool) -> String,
{
    let mut res: Vec<Vec<Vec<String>>> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    let mut grid = empty_grid();
    let mut wall = [[false; 6]; 6];
    let mut score = 0;

    let steps = &progression.patterns;
    for window in steps.windows(2) {
        let previous_step = &window[0];
        let step = &window[1];
        grid = mark_placements_stale(&grid);

        for (m, (previous_line, pattern_line)) in previous_step.iter().zip(step.iter()).enumerate()
        {
            let (color, count, place_tile) = if pattern_line.color >= 0 {
                (pattern_line.color as usize, pattern_line.count, false)
            } else {
                let mut found_color = -1;
                for (i, (p, c)) in previous_line
                    .potential_colors
                    .iter()
                    .zip(pattern_line.potential_colors.iter())
                    .enumerate()
                {
                    if p != c {
                        found_color = i as i8;
                        break;
                    }
                }
                if found_color != -1 {
                    (found_color as usize, (m + 1) as i8, true)
                } else {
                    (0, 0, false)
                }
            };

            let left = style_function(
                &format!("{}/{}", count, m + 1),
                (color + 6 - m % 6) % 6,
                place_tile,
            );

            let mut row_cells = vec![left];
            for n in 0..5 {
                let cell = if n == color && place_tile {
                    score += score_placement(&wall, m, n);
                    wall[m][n] = true;
                    style_function("X", (color + 6 - m % 6) % 6, true)
                } else {
                    grid[m][n + 1].clone()
                };
                row_cells.push(cell);
            }
            grid[m] = row_cells;
        }

        res.push(grid.clone());
        labels.push(score.to_string());
    }

    (res, labels)
}

pub fn styled_grid_progression(progression: &GameProgression, markdown: bool) -> String {
    let style_function = if markdown { md_color } else { colored_text };

    let (grids, labels) = make_grids(progression, style_function);

    if grids.is_empty() {
        return String::new();
    }

    let num_rows = grids[0].len();
    let mut out = Vec::new();

    if !markdown {
        let label_line: Vec<String> = labels.iter().map(|l| format!("{:^13}", l)).collect();
        out.push(label_line.join(&" ".repeat(GRID_SPACING)));
        out.push(vec!["-".repeat(GRID_WIDTH); grids.len()].join(&" ".repeat(GRID_SPACING)));

        for row_idx in 0..num_rows {
            let parts: Vec<String> = grids
                .iter()
                .map(|grid| grid[row_idx].join(" ").to_string())
                .collect();
            out.push(parts.join(&" ".repeat(GRID_SPACING)));
        }

        return out.join("\n");
    }

    // Markdown output
    out.push("---\nmarp: true\ntheme: custom\n---\n".to_string());
    out.push("<style>\n.red { color: red; }\n.green { color: green; }\n.blue { color: blue; }\n.orange { color: orange; }\n.purple { color: purple; }\n.black { color: black; }\ntd { width: 80px; text-align: center; }\n</style>\n".to_string());

    for (grid_idx, grid) in grids.iter().enumerate() {
        let label = &labels[grid_idx];
        out.push(format!(
            "### Step {} — Score: **{}**\n",
            grid_idx + 1,
            label
        ));
        out.push("|  |  |  |  |  |  |".to_string());
        out.push("|------|------|------|------|------|------|".to_string());

        for r in 0..num_rows {
            let row_cells: Vec<String> = grid[r]
                .iter()
                .map(|c| {
                    if c.trim().is_empty() {
                        "&nbsp;".to_string()
                    } else {
                        c.replace("\n", "<br>")
                    }
                })
                .collect();
            out.push(format!("| {} |", row_cells.join(" | ")));
        }

        out.push(String::new());
        out.push("---\n".to_string());
    }

    out.join("\n")
}
