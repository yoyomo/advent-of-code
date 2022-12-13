use std::fs;

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let _: Vec<&str> = contents.split('\n').collect();
}
