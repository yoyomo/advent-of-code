fn is_within_bounds(position: [usize; 2], map: &Vec<Vec<char>>) -> bool {
    position[0] < map.len() && position[1] < map.first().unwrap().len()
}

fn flip_direction(direction: char) -> char {
    match direction {
        'N' => {'S'},
        'S' => {'N'},
        'W' => {'E'},
        'E' => {'W'},
        _ => {panic!()}
    }
}

fn is_connecting_pipe(position: [usize; 2], map: &Vec<Vec<char>>, direction: char) -> bool {
    let pipe = map[position[0]][position[1]];
    let pipe_direction = flip_direction(direction);
    match pipe {
        '|' => { pipe_direction == 'N' || pipe_direction == 'S' }
        '-' => { pipe_direction == 'E' || pipe_direction == 'W' }
        'L' => { pipe_direction == 'N' || pipe_direction == 'E' }
        'J' => { pipe_direction == 'N' || pipe_direction == 'W' }
        '7' => { pipe_direction == 'S' || pipe_direction == 'W' }
        'F' => { pipe_direction == 'S' || pipe_direction == 'E' }
        '.' => { false }
        'S' => { true }
        _ => { false }
    }
}

fn are_pipes_connected(current_position: [usize;2], next_position:[usize;2], map: &Vec<Vec<char>>, direction: char) -> bool {
    if !is_within_bounds(next_position, &map) {
        return false;
    }
    is_connecting_pipe(current_position, map, flip_direction(direction)) &&
        is_connecting_pipe(next_position, map, direction)

}

pub fn part1(lines: Vec<&str>) -> usize {

    // find S
    let mut map: Vec<Vec<char>> = vec![];
    let mut starting_point = [0, 0];
    for (l, line) in lines.iter().enumerate() {
        map.push(line.chars().collect());
        if line.contains('S') {
            starting_point = [l, line.chars().position(|s| s == 'S').unwrap()];
        }
    }

    // find main loop
    let mut main_loop: Vec<[usize; 2]> = vec![];
    let mut current_position = starting_point;
    let mut previous_position = current_position; // to avoid going back
    main_loop.push(starting_point);
    'main: loop {
        for direction in ['N','W','S','E'] {
            let next_position = match direction {
                'N' => if current_position[0] == 0 {previous_position} else { [current_position[0] - 1, current_position[1]]},
                'W' => if current_position[1] == 0 {previous_position} else { [current_position[0], current_position[1] - 1]},
                'S' => [current_position[0] + 1, current_position[1]],
                'E' => [current_position[0], current_position[1] + 1],
                _ => {panic!()}
            };
            if next_position == previous_position {
                continue
            }

            if are_pipes_connected(current_position, next_position, &map, direction) {
                if next_position == starting_point {
                    break 'main;
                }
                previous_position = current_position;
                current_position = next_position;
                main_loop.push(next_position);
                continue 'main;
            }
        }
    }

    //calculate farthest pipe
    return main_loop.len() / 2;
}
