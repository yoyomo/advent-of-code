use itertools::Itertools;

const NUM_OF_ITERATIONS: u32 = 1;

pub fn part2(lines: Vec<&str>) -> u64 {
  let mut total = 0;
  for line in lines {
    let groups: Vec<&str> = line.split(" ").collect();
    let init_springs = groups[0];
    let init_damaged: Vec<u64> = groups[1].split(",").map(|n| n.parse().unwrap()).collect();

    let mut springs_vec = vec![];
    for _ in 0..NUM_OF_ITERATIONS {
      springs_vec.push(init_springs);
    }
    let springs = springs_vec.join("?");
    let mut damaged = vec![];
    for _ in 0..NUM_OF_ITERATIONS {
      damaged.extend(init_damaged.clone());
    }

    let mut unknowns = vec![];
    for (s, spring) in springs.chars().enumerate() {
      if spring == '?' {
        unknowns.push(s);
      }
    }
    let mut arrangements = 0;

    let mut arrangement: Vec<char> = springs.chars().collect();
    let possible_arrangements = (0..unknowns.len()).map(|_| ".#".chars())
      .multi_cartesian_product();

    for possible_arrangement in possible_arrangements {
      for (u, unknown) in unknowns.iter().enumerate() {
        arrangement[*unknown] = possible_arrangement[u];
      }
      let damaged_arrangement = arrangement.clone().into_iter().collect::<String>().split('.').filter(|s| s.len() > 0).map(|s| s.len() as u64).collect::<Vec<u64>>();

      if damaged_arrangement == damaged {
        arrangements += 1;
      }
    }

    total += arrangements;
  }

  total
}