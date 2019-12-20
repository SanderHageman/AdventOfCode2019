pub fn day(input: String) {
    let result_one = get_part_one(&input);
    let result_two = 0;

    println!("Day 16 Result1: {:?}", result_one);
    println!("Day 16 Result2: {:?}", result_two);
}

fn get_part_one(input: &str) -> Vec<i32> {
    let input_signal = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();

    let mut output = phase(&input_signal);
    for _ in 1..100 {
        output = phase(&output);
    }

    output.truncate(8);
    output
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
