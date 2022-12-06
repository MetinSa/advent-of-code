use itertools::Itertools;
use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");
    let signal: Vec<char> = input_string.chars().collect();

    // Part 1)
    let marker_idx = get_marker_idx(&signal, 4).expect("Marker was not found in part 1");
    println!(
        "Part 1: start-of-packet marker found after {} characters",
        marker_idx
    );

    // Part 2)
    let marker_idx = get_marker_idx(&signal, 14).expect("Marker was not found in part 2");
    println!(
        "Part 2: start-of-message marker found after {} characters",
        marker_idx
    );
}

fn get_marker_idx(signal: &Vec<char>, distinct_characters: usize) -> Option<usize> {
    for (idx, c) in signal.windows(distinct_characters).enumerate() {
        if c.iter().unique().count() == distinct_characters {
            return Some(idx + distinct_characters);
        }
    }
    None
}
