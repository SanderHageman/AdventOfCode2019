use cgmath::*;
use num::Integer;

type Vec3 = Vector3<i32>;

pub fn day(input: String) {
    let moon_input = parse(input);

    let result_one = get_part_one(&moon_input);
    let result_two = get_part_two(&moon_input);

    println!("Day 12 Result1: {:?}", result_one);
    println!("Day 12 Result2: {:?}", result_two);
}

fn get_part_one(moon_input: &Vec<Moon>) -> i32 {
    let mut moon_vec = moon_input.clone();

    for _ in 0..1000 {
        let moon_copy = moon_vec.clone();

        for i in 0..moon_vec.len() {
            for other_moon in &moon_copy {
                moon_vec[i].update_velocity(other_moon)
            }
        }

        for i in 0..moon_vec.len() {
            moon_vec[i].apply_velocity();
        }
    }

    let mut total_energy = 0;

    for moon in &moon_vec {
        total_energy += moon.etotal();
    }

    total_energy
}

fn get_part_two(moon_input: &Vec<Moon>) -> u64 {
    let check = moon_input.clone();
    let mut moon_vec = moon_input.clone();

    let mut icount = 0u64;

    let mut counts = [0, 0, 0];

    while counts[0] == 0 || counts[1] == 0 || counts[2] == 0 {
        icount += 1;
        let moon_copy = moon_vec.clone();

        for i in 0..4 {
            for other_moon in &moon_copy {
                moon_vec[i].update_velocity(other_moon)
            }
        }

        for i in 0..4 {
            moon_vec[i].apply_velocity();
        }

        for i in 0..3 {
            if counts[i] == 0 && are_eq(i, &moon_vec, &check) {
                counts[i] = icount;
            }
        }
    }

    counts[0].lcm(&counts[1].lcm(&counts[2]))
}

fn are_eq(index: usize, current: &Vec<Moon>, original: &Vec<Moon>) -> bool {
    for i in 0..current.len() {
        if current[i].pos[index] != original[i].pos[index] {
            return false;
        } else if current[i].vel[index] != original[i].vel[index] {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Moon {
        Moon {
            pos: pos,
            vel: Vec3::new(0, 0, 0),
        }
    }

    fn update_velocity(&mut self, other: &Moon) {
        self.vel.x += Moon::get_velocity_change(self.pos.x, other.pos.x);
        self.vel.y += Moon::get_velocity_change(self.pos.y, other.pos.y);
        self.vel.z += Moon::get_velocity_change(self.pos.z, other.pos.z);
    }

    fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }

    fn etotal(&self) -> i32 {
        self.ekin() * self.epot()
    }

    fn ekin(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn epot(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn get_velocity_change(us: i32, them: i32) -> i32 {
        if us > them {
            -1
        } else if us < them {
            1
        } else {
            0
        }
    }
}

fn parse(input: String) -> Vec<Moon> {
    let moon_data_lines = input.lines().collect::<Vec<_>>();
    let mut result: Vec<Moon> = vec![];

    // <x=-7, y=17, z=-11>
    let blacklist = vec!['<', 'x', '=', 'y', 'z', '>'];
    for line in moon_data_lines {
        let mut extract = line.to_owned();
        extract.retain(|c| !blacklist.contains(&c));

        //-7, 17, -11
        let position_vec = extract
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if position_vec.len() != 3 {
            panic!("Unable to extract correctly for moon {:?}", line);
        }

        let position = Vec3::new(position_vec[0], position_vec[1], position_vec[2]);
        result.push(Moon::new(position));
    }

    result
}
