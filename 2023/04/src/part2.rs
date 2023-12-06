use regex::Regex;

pub fn part2(lines: Vec<&str>) -> usize {

    let re = Regex::new(r"Card\s+[0-9]+:\s(.*)").unwrap();

    let mut points: Vec<(usize,usize)> = vec![(1,0); lines.len()];

    for (l, line) in lines.iter().enumerate() {
        let groups = re.captures(line).unwrap();
        let numbers: Vec<_> = groups.get(1).unwrap().as_str().split(" | ").collect();
        let winning_numbers: Vec<_> = numbers[0].split(" ").collect();
        let obtained_numbers: Vec<_> = numbers[1].split(" ").collect();

        for obtained_number in obtained_numbers {
            if obtained_number.parse::<usize>().is_ok() && winning_numbers.contains(&obtained_number){
                points[l].1 += 1;
            }
        }

    }

    for p in 0..points.len() {
        let point = points[p];
        for n in p+1..(p + point.1 + 1) {
            points[n].0 += point.0
        }
    }

    let mut num_of_scratchpads = 0;
    for point in points {
        num_of_scratchpads += point.0
    }

    return num_of_scratchpads
}
