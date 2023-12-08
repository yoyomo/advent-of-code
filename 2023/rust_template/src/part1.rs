use std::str::Lines;
use regex::Regex;

pub fn part1(lines: Lines) -> &'static str {

    let re = Regex::new(r".*").unwrap();

    for line in lines {
        let groups = re.captures(line).unwrap();
        println!("{}", groups.get(0).unwrap().as_str())
    }

    return ""
}