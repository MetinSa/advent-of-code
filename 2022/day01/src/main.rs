use std::fs;

fn main() {
    let data = read_input("input.txt");
    let calories = get_summed_elf_calories(&data);

    // Part a)
    let max_calories = calories.iter().max().unwrap();
    println!("The most calories carried by an elf is: {}", max_calories);

    // Part b)
    let top_three_max_calories: u32 = calories.iter().rev().take(3).sum();
    println!(
        "The top three elves are carring {} calories",
        top_three_max_calories
    );
}

fn get_summed_elf_calories(input: &str) -> Vec<u32> {
    let mut sum: u32 = 0;
    let mut sums = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    sums
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
