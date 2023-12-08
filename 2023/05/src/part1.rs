use std::collections::HashMap;
use regex::{Regex};

fn get_next_chain(source_map: &Vec<(usize, usize, usize)>, source: usize) -> usize {
    for tuple in source_map {
        let d = tuple.0;
        let s = tuple.1;
        let range = tuple.2;
        if source >= s && source <= s + range {
            return d + source - s;
        }
    }

    return source;
}

pub fn part1(lines: Vec<&str>) -> usize {

    let mut src_dest_map: HashMap<&str, Vec<(usize,usize,usize)>> = HashMap::new();

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
                src_dest_map.insert(src, vec![]);
            }
            None => {}
        }

        match dest_src_range_re.captures(line) {
            Some(g) => {
                let d: usize = g.get(1).unwrap().as_str().parse().unwrap();
                let s: usize = g.get(2).unwrap().as_str().parse().unwrap();
                let range: usize = g.get(3).unwrap().as_str().parse().unwrap();

                src_dest_map.entry(src).and_modify(|v| {
                    v.push((d,s,range));
                });
            }
            None => {}
        }
    }

    let mut min_location = usize::MAX;
    for seed in seeds {
        let soil = get_next_chain(src_dest_map.get("seed").unwrap(), seed);
        let fertilizer = get_next_chain(src_dest_map.get("soil").unwrap(), soil);
        let water = get_next_chain(src_dest_map.get("fertilizer").unwrap(), fertilizer);
        let temperature = get_next_chain(src_dest_map.get("water").unwrap(), water);
        let humidity = get_next_chain(src_dest_map.get("temperature").unwrap(), temperature);
        let location = get_next_chain(src_dest_map.get("humidity").unwrap(), humidity);
        if location < min_location {
            min_location = location
        }
    }

    return min_location;
}
