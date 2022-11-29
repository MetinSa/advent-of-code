#![crate_name = "day01"]

use std::fs;

fn main() {
    let input = read_input("input.txt");
    println!("{:?}", input);
}

fn read_input(filename: &str) -> Vec<usize>{
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}