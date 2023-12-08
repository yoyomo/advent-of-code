mod part2;
mod part1;

use std::fs::read_to_string;
use crate::part1::part1;
use crate::part2::part2;

fn main() {
    let filename = "data/sample.txt";

    let binding = read_to_string(filename).expect("");

    println!("{}", part1(binding.split("\n").collect()));
    println!("{}", part2(binding.split("\n").collect()));
    return
}
