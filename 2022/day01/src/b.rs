use itertools::Itertools;
use crate::common;

pub fn solution() {
    let input = common::read_input("input.txt");
    let sum_of_chunks = common::sum_chunks(&input);
    let ordered_sum_of_chunks = sum_of_chunks.iter().sorted();
    let sum_of_three_largest_chunks: i32 = ordered_sum_of_chunks.rev().take(3).sum();
    println!(
        "The top three elves are carring {} calories",
        sum_of_three_largest_chunks
    );
}
