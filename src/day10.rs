use cgmath::*;
use float_cmp::ApproxEq;
use std::collections::HashSet;

pub fn day(input: std::string::String) {
    let grid = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    let w = (grid.len() as f64).sqrt() as usize;
    let h = (grid.len() as f64).sqrt() as usize;
    assert_eq!(grid.len(), w * h);

    let can_see = grid
        .iter()
        .enumerate()
        .map(|b| get_see(b, w, h, &grid))
        .collect::<Vec<_>>();

    let mut max = u32::min_value();
    let mut max_i = usize::max_value();

    for (i, val) in can_see.iter().enumerate() {
        max = max.max(*val);

        if max == *val {
            max_i = i;
        }
    }

    let input_two = max_i;
    let mut play_grid = grid.clone();
    let mut destroyed: Vec<usize> = vec![];

    while destroyed.len() <= 200 {
        let mut angles = get_see_angles(input_two, w, h, &grid)
            .into_iter()
            .collect::<Vec<_>>();
        angles.sort_unstable();

        for angle in angles {
            let destroy_index = get_closest_by_angle(max_i, angle, w, h, &play_grid);

            assert!(play_grid[destroy_index]);
            assert!(!destroyed.contains(&destroy_index));

            destroyed.push(destroy_index);
            play_grid[destroy_index] = false;
        }
    }

    let (rtx, rty) = get_xy(destroyed[199], w, h);

    let result_one = max;
    let result_two = rtx * 100 + rty;

    println!("Day 10 Result1: {:?} {:?}", result_one, get_xy(max_i, w, h));
    println!("Day 10 Result2: {:?} {:?}", result_two, (rtx, rty));
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
    let normal = Vector2::new(0.0, -1f32);

    let mut result_angle = HashSet::<i32>::new();

    let size = grid_width * grid_height;

    for i in 0..size {
        if i == index || !grid[i] {
            continue;
        }

        let (x, y) = get_xy(i, grid_width, grid_height);
        let target = Vector2::new(x as f32, y as f32);
        let diff = target - origin;
        let mut angle = -diff.angle(normal).0.to_degrees();
        if angle < 0.0 {
            angle = 360.0 + angle;
        }
        let cangle = canon_angle(angle);

        assert!(angle.approx_eq(uncanon_angle(cangle), (0.000001, 1)));
        result_angle.insert(cangle);
    }

    result_angle
}

fn get_closest_by_angle(
    origin_index: usize,
    target_angle: i32,
    grid_width: usize,
    grid_height: usize,
    grid: &Vec<bool>,
) -> usize {
    let (x_index, y_index) = get_xy(origin_index, grid_width, grid_height);
    let origin = Vector2::new(x_index as f32, y_index as f32);
    let normal = Vector2::new(0.0, -1f32);

    let size = grid_width * grid_height;

    let mut index_and_dist: Vec<(usize, f32)> = vec![];

    for i in 0..size {
        if i == origin_index || !grid[i] {
            continue;
        }

        let (x, y) = get_xy(i, grid_width, grid_height);
        let target = Vector2::new(x as f32, y as f32);
        let diff = target - origin;
        let mut angle = -diff.angle(normal).0.to_degrees();
        if angle < 0.0 {
            angle = 360.0 + angle;
        }
        let cangle = canon_angle(angle);
        let mangle = uncanon_angle(target_angle);

        if target_angle != cangle {
            continue;
        }

        assert!(mangle.approx_eq(angle, (0.000001, 1)));

        index_and_dist.push((i, diff.magnitude2()));
    }

    let mut closest = f32::max_value();
    let mut cindex = 0;

    for (index, dist) in index_and_dist {
        closest = closest.min(dist);
        if closest == dist {
            cindex = index;
        }
    }

    cindex
}

fn get_xy(index: usize, width: usize, height: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / height;
    (x, y)
}

fn canon_angle(angle: f32) -> i32 {
    (angle * 1024.0 * 1024.0).round() as i32
}

fn uncanon_angle(canon: i32) -> f32 {
    canon as f32 / 1024.0 / 1024.0
}
