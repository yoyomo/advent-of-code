use std::fs;

use regex::Regex;

//     [W]         [J]     [J]
//     [V]     [F] [F] [S] [S]
//     [S] [M] [R] [W] [M] [C]
//     [M] [G] [W] [S] [F] [G]     [C]
// [W] [P] [S] [M] [H] [N] [F]     [L]
// [R] [H] [T] [D] [L] [D] [D] [B] [W]
// [T] [C] [L] [H] [Q] [J] [B] [T] [N]
// [G] [G] [C] [J] [P] [P] [Z] [R] [H]
//  1   2   3   4   5   6   7   8   9

fn main() {
    let contents = fs::read_to_string("data/input.txt").expect("");

    let arrangements: Vec<&str> = contents.split('\n').collect();

    let mut crates: [Vec<char>; 9] = [
        vec!['G', 'T', 'R', 'W'],
        vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'],
        vec!['C', 'L', 'T', 'S', 'G', 'M'],
        vec!['J', 'H', 'D', 'M', 'W', 'R', 'F'],
        vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'],
        vec!['P', 'J', 'D', 'N', 'F', 'M', 'S'],
        vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'],
        vec!['R', 'T', 'B'],
        vec!['H', 'N', 'W', 'L', 'C'],
    ];

    for arrangement in arrangements {
        let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let cap = re.captures(arrangement).unwrap();
        println!("{} {} {}", &cap[1], &cap[2], &cap[3]);

        let quantity: u32 = cap[1].parse().unwrap();
        let origin = cap[2].parse::<u32>().unwrap() - 1;
        let destination = cap[3].parse::<u32>().unwrap() - 1;

        let destination_len = crates[destination as usize].len();
        for _ in 0..quantity {
            let c = crates[origin as usize].pop().unwrap();
            crates[destination as usize].insert(destination_len, c);
        }

    }

    let mut end_of_the_stacks = "".to_string();
    for c in crates {
        end_of_the_stacks.push(c[c.len() - 1])
    }

    println!("{end_of_the_stacks}")
}
