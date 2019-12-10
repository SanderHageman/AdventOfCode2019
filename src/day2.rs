use std::fs;

pub fn day2() {
    let input = fs::read_to_string("input/day2.txt").expect("Something went wrong!");

    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    println!("Day 2 Result1: {:?}", run_intcode(12, 2, &input_vec));
    println!("Day 2 Result2: {:?}", part_two(&input_vec));
}

fn part_two(input_vec: &Vec<i32>) -> i32 {
    let part_two_target = 19690720;

    for x in 0..99 {
        for y in 0..99 {
            if run_intcode(x, y, input_vec) == part_two_target {
                return 100 * x + y;
            }
        }
    }

    panic!("No value found")
}

fn run_intcode(noun: i32, verb: i32, input_vec: &Vec<i32>) -> i32 {
    let mut result_vec = input_vec.to_vec();

    result_vec[1] = noun;
    result_vec[2] = verb;

    let mut i = 0;
    while i < result_vec.len() {
        let opcode = result_vec[i];
        if opcode == 99 {
            break;
        };

        let parameter_one_index = result_vec[i + 1] as usize;
        let parameter_two_index = result_vec[i + 2] as usize;
        let parameter_result_index = result_vec[i + 3] as usize;

        let parameter_one = result_vec[parameter_one_index];
        let parameter_two = result_vec[parameter_two_index];

        result_vec[parameter_result_index] = match opcode {
            1 => parameter_one + parameter_two,
            2 => parameter_one * parameter_two,
            _ => panic!("uncovered opcode found {:?} at {:?}", opcode, i),
        };

        i += 4;
    }

    result_vec[0]
}
