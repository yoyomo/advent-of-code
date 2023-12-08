use std::fs::read_to_string;

pub fn part1() -> u32 {
    let filename = "data/input.txt";

    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let mut first = 0 ;
        let mut last = 0;
        for letter in line.chars() {
            if letter.is_digit(10) {
                let digit = letter.to_digit(10).unwrap();
                if first == 0 {
                    first = digit * 10;
                }
                last = digit;
            }
        }

        result += first + last;
    }

    return result
}
