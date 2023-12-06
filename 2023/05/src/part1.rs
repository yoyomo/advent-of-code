use std::collections::HashMap;
use regex::{Regex};

pub fn part1(lines: Vec<&str>) -> usize {

    let mut src_dest_map: HashMap<&str, HashMap<usize, usize>> = HashMap::new();

    let seeds_re = Regex::new(r"seeds:\s(.*)").unwrap();
    let src_dest_re = Regex::new(r"(.*)-to-(.*)\smap:").unwrap();
    let dest_src_range_re = Regex::new(r"([0-9]+)\s([0-9]+)\s([0-9]+)").unwrap();

    let mut seeds: Vec<usize> = vec![];

    let mut src = "";
    // let mut dest = "";

    for line in lines {
        match seeds_re.captures(line) {
            Some(g) => {
                let seeds_str: Vec<&str> = g.get(1).unwrap().as_str().split(" ").collect();
                for seed_str in seeds_str {
                    seeds.push(seed_str.parse().unwrap());
                }
                continue;
            }
            None => {}
        };

        match src_dest_re.captures(line) {
            Some(g) => {
                src = g.get(1).unwrap().as_str();
                // dest = g.get(2).unwrap().as_str();
                src_dest_map.insert(src, HashMap::new());
            }
            None => {}
        }

        match dest_src_range_re.captures(line) {
            Some(g) => {
                let mut d: usize = g.get(1).unwrap().as_str().parse().unwrap();
                let s: usize = g.get(2).unwrap().as_str().parse().unwrap();
                let range: usize = g.get(3).unwrap().as_str().parse().unwrap();

                for i in s..s+range {
                    src_dest_map.entry(src).and_modify(|v| {
                        v.insert(i, d);
                    });
                    d += 1;
                }
            }
            None => {}
        }
    }

    let mut min_location = usize::MAX;
    for seed in seeds {
        let soil = match src_dest_map.get("seed").unwrap().get(&seed){
            Some(value) => {value}
            None => {&seed}
        };
        let fertilizer = match src_dest_map.get("soil").unwrap().get(soil){
            Some(value) => {value}
            None => {soil}
        };
        let water = match src_dest_map.get("fertilizer").unwrap().get(fertilizer){
            Some(value) => {value}
            None => {fertilizer}
        };
        let temperature = match src_dest_map.get("water").unwrap().get(water){
            Some(value) => {value}
            None => {water}
        };
        let humidity = match src_dest_map.get("temperature").unwrap().get(temperature){
            Some(value) => {value}
            None => {temperature}
        };
        let location = match src_dest_map.get("humidity").unwrap().get(humidity){
            Some(value) => {value}
            None => {humidity}
        };

        if location < &min_location {
            min_location = *location
        }
    }

    return min_location;
}
