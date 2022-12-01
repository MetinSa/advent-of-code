use crate::common;

pub fn solution() {
    let input = common::read_input("input.txt");
    let sum_of_chunks = common::sum_chunks(&input);
    let max_chunk = sum_of_chunks.iter().max().unwrap();
    println!("The most calories carried by an elf is: {}", max_chunk)
}
