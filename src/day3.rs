use cgmath::Vector2;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day3(input: std::string::String) {
    let mut paths: Vec<HashSet<Vector2<i32>>> = vec![];

    for line in input.lines() {
        let line = line
            .trim()
            .split(",")
            .map(|x| x.parse::<InputInstruction>().unwrap())
            .collect::<Vec<_>>();

        let mut path: HashSet<Vector2<i32>> = HashSet::new();
        let mut pos = Vector2::new(0, 0);
        for input in line {
            input.add_step(&mut pos, &mut path);
        }
        paths.push(path);
    }

    let intersections: HashSet<_> = paths[0].intersection(&paths[1]).collect();
    let mut result_one = i32::max_value();

    for pos in intersections {
        let dist = pos.x.abs() + pos.y.abs();
        if dist < result_one {
            result_one = dist;
        }
    }

    println!("Day 3 Result1: {:?}", result_one);
    println!("Day 3 Result2: {:?}", 0);
}

#[derive(Debug)]
struct InputInstruction {
    direction: Vector2<i32>,
    distance: i32,
}

impl InputInstruction {
    fn add_step(
        &self,
        current_position: &mut Vector2<i32>,
        result_set: &mut HashSet<Vector2<i32>>,
    ) {
        for _x in 0..self.distance {
            *current_position += self.direction;
            result_set.insert(current_position.clone());
        }
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
