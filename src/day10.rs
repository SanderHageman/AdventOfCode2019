use cgmath::*;

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

    let result_one = get_xy(max_i, w, h);
    let result_two = 0;

    println!("Day 10 Result1: {:?}", result_one);
    println!("Day 10 Result2: {:?}", result_two);
}

fn get_see(iter: (usize, &bool), grid_width: usize, grid_height: usize, grid: &Vec<bool>) -> u32 {
    let (index, is_asteroid) = iter;

    if !is_asteroid {
        return 0;
    }

    let (x_index, y_index) = get_xy(index, grid_width, grid_height);

    let mut result = 0;

    // only check for edges
    for x in 0..grid_width {
        let origin = Vector2::new(x_index as f64, y_index as f64);
        let extreme_one = Vector2::new(x as f64, 0f64);
        let extreme_two = Vector2::new(x as f64, (grid_height - 1) as f64);

        let direction_one = (extreme_one - origin).normalize();
        let direction_two = (extreme_two - origin).normalize();

        if has_on_line(&origin, &direction_one, grid_width, grid_height, &grid) {
            result += 1;
        }

        if has_on_line(&origin, &direction_two, grid_width, grid_height, &grid) {
            result += 1;
        }
    }

    for y in 0..grid_height {
        let origin = Vector2::new(x_index as f64, y_index as f64);
        let extreme_one = Vector2::new(0f64, y as f64);
        let extreme_two = Vector2::new((grid_width - 1) as f64, y as f64);

        let direction_one = (extreme_one - origin).normalize();
        let direction_two = (extreme_two - origin).normalize();

        if has_on_line(&origin, &direction_one, grid_width, grid_height, &grid) {
            result += 1;
        }

        if has_on_line(&origin, &direction_two, grid_width, grid_height, &grid) {
            result += 1;
        }
    }

    result
}

fn has_on_line(
    origin: &Vector2<f64>,
    target_direction: &Vector2<f64>,
    grid_width: usize,
    grid_height: usize,
    grid: &Vec<bool>,
) -> bool {
    for i in 0..grid.len() {
        if !grid[i] {
            continue;
        }

        let (x, y) = get_xy(i, grid_width, grid_height);
        let point = Vector2::new(x as f64, y as f64);

        let direction = (point - origin).normalize();
        if direction == *target_direction {
            return true;
        }
    }

    false
}

fn get_xy(index: usize, width: usize, height: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / height;
    (x, y)
}

fn get_on_line(
    line_start: Vector2<f64>,
    line_end: Vector2<f64>,
    point: Vector2<f64>,
) -> (bool, f64) {
    let dist = (line_start - point).magnitude2();
    (
        dist + (line_end - point).magnitude2() == (line_end - line_start).magnitude2(),
        dist,
    )
}
