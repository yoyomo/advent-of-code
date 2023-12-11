mod part2;
mod part1;

use std::fs::read_to_string;
use std::time::Instant;
use crate::part1::part1;
use crate::part2::part2;

fn call_fn(filename: String, func: fn(Vec<&str>) -> usize) {
    let binding = read_to_string(filename).expect("");

    let start = Instant::now();
    println!("{:?} at {:?}", func(binding.split("\n").collect()), start.elapsed());
}

fn main() {
    for f in 1..5 {
        call_fn(format!("data/sample{f}.txt"), part1);
    }
    call_fn("data/input.txt".to_string(), part1);

    for f in 5..9 {
        call_fn(format!("data/sample{f}.txt"), part2);
    }
    call_fn("data/input.txt".to_string(), part2);
    return
}
