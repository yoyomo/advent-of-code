mod part2;
mod part1;

use std::fs::read_to_string;
use crate::part1::part1;
use crate::part2::part2;

fn main() {
    let filename = "data/input.txt";

    let binding = read_to_string(filename).unwrap();
    let lines = binding.lines();

    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines));
    return
}
