use super::intcode_computer::*;

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = input_vec;
    let result_two = 0;

    println!("Day 13 Result1: {:?}", result_one);
    println!("Day 13 Result2: {:?}", result_two);
}
