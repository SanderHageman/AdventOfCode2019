use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    day1::day(get_input(1));
    println!("");
    day2::day(get_input(2));
    println!("");
    day3::day(get_input(3));
    println!("");
    day4::day(get_input(4));
}

fn get_input(day: usize) -> String {
    let file_path = format!("input/day{}.txt", day);
    fs::read_to_string(file_path)
        .expect("Something went wrong!")
        .trim()
        .to_owned()
}
