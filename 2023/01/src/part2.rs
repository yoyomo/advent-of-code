use std::fs::read_to_string;

pub fn part2() -> u32 {
    let filename = "data/input.txt";

    let number_words = [["one", "1"], ["two", "2"], ["three", "3"], ["four", "4"], ["five", "5"], ["six", "6"], ["seven", "7"], ["eight", "8"], ["nine", "9"]];

    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        let mut first_index = usize::MAX;
        let mut last_index = 0;

        for (n, order) in number_words.into_iter().enumerate() {
            for number in order {
                let found_first = line.find(number);
                if found_first.is_some() {
                    let index = found_first.unwrap();
                    if index <= first_index {
                        first = ((n + 1) * 10) as u32;
                        first_index = index;
                    }
                }

                let found_last = line.rfind(number);
                if found_last.is_some() {
                    let index = found_last.unwrap();
                    if index >= last_index {
                        last = (n + 1) as u32;
                        last_index = index;
                    }
                }
            }
        }

        result += first + last;
    }

    result
}
