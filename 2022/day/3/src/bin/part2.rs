use std::fs;

const BASE_DIFFERENCE: u32 = 64; // ignore all ascii chars before uppercase A
const UNUSED_DIFFERENCE: u32 = 6; // ascii chars like [] etc
const TOTAL_LETTERS: u32 = 26;

const GROUP_CAPACITY: u32 = 3;

fn get_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        item as u32 - BASE_DIFFERENCE + TOTAL_LETTERS
    } else {
        item as u32 - BASE_DIFFERENCE - TOTAL_LETTERS - UNUSED_DIFFERENCE
    }
}

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let rucksacks: Vec<&str> = contents.split('\n').collect();

    let mut sum: u32 = 0;
    let mut group_item: char = '\0';

    for group_rucksacks in rucksacks.chunks(GROUP_CAPACITY as usize) {

        for item in group_rucksacks[0].chars() {
            if group_rucksacks[1].contains(item) && group_rucksacks[2].contains(item) {
                group_item = item;
                break;
            }
        }

        sum += get_priority(group_item);

    }

    println!("{sum}");
}
