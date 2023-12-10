mod part2;
mod part1;

use std::fs::read_to_string;
use std::time::Instant;
use crate::part1::part1;
use crate::part2::part2;

fn call_fn(filename: String) {
    let binding = read_to_string(filename).expect("");

    let mut start = Instant::now();
    println!("{:?} at {:?}", part1(binding.split("\n").collect()), start.elapsed());
    start = Instant::now();
    println!("{} at {:?}", part2(binding.split("\n").collect()), start.elapsed());
}

fn main() {
    for f in 1..5 {
        call_fn(format!("data/sample{f}.txt"));
    }
    call_fn("data/input.txt".to_string());
    return
}
