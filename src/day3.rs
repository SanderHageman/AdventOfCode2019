use cgmath::Vector2;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day3(input: std::string::String) {
    let mut paths: Vec<HashSet<Step>> = vec![];

    for line in input.lines() {
        let line = line
            .trim()
            .split(",")
            .map(|x| x.parse::<InputInstruction>().unwrap())
            .collect::<Vec<_>>();

        let mut path: HashSet<Step> = HashSet::new();
        let mut pos = Step {
            step_index: 0,
            position: Vector2::new(0, 0),
        };

        for input in line {
            input.add_step(&mut pos, &mut path);
        }
        paths.push(path);
    }

    let intersections: HashSet<_> = paths[0].intersection(&paths[1]).collect();

    let mut result_one = i32::max_value();
    let mut result_two = i32::max_value();

    for step in intersections {
        let dist_one = step.position.x.abs() + step.position.y.abs();
        if dist_one < result_one {
            result_one = dist_one;
        }

        let a = paths[0].get(&step).unwrap();
        let b = paths[1].get(&step).unwrap();

        let dist_two = (*a).step_index + (*b).step_index;
        if dist_two < result_two {
            result_two = dist_two;
        }
    }

    println!("Day 3 Result1: {:?}", result_one);
    println!("Day 3 Result2: {:?}", result_two);
}

#[derive(Debug)]
struct InputInstruction {
    direction: Vector2<i32>,
    distance: i32,
}

impl InputInstruction {
    fn add_step(&self, current_position: &mut Step, result_set: &mut HashSet<Step>) {
        for _x in 0..self.distance {
            current_position.step_index += 1;
            current_position.position += self.direction;
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

#[derive(Debug, Clone, Eq)]
struct Step {
    step_index: i32,
    position: Vector2<i32>,
}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
