use std::{fs, ops::Range};

fn get_range(pair: &str) -> Range<u32> {
    let range_str: Vec<&str> = pair.split('-').collect();
    let start: u32 = range_str[0].parse().unwrap();
    let end: u32 = range_str[1].parse().unwrap();
    start..end
}

fn does_overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start >= b.start && a.start <= b.end || a.end <= b.end && a.end >= b.start
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let pairs: Vec<&str> = contents.split('\n').collect();

    let mut overlaps: u32 = 0;
    for p in pairs {
        let pair: Vec<&str> = p.split(',').collect();

        let first_range = get_range(pair[0]);
        let second_range = get_range(pair[1]);

        if does_overlap(&first_range, &second_range) || does_overlap(&second_range, &first_range) {
            overlaps += 1;
        }

    }

    println!("{overlaps}")
}
