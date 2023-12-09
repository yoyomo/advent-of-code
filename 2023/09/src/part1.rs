pub fn part1(lines: Vec<&str>) -> i32 {

    let mut histories: Vec<Vec<i32>> = vec![];
    for line in lines {
        histories.push(line.split(" ").map(|n| n.parse().unwrap()).collect())
    }

    let mut values: Vec<i32> = vec![0; histories.len()];

    for history in histories {
        let mut sequences: Vec<Vec<i32>> = vec![];
        sequences.push(history.clone());
        let mut sequence = history;

        while sequence.iter().any(|num| *num != 0) {

            let mut next_sequence = vec![];
            for i in 0..sequence.len()-1 {
                next_sequence.push(sequence[i+1] - sequence[i]);
            }
            sequences.push(next_sequence.clone());

            sequence = next_sequence;

        }

        //calculate values
        let mut value = 0;

        for s in (1..sequences.len()).rev() {
            value = sequences[s].last().unwrap() + sequences[s-1].last().unwrap();
            sequences[s-1].push(value);
        }

        values.push(value);
    }


    return values.iter().sum();
}
