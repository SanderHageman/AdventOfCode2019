pub fn day(input: String) {
    let input_signal = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();

    let result_one = get_part_one(&input_signal);
    let result_two = get_part_two(&input_signal);

    println!("Day 16 Result1: {:?}", result_one);
    println!("Day 16 Result2: {:?}", result_two);
}

fn get_part_one(input_signal: &Vec<i32>) -> Vec<i32> {
    let mut output: Vec<Vec<i32>> = vec![phase(input_signal)];
    for _ in 1..100 {
        output.push(phase(output.last().unwrap()));
    }

    Vec::from(&output.last().unwrap()[0..8])
}

fn get_part_two(input_signal: &Vec<i32>) -> Vec<i32> {
    const INPUT_OFFSET: usize = 5979633;

    let mut signal = Vec::<i32>::new();

    for _ in 0..10000 {
        signal.extend(input_signal.iter())
    }

    let mut current_signal = Vec::from(&signal[INPUT_OFFSET..signal.len()]);
    let mut next_signal = current_signal.clone();
    let siglen = current_signal.len();

    for _ in 0..100 {
        let index = siglen - 2;
        next_signal[index + 1] = current_signal[index + 1];

        for i in (0..index).rev() {
            next_signal[i] = ((next_signal[i + 1] + current_signal[i]) % 10).abs();
        }

        current_signal = next_signal.clone();
    }

    Vec::from(&current_signal[0..8])
}

fn phase(input: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for i in 0..input.len() {
        result.push(calc_num(i, input));
    }

    result
}

fn calc_num(index: usize, input: &Vec<i32>) -> i32 {
    let mut result = 0;
    let pattern = get_pattern(index, input.len());

    for (i, num) in input.iter().enumerate() {
        result += num * pattern[i];
    }

    (result % 10).abs()
}

fn get_pattern(index: usize, len: usize) -> Vec<i32> {
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
    let mut result = vec![];

    let repeat = index;

    let mut i = 0;
    let mut rep_counter = repeat;

    while result.len() <= len {
        result.push(BASE_PATTERN[i % 4]);

        if rep_counter == 0 {
            i += 1;
            rep_counter = repeat;
        } else {
            rep_counter -= 1;
        }
    }

    result.remove(0);
    assert_eq!(result.len(), len);
    result
}
