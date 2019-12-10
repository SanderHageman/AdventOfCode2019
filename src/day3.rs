use cgmath::*;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day3(input: std::string::String) {
    let mut paths: Vec<Vec<Vector2<i32>>>;

    for line in input.lines() {
        let line = line
            .trim()
            .split(",")
            .map(|x| x.parse::<InputInstruction>().unwrap_or_default())
            .collect::<Vec<_>>();

        println!("{:?}", line);
    }

    println!("Day 3 Result1: {:?}", 0);
    println!("Day 3 Result2: {:?}", 0);
}

#[derive(Default, Debug)]
struct InputInstruction {
    direction: char,
    distance: i32,
}

impl FromStr for InputInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().nth(0).unwrap_or_default();

        let num = s.chars().filter(|x| x.is_digit(10)).collect::<String>();
        let dist = num.parse::<i32>().unwrap_or_default();

        Ok(InputInstruction {
            direction: dir,
            distance: dist,
        })
    }
}
