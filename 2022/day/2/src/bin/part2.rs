use std::fs;

enum POINTS {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}
enum LOSS {
    ROCK = 3,
    PAPER = 1,
    SCISSOR = 2,
}
enum WIN {
    ROCK = 2,
    PAPER = 3,
    SCISSOR = 1,
}
enum RESULTS {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSOR: &str = "C";

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

fn get_point (letter: &str) -> POINTS {
    if letter == ROCK {
        POINTS::ROCK
    }
    else if letter == PAPER {
        POINTS::PAPER
    }
    else if letter == SCISSOR{
        POINTS::SCISSOR
    }
    else {
        POINTS::SCISSOR // return null or something
    }
}

fn get_result (letter: &str) -> RESULTS {
    if letter == LOSE {
        RESULTS::LOSE
    }
    else if letter == DRAW {
        RESULTS::DRAW
    }
    else if letter == WIN{
        RESULTS::WIN
    }
    else {
        RESULTS::LOSE // return null or something
    }
}

fn get_win (letter: &str) -> WIN {
    if letter == ROCK {
        WIN::ROCK
    }
    else if letter == PAPER {
        WIN::PAPER
    }
    else if letter == SCISSOR{
        WIN::SCISSOR
    }
    else {
        WIN::ROCK // return null or something
    }
}

fn get_loss (letter: &str) -> LOSS {
    if letter == ROCK {
        LOSS::ROCK
    }
    else if letter == PAPER {
        LOSS::PAPER
    }
    else if letter == SCISSOR{
        LOSS::SCISSOR
    }
    else {
        LOSS::ROCK // return null or something
    }
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("");

    println!("{contents}");

    let battles: Vec<&str> = contents.split('\n').collect();

    let mut points: u32 = 0;
    for battle in battles {
        let columns: Vec<&str> = battle.split(' ').collect();

        let first_hand = columns[0];
        let result = columns[1];
        let player_one_point = get_point(&first_hand);
        let player_two_result = get_result(&result);

        points += player_two_result as u32;

        if result == DRAW {
            points += player_one_point as u32;
        } else if result == WIN {
            points += get_win(first_hand) as u32;
        } else {
            points += get_loss(first_hand) as u32;
        }
    }

    println!("{points}")
}
