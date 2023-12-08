use regex::{Captures, Regex};
use crate::part2::get_wins;

fn get_num_array(g: Captures) -> Vec<u64> {
    let mut num_array: Vec<u64> = vec![];
    let times_str = g.get(1).unwrap().as_str().split(" ");
    for time_str in times_str {
        match time_str.parse() {
            Ok(num) => num_array.push(num),
            _ => {}
        }
    }
    return num_array;
}

pub fn part1(lines: Vec<&str>) -> i32 {

    let time_re = Regex::new(r"Time:(.*)").unwrap();
    let distance_re = Regex::new(r"Distance:(.*)").unwrap();

    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];

    for line in lines {
        match time_re.captures(line) {
            Some(g) => {
                times = get_num_array(g)
            },
            None => {},
        }
        match distance_re.captures(line) {
            Some(g) => {
                distances = get_num_array(g)
            },
            None => {},
        }
    }

    assert_eq!(times.len(), distances.len());

    let races = times.len();

    let mut result = 1;

    for race in 0..races {

        let time = times[race];
        let distance = distances[race];

        result *= get_wins(time, distance);
    }

    return result
}
