use super::intcode_computer;

pub fn day(input: std::string::String) {
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    let result_one = get_part_one(&input_vec);
    let result_two = 0;

    println!("Day 7 Result1: {:?}", result_one);
    println!("Day 7 Result2: {:?}", result_two);
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

                        let amp_a = intcode_computer::Computer::simple(vec![a, 0], &input_vec);
                        let amp_b = intcode_computer::Computer::simple(vec![b, amp_a], &input_vec);
                        let amp_c = intcode_computer::Computer::simple(vec![c, amp_b], &input_vec);
                        let amp_d = intcode_computer::Computer::simple(vec![d, amp_c], &input_vec);
                        let amp_e = intcode_computer::Computer::simple(vec![e, amp_d], &input_vec);

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
