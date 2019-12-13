use super::intcode_computer;

pub fn day(input: std::string::String) {
    let input_vec = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    let result_one = 0;
    let result_two = 0;

    println!("Day 7 Result1: {:?}", result_one);
    println!("Day 7 Result2: {:?}", result_two);
}
