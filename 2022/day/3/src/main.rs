use std::fs;

const BASE_DIFFERENCE: u32 = 64; // ignore all ascii chars before uppercase A
const UNUSED_DIFFERENCE: u32 = 6; // ascii chars like [] etc
const TOTAL_LETTERS: u32 = 26;

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let rucksacks: Vec<&str> = contents.split('\n').collect();
    let mut sum: u32 = 0;
    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

        let mut existing_items: String = "".to_string();

        for item in second_compartment.chars() {
            if first_compartment.contains(item) && !existing_items.contains(item) {

                if item.is_ascii_uppercase() {
                    sum += item as u32 - BASE_DIFFERENCE + TOTAL_LETTERS;
                } else {
                    sum += item as u32 - BASE_DIFFERENCE - TOTAL_LETTERS - UNUSED_DIFFERENCE;
                }

                existing_items.push(item);

            }
        }


    }

    println!("{sum}");
}
