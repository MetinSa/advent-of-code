fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]

struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

fn process(input: &str) -> usize {
    let configuration = Set {
        blue: 14,
        red: 12,
        green: 13,
    };
    let mut games: Vec<Vec<Set>> = Vec::new();
    for line in input.lines() {
        let mut sets: Vec<Set> = Vec::new();
        let game_str = line.split(":").last().unwrap();
        for set_str in game_str.split(";") {
            let mut set = Set {
                blue: 0,
                red: 0,
                green: 0,
            };
            for draw in set_str.split(",") {
                match draw.trim().split_once(" ").unwrap() {
                    (num, "blue") => set.blue += num.parse::<u32>().unwrap(),
                    (num, "red") => set.red += num.parse::<u32>().unwrap(),
                    (num, "green") => set.green += num.parse::<u32>().unwrap(),
                    _ => panic!("Invalid color"),
                };
            }
            sets.push(set);
        }
        games.push(sets);
    }

    let mut score: usize = 0;
    games.iter().enumerate().for_each(|(idx, game)| {
        if !game.iter().any(|set| {
            (set.red > configuration.red)
                || (set.blue > configuration.blue)
                || (set.green > configuration.green)
        }) {
            score += idx + 1
        }
    });
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process(input), 8);
    }
}
