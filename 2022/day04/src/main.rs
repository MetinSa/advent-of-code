use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut full_overlap: u32 = 0;
    let mut partial_overlap: u32 = 0;
    for line in data.lines() {
        let (left, right) = line.split_once(",").unwrap();
        let (a_str, b_str) = left.split_once("-").unwrap();
        let (c_str, d_str) = right.split_once("-").unwrap();

        let a: u32 = a_str.parse().unwrap();
        let b: u32 = b_str.parse().unwrap();
        let c: u32 = c_str.parse().unwrap();
        let d: u32 = d_str.parse().unwrap();

        if (a <= c && b >= d) || (c <= a && d >= b) {
            full_overlap += 1;
        }
        if (a <= d && b >= c) || (d <= a && c >= b) {
            partial_overlap += 1;
        }
    }

    println!(
        "Part 1: there are {} fully contained assignment pairs",
        full_overlap
    );
    println!(
        "Part 2: there are {} partial overlapping pairs",
        partial_overlap
    );
}
