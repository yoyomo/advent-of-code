use std::collections::HashMap;
use regex::{Regex};

pub struct Node {
    left: String,
    right: String,
}

pub fn part1(lines: Vec<&str>) -> usize {
    let instructions_re = Regex::new(r"([RL]+)").unwrap();
    let node_re = Regex::new(r"([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();

    let mut instructions: String = "".to_string();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut first_node: String = "".to_string();

    for line in lines {
        if instructions.is_empty(){
            match instructions_re.captures(line) {
                Some(g) => {
                    instructions = g.get(1).unwrap().as_str().to_string();
                }
                None => {}
            }
        }

        match node_re.captures(line) {
            Some(g) => {
                let node = g.get(1).unwrap().as_str().to_string();
                let left = g.get(2).unwrap().as_str().to_string();
                let right = g.get(3).unwrap().as_str().to_string();
                if first_node.is_empty() {
                    first_node = node.clone();
                }
                nodes.insert(node, Node {
                    left,
                    right,
                });

            }
            None => {}
        }
    }

    let mut current_node = "AAA".to_string();
    let mut steps: usize = 1;
    let mut i: usize = 0;
    loop {
        let instruction = instructions.chars().nth(i).unwrap();
        match instruction {
            'L' => {
                current_node = nodes.get(current_node.as_str()).unwrap().left.clone()
            }
            'R' => {
                current_node = nodes.get(current_node.as_str()).unwrap().right.clone()
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
