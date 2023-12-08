use std::collections::HashMap;
use regex::{Regex};

pub struct Node {
    left: &'static str,
    right: &'static str,
}

pub fn part1(lines: Vec<&str>) -> usize {
    let instructions_re = Regex::new(r"([RL]+)").unwrap();
    let node_re = Regex::new(r"([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();

    let mut instructions = "";

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in lines {
        match instructions_re.captures(line) {
            Some(g) => {
                instructions = g.get(1).unwrap().as_str();
                continue;
            }
            None => {}
        }

        match node_re.captures(line) {
            Some(g) => {
                let node = g.get(1).unwrap().as_str();
                let left = g.get(2).unwrap().as_str();
                let right = g.get(3).unwrap().as_str();
                nodes.insert(node, Node {
                    left,
                    right,
                });
            }
            None => {}
        }
    }

    let mut current_node = "AAA";
    let mut steps: usize = 1;
    let mut i: usize = 0;
    while i < i % instructions.len() {
        let instruction = instructions.chars().nth(i).unwrap();

        match instruction {
            'L' => {
                current_node = nodes.get(current_node).unwrap().left
            }
            'R' => {
                current_node = nodes.get(current_node).unwrap().right
            }
            _ => {}
        }

        if current_node == "ZZZ" {
            break;
        }

        steps += 1;
        i += 1;
        i = i % instructions.len()
    }

    return steps;
}
