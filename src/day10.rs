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
        .map(|b| get_see(b, w, h, &grid))
        .collect::<Vec<_>>();

    let result_one = can_see;
    let result_two = 0;

    println!("Day 10 Result1: {:?}", result_one);
    println!("Day 10 Result2: {:?}", result_two);
}

fn get_see(
    iter: (usize, &bool),
    grid_width: usize,
    grid_height: usize,
    grid: &Vec<Vec<bool>>,
) -> u32 {
    let (index, is_asteroid) = iter;

    if !is_asteroid {
        return 0;
    }

    let x_index = index % grid_width;
    let y_index = (index - x_index) / grid_height;

    for y in 0..grid_height {
        for x in 0..grid_width {
            // only check for edges
            if y != 0 && x != 0 {
                continue;
            }
        }
    }

    (x_index + y_index) as u32
}
