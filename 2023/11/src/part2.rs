pub fn part2(lines: Vec<&str>) -> u64 {
    let mut galaxies: Vec<[isize; 2]> = vec![];
    for (l, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push([l as isize, c as isize]);
            }
        }
    }

    let mut no_galaxies: [Vec<isize>;2] = [vec![], vec![]]; //rows and columns
    for (l, line) in lines.iter().enumerate() {
        if line.chars().all(|char| char == '.') {
            no_galaxies[0].push(l as isize)
        }
    }

    for c in 0..lines[0].len() {
        if lines.iter().all(|line| line.chars().nth(c).unwrap() == '.') {
            no_galaxies[1].push(c as isize);
        }
    }

    // find pairs
    let mut galaxy_pairs = vec![];
    for i in 0..galaxies.len() {
        for j in 0..galaxies.len() {
            if i != j && !galaxy_pairs.contains(&(j, i)) {
                galaxy_pairs.push((i, j))
            }
        }
    }

    // find shortest paths between all pairs
    let mut all_steps: u64 = 0;
    for galaxy_pair in galaxy_pairs {
        let mut steps= 0;

        let mut current_position = galaxies[galaxy_pair.0];
        let destination_position = galaxies[galaxy_pair.1];
        let mut is_horizontal = current_position[1] != destination_position[1];
        while current_position[0] != destination_position[0] ||
            current_position[1] != destination_position[1] {
            let pos = if is_horizontal { 1 } else { 0 };
            let switch_pos = if is_horizontal { 0 } else { 1 };

            let mut diff: i32 = if current_position[pos] < destination_position[pos] {
                1
            } else if current_position[pos] > destination_position[pos] {
                -1
            } else {
                0
            };

            current_position[pos] += diff as isize;

            diff *= if no_galaxies[pos].contains(&current_position[pos]) {1000000} else {1};

            steps += diff.abs() as u64;

            is_horizontal = !is_horizontal && current_position[switch_pos] != destination_position[switch_pos];
        }

        all_steps += steps;
    }

    return all_steps;
}
