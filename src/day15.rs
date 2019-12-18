use super::intcode_computer::*;
use cgmath::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::convert::From;

type Vec2 = Vector2<i64>;

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = get_part_one(&input_vec); // 214
    let result_two = 0;

    println!("Day 15 Result1: {:?}", result_one);
    println!("Day 15 Result2: {:?}", result_two);
}

fn get_part_one(input_vec: &Vec<i64>) -> usize {
    let mut robot = Robot::new(Computer::new(vec![], &input_vec, 3000));
    robot.send_command(Dir::North);

    let mut i: i64 = 0;
    let target = 1000000;

    while robot.keep_update() {
        if i > target {
            break;
        }

        robot.update();

        i += 1;
    }

    robot.maze.set_tile(Vec2::new(0, 0), Tile::Bot);
    robot.maze.set_tile(robot.maze.robot_pos, Tile::Bot);
    robot.maze.set_tile(robot.maze.oxygen_pos, Tile::Oxy);
    // robot.maze.draw();

    // let mut result = 0;
    // while result != 214 {
    //     result = robot.maze.solve();
    //     println!("{}", result);
    // }

    robot.maze.solve()
}

impl Robot {
    fn new(computer: Computer) -> Self {
        Robot {
            computer: computer,
            maze: Maze::new(),
            last_command: Dir::North,
            pos: Vec2::new(0, 0),
        }
    }

    fn keep_update(&self) -> bool {
        !self.computer.stop
    }

    fn update(&mut self) {
        use Reply::*;

        let reply = self.tick();

        match reply {
            Wall => {
                self.maze
                    .set_tile(self.pos + Vec2::from(self.last_command), Tile::Wall);
            }
            Step => {
                self.maze.set_tile(self.pos, Tile::Empty);
                self.pos += Vec2::from(self.last_command);
            }
            Oxy => {
                self.pos += Vec2::from(self.last_command);
                self.maze.oxygen_pos = self.pos;
            }
        }

        self.maze.robot_pos = self.pos;
        self.send_command(self.get_next_direction());
    }

    fn get_next_direction(&self) -> Dir {
        let mut backup = Vec::<Dir>::new();

        for i in 1..5 {
            let dir = Dir::from(i);
            let new_pos = self.pos + Vec2::from(dir);

            match self.maze.get_tile(&new_pos) {
                Tile::Unknown => return dir,
                Tile::Empty => backup.push(dir),
                _ => continue,
            }
        }

        if backup.len() == 0 {
            // we can sometimes get stuck when we've found the oxygen tank
            let val = (random::<u64>() % 4) as i64 + 1;
            return Dir::from(val);
        }

        let rand = random::<usize>() % backup.len();
        backup[rand]
    }

    fn tick(&mut self) -> Reply {
        Reply::from(self.computer.compute_til_output())
    }

    fn send_command(&mut self, dir: Dir) {
        self.last_command = dir;
        self.computer.add_input(dir as i64);
    }
}

impl Maze {
    fn new() -> Self {
        Maze {
            map: HashMap::new(),
            robot_pos: Vec2::new(0, 0),
            oxygen_pos: Vec2::new(0, 0),
            normalized_cells: Vec::new(),
            solve_cells: Vec::new(),
        }
    }

    fn get_tile(&self, pos: &Vec2) -> Tile {
        self.map.get(pos).unwrap_or(&Tile::Unknown).to_owned()
    }

    fn set_tile(&mut self, pos: Vec2, new_tile: Tile) {
        self.map.insert(pos, new_tile);
    }

    fn draw(&self) {
        let mut current_display: Vec<Tile> = vec![];

        let (minx, miny, maxx, maxy) = self.get_screen_dimensions();
        let w = maxx - minx;
        let h = maxy - miny;
        let size = w * h;

        let offset = Vec2::new(minx, miny);

        for i in 0..size {
            let pos = Vec2::from(Maze::get_xy(i, w)) + offset;
            current_display.push(self.get_tile(&pos));
        }

        Maze::draw_image(w as usize, &current_display);
    }

    fn draw_image(width: usize, image: &Vec<Tile>) {
        for i in 0..image.len() {
            if i % width == 0 && i != 0 {
                print!("\n");
            }

            let put = match image[i] {
                Tile::Unknown => '█',
                Tile::Empty => '░',
                Tile::Wall => '█', //'▓',
                Tile::Oxy => 'X',
                Tile::Bot => 'O',
            };

            print!("{}{}", put, put);
        }
        print!("\n");
    }

    fn get_screen_dimensions(&self) -> (i64, i64, i64, i64) {
        let mut minx = i64::max_value();
        let mut miny = i64::max_value();

        let mut maxx = i64::min_value();
        let mut maxy = i64::min_value();

        for pos in self.map.keys() {
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

    ///MAZE SOLVING: Get the next candidate cell
    fn solution_next(&mut self, cell: &Vec2, w: i64, h: i64) -> Option<Vec2> {
        self.solve_cells[cell.x as usize][cell.y as usize] = false;
        let mut neighbors = Vec::new();

        if cell.x > 0
            && self.solve_cells[(cell.x - 1) as usize][cell.y as usize]
            && !self.normalized_cells[cell.x as usize][cell.y as usize]
        {
            neighbors.push(Vec2::new(cell.x - 1, cell.y));
        }
        if cell.y > 0
            && self.solve_cells[cell.x as usize][(cell.y - 1) as usize]
            && !self.normalized_cells[cell.x as usize][cell.y as usize]
        {
            neighbors.push(Vec2::new(cell.x, cell.y - 1));
        }
        if cell.x < w - 2
            && self.solve_cells[(cell.x + 1) as usize][cell.y as usize]
            && !self.normalized_cells[cell.x as usize][(cell.y + 1) as usize]
        {
            neighbors.push(Vec2::new(cell.x + 1, cell.y));
        }
        if cell.y < h - 2
            && self.solve_cells[cell.x as usize][(cell.y + 1) as usize]
            && !self.normalized_cells[(cell.x + 1) as usize][cell.y as usize]
        {
            neighbors.push(Vec2::new(cell.x, cell.y + 1));
        }
        if neighbors.is_empty() {
            None
        } else {
            let next = neighbors.get(random::<usize>() % neighbors.len()).unwrap();
            Some(*next)
        }
    }
    //https://www.rosettacode.org/wiki/Maze_solving#Rust

    ///MAZE SOLVING: solve the maze
    ///Uses self.cells to store the solution cells (true)
    fn solve(&mut self) -> usize {
        self.normalize_map();

        let (minx, miny, maxx, maxy) = self.get_screen_dimensions();
        let w = maxx - minx;
        let h = maxy - miny;

        let offset = Vec2::new(minx, miny);

        self.solve_cells = vec![vec![true; h as usize]; w as usize];

        let mut solution: Vec<Vec2> = Vec::new();
        let mut next = Vec2::new(0, 0) - offset;
        solution.push(next);
        let last = self.oxygen_pos - offset;

        'main: loop {
            while let Some(cell) = self.solution_next(&next, w, h) {
                solution.push(cell);
                if cell == last {
                    break 'main;
                }
                next = cell;
            }
            solution.pop().unwrap();
            next = *solution.last().unwrap();
        }

        for cell in solution {
            self.set_tile(cell, Tile::Oxy);
        }

        self.draw();

        0
    }

    fn normalize_map(&mut self) {
        let (minx, miny, maxx, maxy) = self.get_screen_dimensions();
        let w = maxx - minx;
        let h = maxy - miny;
        let size = w * h;

        self.normalized_cells = vec![vec![false; h as usize]; w as usize];
        let offset = Vec2::new(minx, miny);

        for i in 0..size {
            let pos = Vec2::from(Maze::get_xy(i, w));
            self.normalized_cells[pos.x as usize][pos.y as usize] =
                match self.get_tile(&(pos + offset)) {
                    Tile::Wall => false,
                    Tile::Unknown => false,
                    _ => true,
                }
        }
    }
}

struct Maze {
    map: HashMap<Vec2, Tile>,
    robot_pos: Vec2,
    oxygen_pos: Vec2,
    normalized_cells: Vec<Vec<bool>>,
    solve_cells: Vec<Vec<bool>>,
}

struct Robot {
    computer: Computer,
    maze: Maze,
    last_command: Dir,
    pos: Vec2,
}

impl From<i64> for Reply {
    fn from(val: i64) -> Self {
        use Reply::*;
        match val {
            0 => Wall,
            1 => Step,
            2 => Oxy,
            _ => panic!("Uncovered value {}", val),
        }
    }
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

impl From<Dir> for Vec2 {
    fn from(val: Dir) -> Self {
        use Dir::*;
        match val {
            None => panic!("Cannot convert none!"),
            North => Vec2::new(0, 1),
            South => Vec2::new(0, -1),
            West => Vec2::new(1, 0),
            East => Vec2::new(-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Reply {
    Wall,
    Step,
    Oxy,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxy,
    Bot,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    None,
    North,
    South,
    West,
    East,
}
