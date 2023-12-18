use std::collections::HashMap;
use itertools::{Itertools};

const NUM_OF_ITERATIONS: u32 = 1;

// function to check if a vector is a subset of another vector with order
fn is_subset<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> bool {
  if a.len() > b.len() {
    return false;
  }
  for i in 0..b.len() {
    if a.len() == 0 {
      return true;
    }
    if a[0] == b[i] {
      a.remove(0);
    }
  }
  a.len() == 0
}

pub fn part2(lines: Vec<&str>) -> usize {
  let mut total = 0;
  let mut all_possible_arrangements = HashMap::new();

  for line in lines {
    let groups: Vec<&str> = line.split(" ").collect();
    let init_springs = groups[0];
    let init_damaged: Vec<usize> = groups[1].split(",").map(|n| n.parse().unwrap()).collect();

    let mut springs_vec = vec![];
    for _ in 0..NUM_OF_ITERATIONS {
      springs_vec.push(init_springs);
    }
    let springs_str = springs_vec.join("?");
    let springs: Vec<String> = springs_str.split(".").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();

    let mut damaged = vec![];
    for _ in 0..NUM_OF_ITERATIONS {
      damaged.extend(init_damaged.clone());
    }

    // find all subset damaged arrangements
    for spring in springs.iter() {
      if all_possible_arrangements.contains_key(spring) {
        continue;
      }
      all_possible_arrangements.insert(spring.clone(), vec![]);
      let mut unknowns = vec![];
      for (s, spring) in spring.chars().enumerate() {
        if spring == '?' {
          unknowns.push(s);
        }
      }

      let mut arrangement: Vec<char> = spring.chars().collect();
      let possible_arrangements = match unknowns.len() {
        0 => {(0..arrangement.len()).map(|_| "#".chars()).multi_cartesian_product()},
        _ => (0..unknowns.len()).map(|_| ".#".chars()).multi_cartesian_product()
      };

      for possible_arrangement in possible_arrangements {
        for (u, unknown) in unknowns.iter().enumerate() {
          arrangement[*unknown] = possible_arrangement[u];
        }
        let damaged_arrangement = arrangement.clone().into_iter().collect::<String>().split('.')
          .filter(|s| s.len() > 0).map(|s| s.len() as usize).collect::<Vec<usize>>();

        if is_subset(damaged_arrangement.clone(), damaged.clone()) {
          all_possible_arrangements.get_mut(spring).unwrap().push(damaged_arrangement);
        }
      }
    }

    // combine all damaged_arrangements in spring order
    let mut all_combined_damaged: Vec<Vec<usize>> = vec![];
    for spring in springs.iter() {
      if all_combined_damaged.len() == 0 {
        all_combined_damaged.extend(all_possible_arrangements.get(spring).unwrap().clone());
        continue;
      }
      let mut combined_damaged = vec![];
      for combined in all_combined_damaged.iter() {
        for possible in all_possible_arrangements.get(spring).unwrap().iter() {
          let mut combined = combined.clone();
          combined.extend(possible.clone());
          combined_damaged.push(combined.clone());
        }
      }

      all_combined_damaged.extend(combined_damaged);

    }
    let arrangements = all_combined_damaged.iter().filter(|combined| *combined == &damaged).count();

    total += arrangements;
  }

  total
}