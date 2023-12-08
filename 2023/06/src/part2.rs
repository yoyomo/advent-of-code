use regex::{Captures, Regex};

fn get_number(g: Captures) -> u64 {
    return g.get(1).unwrap().as_str().split_whitespace().collect::<String>().parse().unwrap();
}

pub fn get_wins(time: u64, distance: u64) -> i32 {
    let mut hold_the_button = 0;
    let mut wins = 0;
    while hold_the_button < time {
        let distance_traveled = (time - hold_the_button) * hold_the_button;

        if distance_traveled > distance {
            wins += 1;
        } else if wins > 0 {
            break;
        }

        hold_the_button += 1;
    }
    return wins
}

pub fn part2(lines: Vec<&str>) -> i32 {
    let time_re = Regex::new(r"Time:(.*)").unwrap();
    let distance_re = Regex::new(r"Distance:(.*)").unwrap();

    let mut time = 0;
    let mut distance = 0;

    for line in lines {
        match time_re.captures(line) {
            Some(g) => {
                time = get_number(g)
            }
            None => {}
        }
        match distance_re.captures(line) {
            Some(g) => {
                distance = get_number(g)
            }
            None => {}
        }
    }



    return get_wins(time, distance);
}
