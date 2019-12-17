use super::intcode_computer::*;
use cgmath::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut robot = PaintRobot::new(Computer::new(vec![], &input_vec, 1500));

    let mut visited_tiles = HashSet::<GridNode>::new();

    while !robot.computer.stop {
        let mut node = match visited_tiles.take(&GridNode::new(robot.pos)) {
            Some(val) => val,
            None => GridNode::new(robot.pos),
        };

        let target_color = robot.do_move(node.color);

        node.color = target_color;
        visited_tiles.insert(node);
    }

    let result_one = visited_tiles.len();
    let result_two = 0;

    println!("Day 10 Result1: {:?}", result_one);
    println!("Day 10 Result2: {:?}", result_two);
}

impl PaintRobot {
    fn new(computer: Computer) -> PaintRobot {
        PaintRobot {
            computer: computer,
            pos: Vector2::new(0, 0),
            heading: 0,
        }
    }

    fn do_move(&mut self, input: i64) -> i64 {
        self.computer.add_input(input);

        let result = self.computer.compute_til_output();
        let dir_change = self.computer.compute_til_output();

        self.change_heading(dir_change as usize);
        self.update_position();

        result
    }

    fn change_heading(&mut self, dir_change: usize) {
        let change: i32 = match dir_change {
            0 => -1,
            1 => 1,
            _ => panic!("Unknown direction change encountered {}", dir_change),
        };

        self.heading = PaintRobot::modulo(self.heading as i32 + change, 4) as usize;
    }

    fn update_position(&mut self) {
        let change = match self.heading {
            0 => Vector2::new(0, -1), // up
            1 => Vector2::new(1, 0),  // right
            2 => Vector2::new(0, 1),  // down
            3 => Vector2::new(-1, 0), // left
            _ => panic!("Unknown heading encountered {}", self.heading),
        };

        self.pos += change;
    }

    fn modulo(a: i32, b: i32) -> i32 {
        let remainder = a % b;

        if remainder < 0 {
            remainder + b
        } else {
            remainder
        }
    }
}

#[derive(Debug)]
struct PaintRobot {
    computer: Computer,
    pos: Vector2<i32>,
    heading: usize,
}

#[derive(Debug, Eq)]
struct GridNode {
    pos: Vector2<i32>,
    color: i64,
}

impl GridNode {
    fn new(pos: Vector2<i32>) -> GridNode {
        GridNode { pos: pos, color: 0 }
    }
}

impl Hash for GridNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for GridNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
