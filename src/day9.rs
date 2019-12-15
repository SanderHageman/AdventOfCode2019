use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut computer_one = Computer::new(vec![1], &input_vec, 1100);
    let mut computer_two = Computer::new(vec![2], &input_vec, 1100);

    let result_one = computer_one.compute();
    let result_two = computer_two.compute();

    println!("Day 9 Result1: {:?}", result_one);
    println!("Day 9 Result2: {:?}", result_two);
}
