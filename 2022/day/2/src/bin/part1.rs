use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum POINTS {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}

enum WIN {
    ROCK = 3,
    PAPER = 1,
    SCISSOR = 2,
}

enum RESULTS {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

const ROCKS: [&str; 2] = ["A", "X"];
const PAPERS: [&str; 2] = ["B", "Y"];
const SCISSORS: [&str; 2] = ["C", "Z"];

fn get_type (letter: &str) -> POINTS {
    if ROCKS.contains(&letter) {
        POINTS::ROCK
    }
    else if PAPERS.contains(&letter){
        POINTS::PAPER
    }
    else if SCISSORS.contains(&letter){
        POINTS::SCISSOR
    }
    else {
        POINTS::SCISSOR // return null or something
    }
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("");

    println!("{contents}");

    let battles: Vec<&str> = contents.split('\n').collect();

    let mut points: u32 = 0;
    for battle in battles {
        let players: Vec<&str> = battle.split(' ').collect();

        let player_one_type = get_type(&players[0]);
        let player_two_type = get_type(&players[1]);

        points += player_two_type as u32;

        if player_one_type == player_two_type {
            points += RESULTS::DRAW as u32;
        } else if player_two_type as u32 == POINTS::ROCK as u32 && player_one_type as u32 == WIN::ROCK as u32 {
            points += RESULTS::WIN as u32;
        } else if player_two_type as u32 == POINTS::PAPER as u32 && player_one_type as u32 == WIN::PAPER as u32 {
            points += RESULTS::WIN as u32;
        } else if player_two_type as u32 == POINTS::SCISSOR as u32 && player_one_type as u32 == WIN::SCISSOR as u32 {
            points += RESULTS::WIN as u32;
        } else {
            points += RESULTS::LOSE as u32;
        }
        
    }

    println!("{points}")

}