use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    println!("Day 7 Result1: {:?}", get_part_one(&input_vec));
    println!("Day 7 Result2: {:?}", get_part_two(&input_vec));
}

fn get_part_two(input_vec: &Vec<i32>) -> i32 {
    let mut highest = 0;
    let mut _highest_set: String = Default::default();

    let low = 5;
    let high = 10;

    for a in low..high {
        for b in low..high {
            if b == a {
                continue;
            }
            for c in low..high {
                if c == a || c == b {
                    continue;
                }

                for d in low..high {
                    if d == a || d == b || d == c {
                        continue;
                    }

                    for e in low..high {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let mut computers: Vec<Computer> = Vec::with_capacity(5);
                        for _i in 0..5 {
                            computers.push(Computer::new(vec![a, 0], &input_vec));
                            computers.push(Computer::new(vec![b], &input_vec));
                            computers.push(Computer::new(vec![c], &input_vec));
                            computers.push(Computer::new(vec![d], &input_vec));
                            computers.push(Computer::new(vec![e], &input_vec));
                        }

                        let mut output = computers[0].compute_til_output();

                        // warmup
                        for i in 1..5 {
                            computers[i].add_input(output);
                            output = computers[i].compute_til_output();
                        }

                        while !computers[4].stop {
                            for i in 0..5 {
                                computers[i].add_input(output);
                                output = computers[i].compute_til_output();
                            }
                        }

                        let output = computers[4].output.unwrap();
                        highest = highest.max(output);
                        if highest == output {
                            _highest_set = format!("a{} b{} c{} d{} e{}", a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }

    highest
}

fn get_part_one(input_vec: &Vec<i32>) -> i32 {
    let mut highest = 0;
    let mut _highest_set: String = Default::default();

    let low = 0;
    let high = 5;

    for a in low..high {
        for b in low..high {
            if b == a {
                continue;
            }
            for c in low..high {
                if c == a || c == b {
                    continue;
                }

                for d in low..high {
                    if d == a || d == b || d == c {
                        continue;
                    }

                    for e in low..high {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let amp_a = Computer::simple(vec![a, 0], &input_vec);
                        let amp_b = Computer::simple(vec![b, amp_a], &input_vec);
                        let amp_c = Computer::simple(vec![c, amp_b], &input_vec);
                        let amp_d = Computer::simple(vec![d, amp_c], &input_vec);
                        let amp_e = Computer::simple(vec![e, amp_d], &input_vec);

                        highest = highest.max(amp_e);

                        if highest == amp_e {
                            _highest_set = format!("a{} b{} c{} d{} e{}", a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }

    highest
}
