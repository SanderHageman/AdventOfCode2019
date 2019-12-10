use math::round;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();

    let mut p1_result = 0;
    let mut p2_result = 0;

    for line in input.lock().lines() {
        let line_str = line.unwrap_or_default();

        if line_str == "" {
            break;
        }

        let mass = line_str.parse::<i32>().unwrap_or_default();
        let mut result_mass = get_required_fuel(mass);

        p1_result += result_mass.mass;

        while result_mass.apply {
            p2_result += result_mass.mass;
            result_mass = get_required_fuel(result_mass.mass);
        }
    }

    println!("Result1: {:?}", p1_result);
    println!("Result2: {:?}", p2_result);
}

fn get_required_fuel(mass: i32) -> ResultMass {
    let floored_value = round::floor(mass as f64 / 3f64, 0) as i32;
    let result = floored_value - 2;

    ResultMass {
        apply: result > 0,
        mass: result,
    }
}

#[derive(Default, Debug)]
struct ResultMass {
    apply: bool,
    mass: i32,
}
