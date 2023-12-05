pub fn part1(lines: Vec<&str>) -> u32 {
    let mut parts = 0;

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
                let mut is_part = false;
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
                        if !symbol.is_digit(10) && symbol != '.' {
                            is_part = true
                        }
                    }
                }
                let left = line.chars().nth(start).unwrap();
                if !left.is_digit(10) && left != '.' {
                    is_part = true
                }
                let right = line.chars().nth(end).unwrap();
                if !right.is_digit(10) && right != '.' {
                    is_part = true
                }
                if l < lines.len() - 1 {
                    for surround in start..end + 1 {
                        let symbol = lines.get(l + 1).unwrap().chars().nth(surround).unwrap();
                        if !symbol.is_digit(10) && symbol != '.' {
                            is_part = true
                        }
                    }
                }
                if is_part {
                    parts += current_number;
                }
                from = usize::MAX;
                current_number = 0;
            }
            c += 1;
        }

        l += 1;
    }

    return parts;
}
