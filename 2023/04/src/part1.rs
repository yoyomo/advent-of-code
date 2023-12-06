use regex::Regex;

pub fn part1(lines: Vec<&str>) -> i32 {

    let re = Regex::new(r"Card\s+[0-9]+:\s(.*)").unwrap();

    let mut points = 0;

    for line in lines {
        let groups = re.captures(line).unwrap();
        let numbers: Vec<_> = groups.get(1).unwrap().as_str().split(" | ").collect();
        let winning_numbers: Vec<_> = numbers[0].split(" ").collect();
        let obtained_numbers: Vec<_> = numbers[1].split(" ").collect();

        let mut point = 0;
        for obtained_number in obtained_numbers {
            if obtained_number.parse::<i32>().is_ok() && winning_numbers.contains(&obtained_number){
                if point == 0 {
                    point = 1;
                } else {
                    point *= 2;
                }
            }
        }


        points += point;

    }

    return points
}
