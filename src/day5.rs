use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = Computer::parse_input(input);

    println!(
        "Day 5 Result1: {:?}",
        Computer::simple(vec![1, 0], &input_vec)
    );
    println!(
        "Day 5 Result2: {:?}",
        Computer::simple(vec![5, 0], &input_vec)
    );
}
