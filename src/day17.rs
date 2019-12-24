use super::intcode_computer::*;
use cgmath::*;
use std::collections::HashMap;

type Vec2 = Vector2<i64>;

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let mut map = HashMap::new();

    let result_one = get_part_one(&input_vec, &mut map);
    let result_two = get_part_two(&input_vec, map);

    println!("Day 17 Result1: {:?}", result_one);
    println!("Day 17 Result2: {:?}", result_two);
}

fn get_part_one(input: &Vec<i64>, map: &mut HashMap<Vec2, u8>) -> i64 {
    let mut computer = Computer::new(vec![], &input, 10000);

    let mut pos = Vec2::new(0, 0);

    while !computer.stop {
        let result = computer.compute_til_output();
        map.insert(pos, result as u8);

        pos += if result == 10 {
            Vec2::new(-pos.x, 1)
        } else {
            Vec2::new(1, 0)
        };
    }

    // draw(&map);
    let intersections = get_intersections(&map);
    let alignment = intersections.iter().map(|x| x.x * x.y).collect::<Vec<_>>();

    let mut result = 0;
    for val in alignment {
        result += val;
    }

    result
}

fn get_part_two(input: &Vec<i64>, map: HashMap<Vec2, u8>) -> i64 {
    let mut input =input.clone();
    input[0] = 2;
    let mut robot = Robot::new(Computer::new(vec![], &input, 10000));

    // find the path
    let mut path = Vec::new();
    let mut prev_position = Vec2::new(0, 0);
    let mut current_position = get_start_pos(&map).unwrap();

    while let Some(next) = get_next(&prev_position, &current_position, &map) {
        path.push(next);
        prev_position = current_position;
        current_position = next;
    }

    // translate the path to text
    let mut prev_direction = Dir::None;
    let mut last_turn = 'R';
    let mut current_position = get_start_pos(&map).unwrap();
    let mut dir_to_dist: Vec<(char, u8)> = Vec::new();

    let mut count = 0;

    for pos in path {
        let cur_dir = Dir::from(pos - current_position);

        if prev_direction as u8 != cur_dir as u8 {
            if prev_direction as u8 != Dir::None as u8 {
                dir_to_dist.push((last_turn, count));

                last_turn = l_r(prev_direction, cur_dir);
            }

            count = 0;
            prev_direction = cur_dir;
        }

        count += 1;
        current_position = pos;
    }

    dir_to_dist.push((last_turn, count));

    let routine = "A,B,B,C,C,A,B,B,C,A";
    let a = "R,4,R,12,R,10,L,12";
    let b = "L,12,R,4,R,12";
    let c = "L,12,L,8,R,10";
    let ans = "n";

    input_string(&mut robot, &routine);
    input_string(&mut robot, &a);
    input_string(&mut robot, &b);
    input_string(&mut robot, &c);
    input_string(&mut robot, &ans);

    robot.computer.compute()
}

fn input_string(robot: &mut Robot, input: &str) {
    for byte in input.bytes() {
        robot.computer.add_input(byte as i64);
    }
    robot.computer.add_input('\n' as u8 as i64);
}

fn l_r(prev: Dir, next: Dir) -> char {
    let diff = next as i8 - prev as i8;

    if diff.abs() == 1 {
        if diff > 0 {
            'R'
        } else {
            'L'
        }
    } else {
        if diff < 0 {
            'R'
        } else {
            'L'
        }
    }
}

fn get_start_pos(map: &HashMap<Vec2, u8>) -> Option<Vec2> {
    for (pos, val) in map {
        if *val as char == '^' {
            return Some(pos.to_owned());
        }
    }

    None
}

fn get_next(prev: &Vec2, pos: &Vec2, map: &HashMap<Vec2, u8>) -> Option<Vec2> {
    let target = '#' as u8;

    let preferred = pos + (pos - prev);
    let entry = map.get(&preferred);
    if entry.is_some() && *entry.unwrap() == target {
        return Some(preferred);
    }

    for i in 1..5 {
        let dir = Dir::from(i);
        let new_pos = pos + Vec2::from(dir);

        let entry = map.get(&new_pos);
        if new_pos == *prev || entry.is_none() {
            continue;
        }

        let entry = entry.unwrap();

        if *entry == target {
            return Some(new_pos);
        }
    }

    None
}

impl Robot {
    fn new(computer: Computer) -> Self {
        Robot {
            computer: computer,
            pos: Vec2::new(0, 0),
        }
    }
}

struct Robot {
    computer: Computer,
    pos: Vec2,
}

impl From<i64> for Dir {
    fn from(val: i64) -> Self {
        use Dir::*;
        match val {
            1 => North,
            2 => South,
            3 => West,
            4 => East,
            _ => panic!("Uncovered value {}", val),
        }
    }
}

impl From<Vec2> for Dir {
    fn from(val: Vec2) -> Self {
        use Dir::*;
        match (val.x, val.y) {
            (0, -1) => North,
            (0, 1) => South,
            (-1, 0) => West,
            (1, 0) => East,
            _ => None,
        }
    }
}

impl From<Dir> for Vec2 {
    fn from(val: Dir) -> Self {
        use Dir::*;
        match val {
            None => panic!("Cannot convert none!"),
            North => Vec2::new(0, -1),
            South => Vec2::new(0, 1),
            West => Vec2::new(-1, 0),
            East => Vec2::new(1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    None,
    North,
    East,
    South,
    West,
}

fn get_intersections(map: &HashMap<Vec2, u8>) -> Vec<Vec2> {
    let mut result = Vec::new();

    let l = Vec2::new(-1, 0);
    let r = Vec2::new(1, 0);
    let u = Vec2::new(0, -1);
    let d = Vec2::new(0, 1);

    for (pos, num) in map {
        if *num != 35 {
            continue;
        }

        let mut neighbors = 0;

        match map.get(&(pos + l)) {
            Some(x) => {
                if *x == 35 {
                    neighbors += 1;
                }
            }
            None => {}
        }

        match map.get(&(pos + r)) {
            Some(x) => {
                if *x == 35 {
                    neighbors += 1;
                }
            }
            None => {}
        }

        match map.get(&(pos + u)) {
            Some(x) => {
                if *x == 35 {
                    neighbors += 1;
                }
            }
            None => {}
        }

        match map.get(&(pos + d)) {
            Some(x) => {
                if *x == 35 {
                    neighbors += 1;
                }
            }
            None => {}
        }

        if neighbors == 4 {
            result.push(pos.to_owned());
        }
    }

    result
}

fn draw(map: &HashMap<Vec2, u8>) {
    let mut current_display: Vec<u8> = vec![];

    let (minx, miny, maxx, maxy) = get_screen_dimensions(&map);
    let w = maxx - minx;
    let h = maxy - miny;
    let size = w * h;

    let offset = Vec2::new(minx, miny);

    for i in 0..size {
        let pos = Vec2::from(get_xy(i, w)) + offset;
        match map.get(&pos) {
            Some(x) => current_display.push(x.to_owned()),
            None => {}
        }
    }

    draw_image(&current_display);
}

fn draw_image(image: &Vec<u8>) {
    for i in 0..image.len() {
        let put = image[i] as char;

        if put.is_whitespace() {
            print!("{}", put);
        } else {
            print!("{}{}", put, put);
        }
    }
}

fn get_screen_dimensions<T>(map: &HashMap<Vec2, T>) -> (i64, i64, i64, i64) {
    let mut minx = i64::max_value();
    let mut miny = i64::max_value();

    let mut maxx = i64::min_value();
    let mut maxy = i64::min_value();

    for pos in map.keys() {
        let x = pos.x;
        let y = pos.y;

        minx = minx.min(x);
        miny = miny.min(y);

        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }

    (minx, miny, maxx + 1, maxy + 1)
}

fn get_xy(index: i64, width: i64) -> (i64, i64) {
    let x = index % width;
    let y = index / width;
    (x, y)
}
