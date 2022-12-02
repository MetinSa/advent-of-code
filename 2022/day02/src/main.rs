use std::fs;

enum GameMove {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn main() {
    let input_str = fs::read_to_string("input.txt").expect("could not read file");

    part_a(&input_str);
    part_b(&input_str);
}

fn part_a(input_str: &str) {
    let mut total_score = 0;
    for line in input_str.lines() {
        let (elf_str, player_str) = line.split_once(" ").unwrap();

        let elf_move = get_elf_move_from_str(elf_str);
        let player_move = get_player_move_from_str(player_str);
        let result = play_game(&player_move, &elf_move);

        total_score += result as u32 + player_move as u32;
    }
    println!("Total score part a): {}", total_score);
}

fn part_b(input_str: &str) {
    let mut total_score = 0;
    for line in input_str.lines() {
        let (elf_str, player_str) = line.split_once(" ").unwrap();

        let elf_move = get_elf_move_from_str(elf_str);
        let result = get_result_from_str(player_str);
        let player_move = get_player_move_from_result(&result, &elf_move);

        total_score += player_move as u32 + result as u32;
    }
    println!("Total score part b): {}", total_score);
}

fn get_elf_move_from_str(elf_str: &str) -> GameMove {
    match elf_str {
        "A" => GameMove::Rock,
        "B" => GameMove::Paper,
        "C" => GameMove::Scissor,
        _ => panic!("Invalid elf move"),
    }
}

fn get_player_move_from_str(player_str: &str) -> GameMove {
    match player_str {
        "X" => GameMove::Rock,
        "Y" => GameMove::Paper,
        "Z" => GameMove::Scissor,
        _ => panic!("Invalid player move"),
    }
}

fn get_result_from_str(player_str: &str) -> GameResult {
    match player_str {
        "X" => GameResult::Loss,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Invalid game result"),
    }
}

fn play_game(player_move: &GameMove, elf_move: &GameMove) -> GameResult {
    match player_move {
        GameMove::Rock => match elf_move {
            GameMove::Rock => GameResult::Draw,
            GameMove::Paper => GameResult::Loss,
            GameMove::Scissor => GameResult::Win,
        },
        GameMove::Paper => match elf_move {
            GameMove::Rock => GameResult::Win,
            GameMove::Paper => GameResult::Draw,
            GameMove::Scissor => GameResult::Loss,
        },
        GameMove::Scissor => match elf_move {
            GameMove::Rock => GameResult::Loss,
            GameMove::Paper => GameResult::Win,
            GameMove::Scissor => GameResult::Draw,
        },
    }
}

fn get_player_move_from_result(game_result: &GameResult, elf_move: &GameMove) -> GameMove {
    match game_result {
        GameResult::Win => match elf_move {
            GameMove::Rock => GameMove::Paper,
            GameMove::Paper => GameMove::Scissor,
            GameMove::Scissor => GameMove::Rock,
        },
        GameResult::Draw => match elf_move {
            GameMove::Rock => GameMove::Rock,
            GameMove::Paper => GameMove::Paper,
            GameMove::Scissor => GameMove::Scissor,
        },
        GameResult::Loss => match elf_move {
            GameMove::Rock => GameMove::Scissor,
            GameMove::Paper => GameMove::Rock,
            GameMove::Scissor => GameMove::Paper,
        },
    }
}
