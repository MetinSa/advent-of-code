use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input file");

    let tree_grid = get_tree_grid(&input);

    let visibility_grid = get_visibility_grid(&tree_grid);
    let visability_score: u32 = visibility_grid
        .iter()
        .flatten()
        .map(|tree| *tree as u32)
        .sum();
    println!("Part 1: there are {} total visible trees", visability_score);

    let scenic_grid = get_scenic_grid(&tree_grid);
    let max_score = scenic_grid.iter().flatten().max().unwrap();
    println!("Part 2: the max score for any tree is {}", max_score);
}

fn get_tree_grid(input_string: &str) -> Vec<Vec<u8>> {
    input_string
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_visibility_grid(tree_grid: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    const MAX_HEIGHT: u8 = 9;
    let n_cols = tree_grid[0].len();
    let n_rows = tree_grid.len();

    let mut visibility_grid = vec![vec![false; n_cols]; n_rows];
    for i in 0..n_rows {
        for j in 0..n_cols {
            if i == 0 || j == 0 || i == n_rows - 1 || j == n_cols - 1 {
                visibility_grid[i][j] = true;
            }
        }
    }

    // From left to right
    let mut tallest: u8;
    let mut height: u8;
    for i in 1..n_rows - 1 {
        tallest = tree_grid[i][0];
        for j in 1..n_cols - 1 {
            height = tree_grid[i][j];
            if height > tallest {
                tallest = height;
                visibility_grid[i][j] = true;
            }
            if height == MAX_HEIGHT {
                break;
            }
        }
    }

    // From right to left
    let mut tallest: u8;
    let mut height: u8;
    for i in (1..n_rows - 1).rev() {
        tallest = tree_grid[i][n_cols - 1];
        for j in (1..n_cols - 1).rev() {
            height = tree_grid[i][j];
            if height > tallest {
                tallest = height;
                visibility_grid[i][j] = true;
            }
            if height == MAX_HEIGHT {
                break;
            }
        }
    }

    // From top to bottom
    let mut tallest: u8;
    let mut height: u8;
    for j in 1..n_cols - 1 {
        tallest = tree_grid[0][j];
        for i in 1..n_rows - 1 {
            height = tree_grid[i][j];
            if height > tallest {
                tallest = height;
                visibility_grid[i][j] = true;
            }
            if height == MAX_HEIGHT {
                break;
            }
        }
    }

    // From top to bottom
    let mut tallest: u8;
    let mut height: u8;
    for j in (1..n_cols - 1).rev() {
        tallest = tree_grid[n_cols - 1][j];
        for i in (1..n_rows - 1).rev() {
            height = tree_grid[i][j];
            if height > tallest {
                tallest = height;
                visibility_grid[i][j] = true;
            }
            if height == MAX_HEIGHT {
                break;
            }
        }
    }
    visibility_grid
}

fn get_scenic_score(
    row: usize,
    col: usize,
    n_rows: usize,
    n_cols: usize,
    tree_grid: &Vec<Vec<u8>>,
) -> u32 {
    let tree_height = tree_grid[row][col];

    // left to right
    let mut right_score = 0;
    for i in col..n_cols - 1 {
        if tree_height > tree_grid[row][i + 1] {
            right_score += 1;
        } else if tree_height <= tree_grid[row][i + 1] {
            right_score += 1;
            break;
        }
    }

    // right to left
    let mut left_score = 0;
    for i in (0..col).rev() {
        if tree_height > tree_grid[row][i] {
            left_score += 1;
        } else if tree_height <= tree_grid[row][i] {
            left_score += 1;
            break;
        }
    }

    // top to bottom
    let mut bottom_score = 0;
    for i in row..n_rows - 1 {
        if tree_height > tree_grid[i + 1][col] {
            bottom_score += 1;
        } else if tree_height <= tree_grid[i + 1][col] {
            bottom_score += 1;
            break;
        }
    }

    // bottom to top
    let mut top_score = 0;
    for i in (0..row).rev() {
        if tree_height > tree_grid[i][col] {
            top_score += 1;
        } else if tree_height <= tree_grid[i][col] {
            top_score += 1;
            break;
        }
    }
    left_score * right_score * top_score * bottom_score
}

fn get_scenic_grid(tree_grid: &Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    let n_cols = tree_grid[0].len();
    let n_rows = tree_grid.len();
    let mut scenic_score_grid = vec![vec![0; n_cols]; n_rows];

    for i in 1..n_rows - 1 {
        for j in 1..n_cols - 1 {
            scenic_score_grid[i][j] = get_scenic_score(i, j, n_rows, n_cols, &tree_grid);
        }
    }
    scenic_score_grid
}
