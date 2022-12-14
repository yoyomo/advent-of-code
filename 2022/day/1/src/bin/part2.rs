use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("");

    println!("{contents}");

    let calories: Vec<&str> = contents.split('\n').collect();

    const TOP_NUM: usize = 3;
    let mut top_three_calories: [u32; TOP_NUM] = [0,0,0];
    let mut top_three_total = 0;

    let mut num_of_calories: u32 = 0;
    for calorie in calories {

        match calorie.parse::<u32>() {
            Ok(c) => num_of_calories += c,
            Err(_) => {
                top_three_total = 0;
                for (i, top_three) in top_three_calories.iter().enumerate() {
                    top_three_total += top_three;

                    if num_of_calories > *top_three {
                        let mut j = TOP_NUM - 1;
                        while j > i {
                            top_three_calories[j] = top_three_calories[j-1];
                            j -= 1;
                        }
                        top_three_calories[i] = num_of_calories;
                        break;
                    }
                }
                num_of_calories = 0;
            },
        }

    }

    println!("Top three: {:?}", top_three_calories);
    println!("Total num: {top_three_total}")

}
