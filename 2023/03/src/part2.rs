pub fn part2(lines: Vec<&str>) -> u32 {
    let mut gears: Vec<Vec<(bool, u32)>> = vec![vec![(false, 0); lines.get(0).unwrap().chars().count()]; lines.len()];
    let mut l = 0;
    while l < lines.len() {
        let line = lines.get(l).unwrap();

        //grab number and range
        let mut c = 0;
        let mut current_number = 0;
        let mut from = usize::MAX;
        while c <= line.chars().count() {
            let char = line.chars().nth(c);
            if char.is_some() && char.unwrap().is_digit(10) {
                if c < from {
                    from = c;
                }
                current_number *= 10;
                current_number += char.unwrap().to_digit(10).unwrap();
            } else if current_number > 0 && (char.is_none() || !char.unwrap().is_digit(10)) {
                let mut start = from;
                if from > 0 {
                    start = from - 1;
                }
                let mut end = c;
                if c == line.chars().count() {
                    end = c - 1;
                }

                if l > 0 {
                    for surround in start..end + 1 {
                        let symbol = lines.get(l - 1).unwrap().chars().nth(surround).unwrap();
                        if symbol == '*' {
                            if gears[l - 1][surround].1 == 0 {
                                gears[l - 1][surround].1 = current_number;
                            } else {
                                gears[l - 1][surround].0 = true;
                                gears[l - 1][surround].1 *= current_number;
                            }
                        }
                    }
                }
                let left = line.chars().nth(start).unwrap();
                if left == '*' {
                    if gears[l][start].1 == 0 {
                        gears[l][start].1 = current_number;
                    } else {
                        gears[l][start].0 = true;
                        gears[l][start].1 *= current_number;
                    }
                }
                let right = line.chars().nth(end).unwrap();
                if right == '*' {
                    if gears[l][end].1 == 0 {
                        gears[l][end].1 = current_number;
                    } else {
                        gears[l][end].0 = true;
                        gears[l][end].1 *= current_number;
                    }
                }
                if l < lines.len() - 1 {
                    for surround in start..end + 1 {
                        let symbol = lines.get(l + 1).unwrap().chars().nth(surround).unwrap();
                        if symbol == '*' {
                            if gears[l + 1][surround].1 == 0 {
                                gears[l + 1][surround].1 = current_number;
                            } else {
                                gears[l + 1][surround].0 = true;
                                gears[l + 1][surround].1 *= current_number;
                            }
                        }
                    }
                }
                from = usize::MAX;
                current_number = 0;
            }
            c += 1;
        }

        l += 1;
    }

    let mut parts = 0;

    for line in gears {
        for gear in line {
            if gear.0 {
                parts += gear.1;
            }
        }
    }

    return parts;
}
