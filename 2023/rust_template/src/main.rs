mod part2;
mod part1;

use std::fs::read_to_string;
use std::time::Instant;
use crate::part1::part1;
use crate::part2::part2;

fn call_part(filename: &str, part: fn(Vec<&str>) -> u32) {
	let binding = read_to_string(filename).expect("");
	let lines = binding.split("\n").collect();

	let start = Instant::now();
	println!("{:?} at {:?}", part(lines), start.elapsed());
}

fn main() {
	call_part("data/sample.txt", part1);
	call_part("data/input.txt", part1);

	call_part("data/sample.txt", part2);
	call_part("data/input.txt", part2);
	return;
}
