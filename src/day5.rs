use super::intcode_computer;

pub fn day(input: std::string::String) {
    let input_vec = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    println!(
        "Day 5 Result1: {:?}",
        intcode_computer::Computer::simple(vec![1, 0], &input_vec)
    );
    println!(
        "Day 5 Result2: {:?}",
        intcode_computer::Computer::simple(vec![5, 0], &input_vec)
    );
}
