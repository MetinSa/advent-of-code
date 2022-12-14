use std::cmp::Reverse;
use std::{borrow::BorrowMut, collections::VecDeque, fs};

use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut grid: VecDeque<VecDeque<char>> = input_str
        .lines()
        .map(|line| line.chars().collect::<VecDeque<char>>())
        .collect();

    // Add padding to the grid to avoid index out of bounds errors.
    for row in grid.borrow_mut() {
        row.push_front('0');
        row.push_back('0');
    }
    grid.push_front(VecDeque::from(vec!['0'; grid[0].len()]));
    grid.push_back(VecDeque::from(vec!['0'; grid[0].len()]));

    // Use a binary heap to track steps of different routes.
    let (start_indicies, end_indicies) = get_start_and_end_indices(&grid);
    let mut candidates: BinaryHeap<Reverse<(i32, (usize, usize))>> = BinaryHeap::new();

    // Store steps for each position to avoid routes where we already found a shorter route.
    let mut steps: HashMap<(usize, usize), i32> = HashMap::new();

    candidates.push(Reverse((0, start_indicies)));

    loop {
        let (current_steps, current_indices) = candidates.pop().unwrap().0;

        if current_indices == end_indicies {
            println!("Fastest route requires {} steps.", current_steps);
            break;
        }
        let neighbors = get_valid_neighbors(current_indices, &grid);

        // Part 1:
        // let new_steps = current_steps + 1;

        // Part 2:
        let new_steps = if grid[current_indices.0][current_indices.1] == 'a' {
            1
        } else {
            current_steps + 1
        };

        for neigbor in neighbors {
            if steps.contains_key(&neigbor) && steps[&neigbor] <= new_steps {
                continue;
            }
            steps.insert(neigbor, new_steps);
            candidates.push(Reverse((new_steps, neigbor)));
        }
    }
}

fn get_start_and_end_indices(grid: &VecDeque<VecDeque<char>>) -> ((usize, usize), (usize, usize)) {
    let start_value = 'S';
    let end_value = 'E';
    let start_row = grid
        .iter()
        .position(|row| row.contains(&start_value))
        .unwrap();

    let start_col = grid[start_row]
        .iter()
        .position(|col| *col == start_value)
        .unwrap();

    let end_row = grid
        .iter()
        .position(|row| row.contains(&end_value))
        .unwrap();
    let end_col = grid[end_row]
        .iter()
        .position(|col| *col == end_value)
        .unwrap();

    ((start_row, start_col), (end_row, end_col))
}

fn get_valid_neighbors(
    current_indices: (usize, usize),
    grid: &VecDeque<VecDeque<char>>,
) -> Vec<(usize, usize)> {
    let mut valid_neighbors = vec![];
    let (x, y) = current_indices;

    let neighbors = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)];
    for neighbor in neighbors {
        let current_value = grid[current_indices.0][current_indices.1];
        let neighbor_value = grid[neighbor.0][neighbor.1];

        if neighbor_value == '0' || (neighbor_value == 'E' && current_value != 'z') {
            continue;
        } else if current_value == 'S' && neighbor_value == 'a' {
            valid_neighbors.push(neighbor);
        } else if neighbor_value == 'E' && current_value == 'z' {
            valid_neighbors.push(neighbor);
        } else if ((neighbor_value as i32) - (current_value as i32)).abs() <= 1 {
            valid_neighbors.push(neighbor);
        } else if (neighbor_value as i32) < (current_value as i32) {
            valid_neighbors.push(neighbor);
        }
    }
    valid_neighbors
}
