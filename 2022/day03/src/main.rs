use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn main() {
    let data = read_input("input.txt");
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    // Part 1
    let mut priority: u32 = 0;
    for line in data.iter() {
        let (first, second) = line.split_at(line.len() / 2);
        let unique_chars: HashSet<char> = first.chars().collect();
        for c in second.chars() {
            if unique_chars.contains(&c) {
                priority += alphabet.find(c).unwrap() as u32 + 1;
                break;
            }
        }
    }
    println!("Part 1: the sum of the priorities are {}", priority);

    // Part 2
    let mut priority: u32 = 0;
    let mut shared_chars_in_first_two: Vec<char> = Vec::new();

    for (first, second, third) in data.iter().tuples() {
        let unique_chars: HashSet<char> = first.chars().collect();
        for c in second.chars() {
            if unique_chars.contains(&c) {
                shared_chars_in_first_two.push(c);
            }
        }
        let unique_shared_chars_in_first_two: HashSet<char> =
            HashSet::from_iter(shared_chars_in_first_two.clone());

        for c in third.chars() {
            if unique_shared_chars_in_first_two.contains(&c) {
                priority += alphabet.find(c).unwrap() as u32 + 1;
                break;
            }
        }
        shared_chars_in_first_two.clear();
    }
    println!("Part 2: the sum of the priorities are {}", priority);
}

fn read_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string()) // Convert &str to String
        .collect()
}
