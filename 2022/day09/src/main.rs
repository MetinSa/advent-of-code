use std::collections::HashMap;
use std::fs;

const N_ROWS: usize = 1000;
const N_COLS: usize = 1000;
const N_KNOTS: usize = 10;
fn main() {
    let input_vec: Vec<String> = fs::read_to_string("input.txt")
        .expect("Error reading input.txt")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut knot_grids: Vec<[[bool; N_COLS]; N_ROWS]> = Vec::new();
    let mut tail_visited_grid = [[false; N_COLS]; N_ROWS];
    tail_visited_grid[N_ROWS / 2][N_COLS / 2] = true;

    let mut knot_indicies: Vec<(isize, isize)> = Vec::new();
    for i in 0..N_KNOTS {
        knot_grids.push([[false; N_COLS]; N_ROWS]);
        knot_indicies.push((N_ROWS as isize / 2, N_COLS as isize / 2));
        knot_grids[i][knot_indicies[i].0 as usize][knot_indicies[i].1 as usize] = true;
    }

    let initial_direction_mapping: HashMap<&str, (isize, isize)> =
        HashMap::from([("U", (-1, 0)), ("D", (1, 0)), ("L", (0, -1)), ("R", (0, 1))]);

    for line in input_vec.iter() {
        let (direction, distance) = line.split_once(" ").unwrap();

        let (head_row_offset, head_col_offset) = initial_direction_mapping.get(direction).unwrap();

        for _ in 0..distance.parse().unwrap() {
            knot_grids[0][knot_indicies[0].0 as usize][knot_indicies[0].1 as usize] = false;

            knot_indicies[0].0 += head_row_offset;
            knot_indicies[0].1 += head_col_offset;

            knot_grids[0][knot_indicies[0].0 as usize][knot_indicies[0].1 as usize] = true;

            for i in 1..N_KNOTS {
                let prev_knot_indices = knot_indicies[i - 1];
                if (prev_knot_indices.0 - knot_indicies[i].0).abs() >= 2
                    || (prev_knot_indices.1 - knot_indicies[i].1).abs() >= 2
                {
                    knot_grids[i][knot_indicies[i].0 as usize][knot_indicies[i].1 as usize] = false;

                    knot_indicies[i].0 += (prev_knot_indices.0 - knot_indicies[i].0).signum();
                    knot_indicies[i].1 += (prev_knot_indices.1 - knot_indicies[i].1).signum();

                    knot_grids[i][knot_indicies[i].0 as usize][knot_indicies[i].1 as usize] = true;
                }
            }
            tail_visited_grid[knot_indicies.last().unwrap().0 as usize]
                [knot_indicies.last().unwrap().1 as usize] = true;
        }
    }

    let total_visited: u32 = tail_visited_grid.iter().flatten().map(|x| *x as u32).sum();
    println!("Total visited: {}", total_visited)
}
