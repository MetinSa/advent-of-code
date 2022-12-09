use std::collections::HashMap;
use std::fs;

const N_ROWS: usize = 1000;
const N_COLS: usize = 1000;

fn main() {
    let input_vec: Vec<String> = fs::read_to_string("input.txt")
        .expect("Error reading input.txt")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    // Part 1
    let visited_grid = get_tail_visited_grid(&input_vec, 2);
    let total_visited: u32 = visited_grid.iter().flatten().map(|x| *x as u32).sum();
    println!(
        "Part 1: the tail visited {} positions in total",
        total_visited
    );

    // Part 2
    let visited_grid = get_tail_visited_grid(&input_vec, 10);
    let total_visited: u32 = visited_grid.iter().flatten().map(|x| *x as u32).sum();
    println!(
        "Part 2: the tail visited {} positions in total",
        total_visited
    );
}

fn get_tail_visited_grid(commands: &Vec<String>, n_knots: usize) -> Vec<Vec<bool>> {
    let mut knot_indicies: Vec<(isize, isize)> = Vec::new();
    for _ in 0..n_knots {
        knot_indicies.push((N_ROWS as isize / 2, N_COLS as isize / 2));
    }

    let mut tail_visited_grid = vec![vec![false; N_COLS]; N_ROWS];
    tail_visited_grid[N_ROWS / 2][N_COLS / 2] = true;

    let initial_direction_mapping: HashMap<&str, (isize, isize)> =
        HashMap::from([("U", (-1, 0)), ("D", (1, 0)), ("L", (0, -1)), ("R", (0, 1))]);

    for command in commands.iter() {
        let (direction, distance) = command.split_once(" ").unwrap();
        let (head_row_offset, head_col_offset) = initial_direction_mapping.get(direction).unwrap();

        for _ in 0..distance.parse().unwrap() {
            knot_indicies[0].0 += head_row_offset;
            knot_indicies[0].1 += head_col_offset;
            for i in 1..n_knots {
                if !((knot_indicies[i - 1].0 - knot_indicies[i].0).abs() >= 2
                    || (knot_indicies[i - 1].1 - knot_indicies[i].1).abs() >= 2)
                {
                    continue;
                }

                knot_indicies[i].0 += (knot_indicies[i - 1].0 - knot_indicies[i].0).signum();
                knot_indicies[i].1 += (knot_indicies[i - 1].1 - knot_indicies[i].1).signum();
            }
            tail_visited_grid[knot_indicies.last().unwrap().0 as usize]
                [knot_indicies.last().unwrap().1 as usize] = true;
        }
    }
    tail_visited_grid
}
