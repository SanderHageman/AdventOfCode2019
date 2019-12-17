use super::intcode_computer::*;
use std::collections::BTreeMap;
use std::{thread, time};

pub fn day(input: String) {
    let input_vec = Computer::parse_input(input);

    let result_one = get_part_one(&input_vec);
    let result_two = get_part_two(&input_vec);

    println!("Day 13 Result1: {:?}", result_one);
    println!("Day 13 Result2: {:?}", result_two);
}

fn get_part_one(input_vec: &Vec<i64>) -> usize {
    let mut arcade = Arcade::new(Computer::new(vec![], &input_vec, 3000));

    while arcade.keep_update() {
        arcade.update();
    }

    arcade.screen.iter().filter(|x| *x.1 == 2).count()
}

fn get_part_two(input_vec: &Vec<i64>) -> usize {
    let mut register = input_vec.clone();

    //enable free play
    register[0] = 2;

    let mut arcade = Arcade::new(Computer::new(vec![0], &register, 3000));

    while arcade.keep_update() {
        arcade.update();
    }

    arcade.score
}

impl Arcade {
    fn new(computer: Computer) -> Arcade {
        Arcade {
            computer: computer,
            screen: BTreeMap::new(),
            screen_width: 0,
            screen_height: 0,
            score: 0,
        }
    }

    fn keep_update(&self) -> bool {
        !self.computer.stop
    }

    fn update(&mut self) {
        let tick_data = self.tick();

        if tick_data.0 == (-1, 0) {
            self.score = tick_data.1;
        } else {
            self.put(tick_data);
        }
    }

    fn tick(&mut self) -> ((i64, i64), usize) {
        (
            (
                self.computer.compute_til_output(),
                self.computer.compute_til_output(),
            ),
            self.computer.compute_til_output() as usize,
        )
    }

    fn put(&mut self, tick_data: ((i64, i64), usize)) {
        match self.screen.insert(tick_data.0, tick_data.1) {
            Some(_) => {
                if self.screen_width == 0 {
                    self.screen_width = self.get_screen_width();
                    self.screen_height = self.screen.len() / self.screen_width;
                    self.draw();
                }
                let wait = time::Duration::from_millis(150);
                thread::sleep(wait);
                self.draw();
            }
            None => {}
        }
    }

    fn draw(&self) {
        let mut current_display: Vec<usize> = vec![];

        let w = self.screen_width;
        let size = w * self.screen_height;

        for i in 0..size {
            let pos = Arcade::get_xy(i, w);
            current_display.push(self.screen.get(&pos).unwrap().to_owned());
        }

        println!("{}[2J", 27 as char);
        Arcade::draw_image(self.screen_width, &current_display);
    }

    fn get_screen_width(&self) -> usize {
        let mut maxx = i64::min_value();

        for pos in &self.screen {
            let (x, _) = pos.0;
            maxx = maxx.max(*x);
        }

        maxx as usize + 1
    }

    fn draw_image(width: usize, image: &Vec<usize>) {
        let mut data = String::new();

        for i in 0..image.len() {
            if i % width == 0 && i != 0 {
                data.push_str("\n");
            }

            let (put1, put2) = match image[i] {
                0 => ('░', '░'),
                1 => ('▓', '▓'),
                2 => ('█', '█'),
                3 => ('▂', '▂'),
                4 => ('▗', '▖'),
                _ => panic!("pixel out of range"),
            };

            data.push_str(&format!("{}{}", put1, put2));
        }

        println!("{}", data);
    }

    fn get_xy(index: usize, width: usize) -> (i64, i64) {
        let x = index % width;
        let y = index / width;
        (x as i64, y as i64)
    }
}

#[derive(Debug)]
struct Arcade {
    computer: Computer,
    screen: BTreeMap<(i64, i64), usize>,
    screen_width: usize,
    screen_height: usize,
    score: usize,
}
