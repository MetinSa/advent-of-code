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

fn process(input: &str) -> u32 {
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

    let mut min_sets: Vec<Set> = Vec::new();
    games.iter().for_each(|game| {
        min_sets.push(Set {
            blue: game.iter().max_by_key(|set| set.blue).unwrap().blue,
            red: game.iter().max_by_key(|set| set.red).unwrap().red,
            green: game.iter().max_by_key(|set| set.green).unwrap().green,
        });
    });

    let mut score = 0;
    min_sets.iter().for_each(|set| {
        score += set.blue * set.red * set.green;
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
        assert_eq!(process(input), 2286);
    }
}
