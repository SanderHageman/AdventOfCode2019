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

    let mut i = 0;
    while i < result_vec.len() {
        let opcode = Instruction::new(i, &result_vec);
        if opcode.opcode == 99 {
            break;
        };

        if opcode.opcode > 4 || opcode.opcode < 0 {
            panic!(
                "opcode out of range {:?} with input {:?} and i {:?}",
                opcode, input, i
            );
        }

        let instruction_count;
        match opcode.opcode {
            1 => {
                instruction_count = 4;
                let set_index = result_vec[i + 3] as usize;

                let param_one = opcode.get_parameter_one(&result_vec);
                let param_two = opcode.get_parameter_two(&result_vec);
                result_vec[set_index] = param_one + param_two;
            }
            2 => {
                instruction_count = 4;
                let set_index = result_vec[i + 3] as usize;

                let param_one = opcode.get_parameter_one(&result_vec);
                let param_two = opcode.get_parameter_two(&result_vec);
                result_vec[set_index] = param_one * param_two;
            }
            3 => {
                instruction_count = 2;
                let set_index = result_vec[i + 1] as usize;

                result_vec[set_index] = input;
            }
            4 => {
                instruction_count = 2;

                let next_op = Instruction::new(i + instruction_count, &result_vec);
                if next_op.opcode == 99 {
                    return opcode.get_parameter_one(&result_vec);
                }
            }
            _ => panic!("uncovered opcode found {:?} at {:?}", opcode, i),
        };

        i += instruction_count;
    }

    panic!("unable to find code")
}

impl Instruction {
    fn new(index: usize, result_vec: &Vec<i32>) -> Instruction {
        Instruction {
            instruction_index: index,
            opcode: get_input_value(result_vec[index], 0, 2),
            paramode_one: get_input_value(result_vec[index], 2, 1),
            paramode_two: get_input_value(result_vec[index], 3, 1),
            paramode_thr: get_input_value(result_vec[index], 4, 1),
        }
    }

    fn get_parameter_one(self, result_vec: &Vec<i32>) -> i32 {
        let offset = 1;
        match self.paramode_one {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }

    fn get_parameter_two(self, result_vec: &Vec<i32>) -> i32 {
        let offset = 2;
        match self.paramode_two {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }

    fn _get_parameter_three(self, result_vec: &Vec<i32>) -> i32 {
        let offset = 3;
        match self.paramode_thr {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    instruction_index: usize,
    opcode: i32,
    paramode_one: i32,
    paramode_two: i32,
    paramode_thr: i32,
}

fn get_input_value(input: i32, position: u32, count: u32) -> i32 {
    (input / i32::pow(10, position)) % i32::pow(10, count)
}
