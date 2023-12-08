use std::str::Lines;
use regex::Regex;

pub fn part2(lines: Lines) -> i32 {
    let game_re = Regex::new(r"Game ([0-9]+):(.*)").unwrap();
    let set_re = Regex::new(r"([0-9]+) (blue|red|green)").unwrap();


    let mut sum_of_powers = 0;
    for line in lines {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let game_groups = game_re.captures(line).unwrap();
        let game = game_groups.get(2).unwrap().as_str();

        for set in game.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for draw in set.split(", ") {
                let set_groups = set_re.captures(draw).unwrap();
                let quantity: i32 = set_groups.get(1).unwrap().as_str().parse().unwrap();
                let color = set_groups.get(2).unwrap().as_str();

                match color {
                    "red" => red = quantity,
                    "green" => green = quantity,
                    "blue" => blue = quantity,
                    _ => {}
                }
                if red > max_red {
                    max_red = red;
                }
                if green > max_green {
                    max_green = green
                }
                if blue > max_blue {
                    max_blue = blue
                }
            }
        }

        let power = max_red * max_green * max_blue;

        sum_of_powers += power;
    }

    return sum_of_powers;
}
