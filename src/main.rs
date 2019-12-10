use math::round;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();

    let mut result = 0;

    for line in input.lock().lines() {
        let line_str = line.unwrap_or_default();
        let mass = line_str.parse::<i32>().unwrap_or_default();
        result += get_required_fuel(mass)
    }

    println!("Result: {:?}", result);
}

fn get_required_fuel(mass: i32) -> i32 {
    let t = mass as f64 / 3f64;
    let b = round::floor(t, 0) as i32;
    b - 2
}
