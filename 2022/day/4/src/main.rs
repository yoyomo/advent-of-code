use std::{fs, ops::Range};

fn get_range(pair: &str) -> Range<u32> {
    let range_str: Vec<&str> = pair.split('-').collect();
    let start: u32 = range_str[0].parse().unwrap();
    let end: u32 = range_str[1].parse().unwrap();
    start..end
}

fn is_fully_contained(a: &Range<u32>, b: &Range<u32>) -> bool{
    a.start >= b.start && a.end <= b.end
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let pairs: Vec<&str> = contents.split('\n').collect();

    let mut fully_contained: u32 = 0;
    for p in pairs {
        let pair: Vec<&str> = p.split(',').collect();

        let first_range = get_range(pair[0]);
        let second_range = get_range(pair[1]);

        if is_fully_contained(&first_range, &second_range) || is_fully_contained(&second_range, &first_range) {
            fully_contained += 1;
        }

    }

    println!("{fully_contained}")
}
