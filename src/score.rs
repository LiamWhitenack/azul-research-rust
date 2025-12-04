/// Calculate the score for placing a piece at position (m, n)
pub fn score_placement(wall: &[[bool; 5]; 5], m: usize, n: usize) -> i8 {
    if wall[m][n] {
        panic!("A piece has already been placed in that position!");
    }

    let mut points = 1;
    let mut score_row = false;
    let mut score_column = false;

    // Check to the right
    for ni in (n + 1)..5 {
        if !wall[m][ni] {
            break;
        }
        score_row = true;
        points += 1;
    }

    // Check to the left
    for ni in 0..n {
        if !wall[m][n - 1 - ni] {
            break;
        }
        score_row = true;
        points += 1;
    }

    // Check downwards
    for mi in (m + 1)..5 {
        if !wall[mi][n] {
            break;
        }
        score_column = true;
        points += 1;
    }

    // Check upwards
    for mi in 0..m {
        if !wall[m - 1 - mi][n] {
            break;
        }
        score_column = true;
        points += 1;
    }

    if score_row && score_column {
        points += 1;
    }

    points
}

/// Calculate the endgame score
pub fn score_endgame(wall: &[[bool; 5]; 5]) -> i8 {
    let mut res = 0;

    // Rows
    for m in 0..5 {
        if (0..5).all(|n| wall[m][n]) {
            res += 2;
        }
    }

    // Columns
    for n in 0..5 {
        if (0..5).all(|m| wall[m][n]) {
            res += 7;
        }
    }

    // Diagonals
    for n in 0..5 {
        if (0..5).all(|i| wall[i][(n + i) % 5]) {
            res += 10;
        }
    }

    res
}
