use crate::part1::{find_main_loop, get_map_and_starting_point, is_connecting_pipe};

pub fn part2(lines: Vec<&str>) -> usize {

    let (starting_point, map) = get_map_and_starting_point(lines);
    let main_loop = find_main_loop(starting_point, &map);

    let mut sum = 0;
    for r in 0..map.len() {
        let mut inside = false;
        for c in 0..map[r].len() {
            if !main_loop.contains(&[r, c]) {
                if inside {
                    sum += 1;
                }
            } else if is_connecting_pipe([r,c], &map, 'N') {
                inside = !inside
            }
        }
    }

    return sum;
}
