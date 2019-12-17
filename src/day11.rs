use super::intcode_computer::*;
use cgmath::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = get_part_one(&input_vec);
    let (image_width, color_vec) = get_part_two(&input_vec);

    println!("Day 11 Result1: {:?}", result_one);
    println!("Day 11 Result2");
    draw_image(image_width, &color_vec);
}

fn get_part_one(input_vec: &Vec<i64>) -> usize {
    let mut robot = PaintRobot::new(Computer::new(vec![], &input_vec, 1500));
    let mut visited_nodes = HashSet::<GridNode>::new();

    robot.paint(&mut visited_nodes);

    visited_nodes.len()
}

fn get_part_two(input_vec: &Vec<i64>) -> (usize, Vec<u32>) {
    let mut robot = PaintRobot::new(Computer::new(vec![], &input_vec, 1500));
    let mut visited_nodes = HashSet::<GridNode>::new();

    visited_nodes.insert(GridNode {
        pos: Vector2::new(0, 0),
        color: 1,
    });

    robot.paint(&mut visited_nodes);

    let mut minx = i32::max_value();
    let mut miny = i32::max_value();

    let mut maxx = i32::min_value();
    let mut maxy = i32::min_value();

    for node in &visited_nodes {
        let x = node.pos.x;
        let y = node.pos.y;

        minx = minx.min(x);
        miny = miny.min(y);

        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    // grid is nice enough to start 0,0 so we don't need an offset

    let w = maxx as usize;
    let h = maxy as usize + 1;
    let size = w * h;

    let mut result: Vec<u32> = Vec::new();

    for i in 0..size {
        let (x, y) = get_xy(i, w);
        let pos = Vector2::new(x, y);

        let color = match visited_nodes.get(&GridNode::new(pos)) {
            Some(val) => val.color,
            None => 0,
        };

        result.push(color as u32);
    }

    (w as usize, result)
}

fn draw_image(width: usize, image: &Vec<u32>) {
    for i in 0..image.len() {
        if i % width == 0 && i != 0 {
            print!("\n");
        }

        let put = match image[i] {
            0 => '░',
            1 => '█',
            _ => panic!("pixel out of range"),
        };

        print!("{}", put);
    }
    print!("\n");
}

fn get_xy(index: usize, width: usize) -> (i32, i32) {
    let x = index % width;
    let y = index / width;
    (x as i32, y as i32)
}

impl PaintRobot {
    fn new(computer: Computer) -> PaintRobot {
        PaintRobot {
            computer: computer,
            pos: Vector2::new(0, 0),
            heading: 0,
        }
    }

    fn paint(&mut self, tiles: &mut HashSet<GridNode>) {
        while !self.computer.stop {
            let mut node = match tiles.take(&GridNode::new(self.pos)) {
                Some(val) => val,
                None => GridNode::new(self.pos),
            };

            let target_color = self.do_move(node.color);

            node.color = target_color;
            tiles.insert(node);
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
