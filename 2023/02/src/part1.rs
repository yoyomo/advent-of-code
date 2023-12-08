use std::str::Lines;
use regex::Regex;

pub fn part1(lines: Lines) -> i32 {
    let game_re = Regex::new(r"Game ([0-9]+):(.*)").unwrap();
    let set_re = Regex::new(r"([0-9]+) (blue|red|green)").unwrap();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut ids = 0;
    'lines: for line in lines {
        let game_groups = game_re.captures(line).unwrap();
        let id: i32 = game_groups.get(1).unwrap().as_str().parse().unwrap();
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
                if red > max_red || green > max_green || blue > max_blue {
                    continue 'lines;
                }
            }
        }

        ids += id;

    }

    return ids;
}
