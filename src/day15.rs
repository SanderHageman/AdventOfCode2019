use super::intcode_computer::*;
use cgmath::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::convert::From;

type Vec2 = Vector2<i64>;

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = get_part_one(&input_vec);
    let result_two = 0;

    println!("Day 15 Result1: {:?}", result_one);
    println!("Day 15 Result2: {:?}", result_two);
}

fn get_part_one(input_vec: &Vec<i64>) -> i64 {
    let mut robot = Robot::new(Computer::new(vec![], &input_vec, 3000));
    robot.send_command(Dir::North);

    let mut i = 0;

    while robot.keep_update() {
        if i > 1000000000 {
            robot.maze.draw();
            break;
        }
        robot.update();

        i += 1;
    }

    0
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

                let val = (random::<u64>() % 4) as i64 + 1;
                self.send_command(Dir::from(val));
            }
            Step => {
                self.maze.set_tile(self.pos, Tile::Empty);
                self.pos += Vec2::from(self.last_command);
                self.send_command(self.last_command.to_owned());
            }
            Oxy => {
                self.maze.set_tile(self.pos, Tile::Oxy);
                self.pos += Vec2::from(self.last_command);
                println!("Found the oxygen!");
                self.computer.stop = true;
            }
        }

        self.maze.set_tile(self.pos, Tile::Bot);
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
        }
    }

    fn set_tile(&mut self, pos: Vec2, new_tile: Tile) {
        self.map.insert(pos, new_tile);
    }

    fn draw(&self) {
        //println!("{}[2J", 27 as char);

        let mut current_display: Vec<Tile> = vec![];

        let (minx, miny, maxx, maxy) = self.get_screen_dimensions();

        let w = maxx - minx;
        let h = maxy - miny;
        let size = w * h;

        for i in 0..size {
            let pos = Maze::get_xy(i, w);
            let corpos = (pos.0 + minx, pos.1 + miny);
            let pixel = self
                .map
                .get(&Vec2::from(corpos))
                .unwrap_or(&Tile::Wall)
                .to_owned();
            current_display.push(pixel);
        }

        Maze::draw_image(w as usize, &current_display);
    }

    fn draw_image(width: usize, image: &Vec<Tile>) {
        for i in 0..image.len() {
            if i % width == 0 && i != 0 {
                print!("\n");
            }

            let put = match image[i] {
                Tile::Empty => '░',
                Tile::Wall => '█',
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
}

struct Maze {
    map: HashMap<Vec2, Tile>,
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
    Empty,
    Wall,
    Oxy,
    Bot,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North = 1,
    South,
    West,
    East,
}
