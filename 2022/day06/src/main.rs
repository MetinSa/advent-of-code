use itertools::Itertools;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read file");

    // Part 1)
    let marker_idx = get_marker_idx(&input_data, 4).expect("Marker was not found in part 1");
    println!(
        "Part 1: start-of-packet marker found after {} chars",
        marker_idx
    );

    // Part 2)
    let marker_idx = get_marker_idx(&input_data, 14).expect("Marker was not found in part 2");
    println!(
        "Part 2: start-of-message marker found after {} chars",
        marker_idx
    );
}

fn get_marker_idx(data: &str, distinct_chars: usize) -> Option<usize> {
    let char_collection: Vec<char> = data.chars().collect();
    for (idx, c) in char_collection.windows(distinct_chars).enumerate() {
        if c.iter().unique().count() == distinct_chars {
            return Some(idx + distinct_chars);
        }
    }
    None
}
