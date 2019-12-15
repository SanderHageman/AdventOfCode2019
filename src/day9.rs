use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut computer = Computer::new(vec![1], &input_vec, 1100);

    while !computer.stop {
        println!("{}", computer.compute_til_output());
    }

    let result_one = computer.output.unwrap();
    let result_two = 0;

    println!("Day 9 Result1: {:?}", result_one);
    println!("Day 9 Result2: {:?}", result_two);
}
