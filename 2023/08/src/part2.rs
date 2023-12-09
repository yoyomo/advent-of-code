use std::collections::HashMap;
use regex::{Regex};

pub struct Node {
    left: String,
    right: String,
}

pub fn part2(lines: Vec<&str>) -> usize {
    let instructions_re = Regex::new(r"([RL]+)").unwrap();
    let node_re = Regex::new(r"([A-Z0-9]{3})\s=\s\(([A-Z0-9]{3}),\s([A-Z0-9]{3})\)").unwrap();

    let mut instructions: String = "".to_string();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut starting_nodes: Vec<String> = vec![];

    for line in lines {
        if instructions.is_empty() {
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
                starting_nodes.push(node.clone());
                nodes.insert(node, Node {
                    left,
                    right,
                });
            }
            None => {}
        }
    }

    let mut current_nodes: Vec<String> = starting_nodes.into_iter().filter(|n| n.ends_with('A')).collect::<Vec<String>>();
    let mut steps: Vec<usize> = vec![1; current_nodes.len()];

    for c in 0..current_nodes.len() {
        let mut i: usize = 0;
        loop {
            let instruction = instructions.chars().nth(i).unwrap();

            let current_node = current_nodes.get(c).unwrap();
            match instruction {
                'L' => {
                    current_nodes[c] = nodes.get(current_node.as_str()).unwrap().left.clone()
                }
                'R' => {
                    current_nodes[c] = nodes.get(current_node.as_str()).unwrap().right.clone()
                }
                _ => {}
            }

            if current_nodes[c].ends_with('Z') {
                break;
            }

            steps[c] += 1;
            i += 1;
            i = i % instructions.len()
        }
    }

    let mut total_steps = 1;

    for step in steps {
        total_steps = num::integer::lcm(step, total_steps);
    }

    return total_steps;
}
