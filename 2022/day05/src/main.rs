use std::collections::vec_deque::VecDeque;
use std::fs;
use std::str;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut crate_stacks: Vec<VecDeque<String>> = Vec::new();
    for (idx, line) in data.lines().enumerate() {
        let crate_layer: Vec<String> = line
            .as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .iter()
            .map(|x| x.trim().replace("[", "").replace("]", ""))
            .collect();

        if crate_layer[0] == "1" {
            // Finished with crate layers
            break;
        }

        if idx == 0 {
            // Initialize create stack layout
            for _ in 0..crate_layer.len() {
                crate_stacks.push(VecDeque::new());
            }
        }
        for (idx, c) in crate_layer.iter().enumerate() {
            if c.is_empty() {
                continue;
            }
            crate_stacks[idx].push_front(c.to_string());
        }
    }

    // Part 1
    let part1_reordered_stacks = move_one_by_one(&data, &crate_stacks);
    let top_crates: String = part1_reordered_stacks
        .iter()
        .map(|s| s.back().unwrap().to_string())
        .collect();
    println!("Part 1: the create order are {:?}", top_crates);

    // Part 2
    let part2_reordered_stacks = move_several(&data, &crate_stacks);
    let top_crates: String = part2_reordered_stacks
        .iter()
        .map(|s| s.back().unwrap().to_string())
        .collect();
    println!("Part 2: the create order are {:?}", top_crates);
}

fn move_one_by_one(data: &String, stacks: &Vec<VecDeque<String>>) -> Vec<VecDeque<String>> {
    let mut reordered_stacks = stacks.clone();
    for line in data.lines() {
        if line.starts_with("move") {
            let moves: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();

            for _ in 0..moves[0] {
                let c = reordered_stacks[moves[1] - 1].pop_back().unwrap();
                reordered_stacks[moves[2] - 1].push_back(c);
            }
        }
    }
    reordered_stacks
}

fn move_several(data: &String, stacks: &Vec<VecDeque<String>>) -> Vec<VecDeque<String>> {
    let mut reordered_stacks = stacks.clone();
    for line in data.lines() {
        if line.starts_with("move") {
            let moves: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();

            let mut stacked_crates: VecDeque<String> = VecDeque::new();
            for _ in 0..moves[0] {
                let c = reordered_stacks[moves[1] - 1].pop_back().unwrap();
                stacked_crates.push_front(c);
            }
            for c in stacked_crates {
                reordered_stacks[moves[2] - 1].push_back(c);
            }
        }
    }
    reordered_stacks
}
