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

fn get_part_one(input_vec: &Vec<i32>) -> i32 {
    let mut highest = 0;
    let mut _highest_set: String = Default::default();

    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }

                for d in 0..5 {
                    if d == a || d == b || d == c {
                        continue;
                    }

                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let amp_a = intcode_computer::run_intcode(a, 0, &input_vec);
                        let amp_b = intcode_computer::run_intcode(b, amp_a, &input_vec);
                        let amp_c = intcode_computer::run_intcode(c, amp_b, &input_vec);
                        let amp_d = intcode_computer::run_intcode(d, amp_c, &input_vec);
                        let amp_e = intcode_computer::run_intcode(e, amp_d, &input_vec);

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
