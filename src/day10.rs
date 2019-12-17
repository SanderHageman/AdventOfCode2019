use cgmath::*;
use std::collections::HashSet;

pub fn day(input: std::string::String) {
    let input_vec = input.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<bool>> = vec![vec![]];
    grid.clear();

    for line in input_vec {
        grid.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
    }

    let w = grid[0].len();
    let h = grid.len();

    let grid_one_d = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    assert_eq!(grid_one_d.len(), w * h);

    let can_see = grid_one_d
        .iter()
        .enumerate()
        .map(|b| get_see(b, w, h, &grid_one_d))
        .collect::<Vec<_>>();

    let mut max = u32::min_value();
    let mut max_i = usize::max_value();

    for (i, val) in can_see.iter().enumerate() {
        max = max.max(*val);

        if max == *val {
            max_i = i;
        }
    }

    let result_one = max;
    let result_two = 0;

    println!("Day 10 Result1: {:?} {:?}", result_one, get_xy(max_i, w, h));
    println!("Day 10 Result2: {:?}", result_two);
}

fn get_see(iter: (usize, &bool), grid_width: usize, grid_height: usize, grid: &Vec<bool>) -> u32 {
    let (index, is_asteroid) = iter;

    if !is_asteroid {
        return 0;
    }

    get_see_angles(index, grid_width, grid_height, &grid).len() as u32
}

fn get_see_angles(
    index: usize,
    grid_width: usize,
    grid_height: usize,
    grid: &Vec<bool>,
) -> HashSet<i32> {
    let (x_index, y_index) = get_xy(index, grid_width, grid_height);
    let origin = Vector2::new(x_index as f32, y_index as f32);
    let base = Vector2::new(x_index as f32, y_index as f32 - 1f32);

    let mut result_angle = HashSet::<i32>::new();

    let size = grid_width * grid_height;

    for i in 0..size {
        if i == index || !grid[i] {
            continue;
        }

        let (x, y) = get_xy(i, grid_width, grid_height);
        let target = Vector2::new(x as f32, y as f32);
        let diff = target - origin;
        let angle = diff.angle(base);

        result_angle.insert(canon_angle(angle.0));
    }

    result_angle
}

fn get_xy(index: usize, width: usize, height: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / height;
    (x, y)
}

fn canon_angle(angle: f32) -> i32 {
    (angle * 1024.0 * 1024.0).round() as i32
}
