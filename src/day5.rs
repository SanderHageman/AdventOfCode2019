pub fn day(input: std::string::String) {
    let input_vec = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    println!("Day 5 Result1: {:?}", run_intcode(1, &input_vec));
    println!("Day 5 Result2: {:?}", 0);
}

fn run_intcode(input: i32, input_vec: &Vec<i32>) -> i32 {
    let mut result_vec = input_vec.to_vec();
    let mut output = 0;

    let mut i = 0;
    while i < result_vec.len() {
        let opcode = Instruction::new(result_vec[i]);
        if opcode.opcode == 99 {
            break;
        };

        if opcode.opcode > 4 || opcode.opcode < 0 {
            println!("{:?} {:?}", input, opcode);
            continue;
        }

        let parameter_one = match opcode.paramode_one {
            0 => result_vec[result_vec[i + 1] as usize],
            1 => result_vec[i + 1],
            _ => panic!("uncovered parameter mode {:?}", opcode),
        };

        let parameter_two = match opcode.paramode_one {
            0 => result_vec[result_vec[i + 2] as usize],
            1 => result_vec[i + 2],
            _ => panic!("uncovered parameter mode {:?}", opcode),
        };

        // let parameter_thr = match opcode.paramode_one {
        //     0 => result_vec[result_vec[i + 3] as usize],
        //     1 => result_vec[i + 3],
        //     _ => panic!("uncovered parameter mode {:?}", opcode),
        // };

        let instruction_count;
        let index = result_vec[i + 3] as usize;
        let i2 = result_vec[i + 1] as usize;
        result_vec[index] = match opcode.opcode {
            1 => {
                instruction_count = 4;
                parameter_one + parameter_two
            }
            2 => {
                instruction_count = 4;
                parameter_one * parameter_two
            }
            3 => {
                instruction_count = 2;
                input
            }
            4 => {
                instruction_count = 2;
                return run_intcode(result_vec[parameter_two as usize], &input_vec);
            }
            _ => panic!("uncovered opcode found {:?} at {:?}", opcode, i),
        };

        i += instruction_count;
    }

    output
}

impl Instruction {
    fn new(input: i32) -> Instruction {
        Instruction {
            opcode: get_input_value(input, 0, 2),
            paramode_one: get_input_value(input, 2, 1),
            paramode_two: get_input_value(input, 3, 1),
            paramode_thr: get_input_value(input, 4, 1),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    paramode_one: i32,
    paramode_two: i32,
    paramode_thr: i32,
}

fn get_input_value(input: i32, position: u32, count: u32) -> i32 {
    (input / i32::pow(10, position)) % i32::pow(10, count)
}
