use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap_or_default())
        .collect::<Vec<_>>();

    let result_one = Computer::simple(vec![0], &input_vec);
    let result_two = 0;

    println!("Day 9 Result1: {:?}", input_vec);
    println!("Day 9 Result2: {:?}", result_two);
}
