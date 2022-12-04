use std::fs;

fn main() {
    let file_path = "src/input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    println!("With text:\n{contents}");

    let calories: Vec<&str> = contents.split('\n').collect();

    let mut max_num_of_calories: u32 = 0;
    let mut num_of_calories: u32 = 0;
    for calorie in calories {

        match calorie.parse::<u32>() {
            Ok(c) => num_of_calories += c,
            Err(_) => num_of_calories = 0,
        }

        if num_of_calories > max_num_of_calories {
            max_num_of_calories = num_of_calories;
        }
        
    }

    println!("Max num: {max_num_of_calories}")


}
