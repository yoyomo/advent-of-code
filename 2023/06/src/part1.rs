use regex::{Captures, Regex};

fn get_num_array(g: Captures) -> Vec<u32> {
    let mut num_array: Vec<u32> = vec![];
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

    let mut times: Vec<u32> = vec![];
    let mut distances: Vec<u32> = vec![];

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

        let mut hold_the_button = 0;
        let mut wins = 0;
        while hold_the_button < times[race] {

            let distance_traveled = (time - hold_the_button) * hold_the_button;

            if distance_traveled > distance {
                wins += 1;
            } else if wins > 0 {
                break;
            }

            hold_the_button += 1;
        }

        result *= wins;
    }

    return result
}
