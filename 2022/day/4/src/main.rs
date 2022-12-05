fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let rucksacks: Vec<&str> = contents.split('\n').collect();
}
