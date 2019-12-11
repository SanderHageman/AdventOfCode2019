use cgmath::*;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day3(input: std::string::String) {
    let mut paths: Vec<Vec<InputInstruction>> = vec![];

    // hashset and use intersection

    for line in input.lines() {
        let line = line
            .trim()
            .split(",")
            .map(|x| x.parse::<InputInstruction>().unwrap())
            .collect::<Vec<_>>();

        paths.push(line);
    }

    println!("Day 3 Result1: {:?}", paths);
    println!("Day 3 Result2: {:?}", 0);
}

#[derive(Debug)]
struct InputInstruction {
    direction: Vector2<i32>,
    distance: i32,
}

impl InputInstruction {
    fn step(&self) -> Vector2<i32> {
        self.direction * self.distance
    }
}

impl FromStr for InputInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir_char = s.chars().nth(0).unwrap_or_default();

        let dir = match dir_char {
            'U' => Vector2::new(0, 1),
            'D' => Vector2::new(0, -1),
            'L' => Vector2::new(-1, 0),
            'R' => Vector2::new(1, 0),
            _ => panic!("Unknown direction {:?}", dir_char),
        };

        let num = s.chars().filter(|x| x.is_digit(10)).collect::<String>();
        let dist = num.parse::<i32>().unwrap_or_default();

        Ok(InputInstruction {
            direction: dir,
            distance: dist,
        })
    }
}
