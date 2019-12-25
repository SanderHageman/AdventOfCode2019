use super::intcode_computer::*;
use cgmath::*;
use rand::prelude::*;
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
    let mut input = input.clone();
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

                last_turn = get_l_or_r(prev_direction, cur_dir);
            }

            count = 0;
            prev_direction = cur_dir;
        }

        count += 1;
        current_position = pos;
    }

    dir_to_dist.push((last_turn, count));

    let input_strings = find_routine(&dir_to_dist);

    for line in input_strings {
        send_string_to_robot(&mut robot, &line);
    }

    // never display
    send_string_to_robot(&mut robot, &"n");

    robot.computer.compute()
}

fn find_routine(path: &Vec<(char, u8)>) -> Vec<String> {
    // steps to take:
    // keep taking entry [0..i(+1)] until it doesn't match anything else any more
    // assign that to a letter, remove from the path and continue until done ;)

    let mut max_size = vec![4, 4, 4];
    let mut entries = Vec::new();

    'main: loop {
        let mut mut_path = path.to_owned();
        entries.clear();

        while entries.len() < 3 {
            let mut test_size = 1;
            while test_size <= max_size[entries.len()]
                && line_length_allowed(test_size, &mut_path)
                && has_equal(test_size, &mut_path)
            {
                test_size += 1;
            }
            test_size -= 1;

            if test_size <= 0 {
                break;
            }

            entries.push(Vec::from(&mut_path[0..test_size]));
            remove_from_vec(test_size, &mut mut_path);

            if mut_path.len() <= 0 {
                break 'main;
            }
        }

        for i in 0..max_size.len() {
            max_size[i] = (random::<usize>() % 3) + 2;
        }
    }

    routine_to_string(&entries, path)
}

fn routine_to_string(entries: &Vec<Vec<(char, u8)>>, path: &Vec<(char, u8)>) -> Vec<String> {
    let mut x_indices = Vec::new();
    for entry in entries {
        x_indices.push(get_equal_indices(entry, path));
    }

    let mut indices = Vec::<usize>::new();
    for entry in &x_indices {
        indices.extend(entry.iter());
    }

    indices.sort();

    let mut main_routine = Vec::new();
    for index in &indices {
        for i in 0..3 {
            if x_indices[i].contains(&index) {
                let put = match i {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    _ => panic!("Unhandled index {}", i),
                };
                main_routine.push(put);
                break;
            }
        }
    }

    let mut result = Vec::new();

    let mut main_str = String::new();
    for val in main_routine {
        main_str.push(val);
        main_str.push(',');
    }
    main_str.pop();
    result.push(main_str);

    for i in 0..3 {
        let mut res_str = String::new();

        for (ch, count) in &entries[i] {
            res_str += &format!(",{},{}", ch, count);
        }

        res_str.remove(0);
        result.push(res_str);
    }

    result
}

fn remove_from_vec(count: usize, path: &mut Vec<(char, u8)>) {
    let mut remove_indices = get_equal(count, path);
    remove_indices.reverse();

    for base_index in remove_indices {
        for i in (0..count).rev() {
            path.remove(base_index + i);
        }
    }
}

fn line_length_allowed(count: usize, path: &Vec<(char, u8)>) -> bool {
    let entry = &path[0..count];
    let mut result = 0;

    for (_, i) in entry {
        // 'R' + ',' + '1' (+ '2') + ','
        result += 4;

        if *i > 9 {
            result += 1;
        }
    }

    result <= 20
}

fn has_equal(count: usize, path: &Vec<(char, u8)>) -> bool {
    get_equal(count, path).len() > 1
}

fn get_equal_indices(slice: &[(char, u8)], path: &Vec<(char, u8)>) -> Vec<usize> {
    let mut result = Vec::new();
    let count = slice.len();

    for i in 0..=(path.len() - count) {
        let extend = i + count;
        assert!(extend <= path.len());

        let comp = &path[i..extend];

        if are_equal(slice, comp) {
            result.push(i);
        }
    }

    result
}

fn get_equal(count: usize, path: &Vec<(char, u8)>) -> Vec<usize> {
    let max_count = path.len() / count;
    assert!(max_count > 1);

    let entry = &path[0..count];
    get_equal_indices(entry, path)
}

fn are_equal(a: &[(char, u8)], b: &[(char, u8)]) -> bool {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn send_string_to_robot(robot: &mut Robot, input: &str) {
    for byte in input.bytes() {
        robot.computer.add_input(byte as i64);
    }
    robot.computer.add_input('\n' as u8 as i64);
}

fn get_l_or_r(prev: Dir, next: Dir) -> char {
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
    let change = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (pos, num) in map {
        if *num != 35 {
            continue;
        }

        let mut neighbors = 0;

        for i in 0..change.len() {
            match map.get(&(pos + Vec2::from(change[i]))) {
                Some(x) => {
                    if *x == 35 {
                        neighbors += 1;
                    }
                }
                None => {}
            }
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
