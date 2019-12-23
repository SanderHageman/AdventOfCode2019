use super::intcode_computer::*;
use cgmath::*;
use std::collections::HashMap;

type Vec2 = Vector2<i64>;

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = get_part_one(&input_vec);
    let result_two = 0;

    println!("Day 17 Result1: {:?}", result_one);
    println!("Day 17 Result2: {:?}", result_two);
}

fn get_part_one(input: &Vec<i64>) -> i64 {
    let mut computer = Computer::new(vec![], &input, 10000);

    let mut map = HashMap::new();
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

    draw(&map);
    let intersections = get_intersections(&map);
    let alignment = intersections.iter().map(|x| x.x * x.y).collect::<Vec<_>>();

    let mut result = 0;
    for val in alignment {
        result += val;
    }

    result
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
