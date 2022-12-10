use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading the file");

    let n_pixel_cols: i32 = 40;
    let mut row_idx: i32;

    let mut total_cycles: u32 = 0;
    let mut cycle: u32 = 1;
    let mut registry: i32 = 1;

    let mut total_signal_strength: i32 = 0;

    let mut command: Option<i32>;
    println!("Part 2:  Display:");
    for line in input.lines() {
        if line.starts_with("addx") {
            cycle += 2;
            command = Some(line.split_once(" ").unwrap().1.parse::<i32>().unwrap());
        } else {
            cycle += 1;
            command = None;
        }

        while cycle > 0 {
            total_cycles += 1;
            cycle -= 1;

            if cycle == 0 && command.is_some() {
                registry += command.unwrap();
            }

            row_idx = (total_cycles as i32 - 1) % n_pixel_cols;
            if (registry - row_idx).abs() <= 1 {
                print! {"#"};
            } else {
                print!(".");
            }

            if row_idx + 1 - 20 == 0 {
                total_signal_strength += registry * total_cycles as i32;
            }

            if row_idx == n_pixel_cols - 1 {
                println!("");
            }
        }
    }
    println!(
        "Part 1:  The sum of si signal strengths are {}",
        total_signal_strength
    );
}
