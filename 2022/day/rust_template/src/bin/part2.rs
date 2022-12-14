use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let lines: Vec<&str> = contents.split('\n').collect();

    for line in lines {
        let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let _ = re.captures(line).unwrap();
    }
}
