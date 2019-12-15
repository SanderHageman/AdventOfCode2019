#[derive(Debug)]
pub struct Computer {
    registers: Vec<i32>,
    input: Vec<i32>,
    instruction_pointer: usize,
    relative_base: i32,
    pub output: Option<i32>,
    pub stop: bool,
    pub pause: bool,
}

impl Computer {
    pub fn new(input: Vec<i32>, input_registers: &Vec<i32>) -> Computer {
        let mut inputs = input.clone();
        inputs.reverse();

        let computer = Computer {
            registers: input_registers.to_vec(),
            input: inputs,
            instruction_pointer: 0,
            relative_base: 0,
            output: None,
            stop: false,
            pause: false,
        };

        computer
    }

    pub fn empty() -> Computer {
        Computer {
            registers: Vec::new(),
            input: Vec::new(),
            instruction_pointer: 0,
            relative_base: 0,
            output: None,
            stop: false,
            pause: false,
        }
    }

    pub fn simple(input: Vec<i32>, input_registers: &Vec<i32>) -> i32 {
        Computer::new(input, input_registers).compute()
    }

    pub fn compute(&mut self) -> i32 {
        while !self.stop {
            let opcode = Instruction::new(self.instruction_pointer, &self.registers);
            self.instruction_pointer += self.run_instruction(opcode);
        }

        self.output.unwrap_or(self.registers[0])
    }

    pub fn compute_til_output(&mut self) -> i32 {
        if self.pause {
            self.pause = false;
        }

        while !self.pause {
            let opcode = Instruction::new(self.instruction_pointer, &self.registers);
            self.instruction_pointer += self.run_instruction(opcode);
        }

        self.output.unwrap()
    }

    pub fn add_input(&mut self, input: i32) {
        self.input.insert(0, input)
    }

    fn run_instruction(&mut self, opcode: Instruction) -> usize {
        if (opcode.opcode > 9 || opcode.opcode < 0) && opcode.opcode != 99 {
            panic!("opcode out of range {:?}", opcode);
        }

        let pointer = self.instruction_pointer;
        let instruction_count;

        match opcode.opcode {
            1 => {
                instruction_count = 4;
                let set_index = self.registers[pointer + 3] as usize;

                let param_one = opcode.get_parameter_one(self.relative_base, &self.registers);
                let param_two = opcode.get_parameter_two(self.relative_base, &self.registers);
                self.registers[set_index] = param_one + param_two;
            }
            2 => {
                instruction_count = 4;
                let set_index = self.registers[pointer + 3] as usize;

                let param_one = opcode.get_parameter_one(self.relative_base, &self.registers);
                let param_two = opcode.get_parameter_two(self.relative_base, &self.registers);
                self.registers[set_index] = param_one * param_two;
            }
            3 => {
                instruction_count = 2;
                let set_index = self.registers[pointer + 1] as usize;
                self.registers[set_index] = self.input.pop().unwrap();
            }
            4 => {
                instruction_count = 2;

                self.output = Some(opcode.get_parameter_one(self.relative_base, &self.registers));
                self.pause = true;

                let next_op = Instruction::new(pointer + instruction_count, &self.registers);
                if next_op.opcode == 99 {
                    self.stop = true;
                }
            }
            5 => {
                if opcode.get_parameter_one(self.relative_base, &self.registers) != 0 {
                    self.instruction_pointer =
                        opcode.get_parameter_two(self.relative_base, &self.registers) as usize;
                    instruction_count = 0;
                } else {
                    instruction_count = 3;
                }
            }
            6 => {
                if opcode.get_parameter_one(self.relative_base, &self.registers) == 0 {
                    self.instruction_pointer =
                        opcode.get_parameter_two(self.relative_base, &self.registers) as usize;
                    instruction_count = 0;
                } else {
                    instruction_count = 3;
                }
            }
            7 => {
                instruction_count = 4;
                let set = opcode.get_parameter_one(self.relative_base, &self.registers)
                    < opcode.get_parameter_two(self.relative_base, &self.registers);

                let set_index = self.registers[pointer + 3] as usize;
                self.registers[set_index] = if set { 1 } else { 0 };
            }
            8 => {
                instruction_count = 4;
                let set = opcode.get_parameter_one(self.relative_base, &self.registers)
                    == opcode.get_parameter_two(self.relative_base, &self.registers);

                let set_index = self.registers[pointer + 3] as usize;
                self.registers[set_index] = if set { 1 } else { 0 };
            }
            9 => {
                instruction_count = 2;
                self.relative_base += opcode.get_parameter_one(self.relative_base, &self.registers);
            }
            99 => {
                instruction_count = 0;
                self.stop = true;
            }
            _ => panic!("uncovered opcode found {:?} at {:?}", opcode, pointer),
        };

        instruction_count
    }
}

impl Instruction {
    fn new(index: usize, result_vec: &Vec<i32>) -> Instruction {
        Instruction {
            instruction_index: index,
            opcode: Instruction::get_input_value(result_vec[index], 0, 2),
            paramode_one: Instruction::get_input_value(result_vec[index], 2, 1),
            paramode_two: Instruction::get_input_value(result_vec[index], 3, 1),
            paramode_thr: Instruction::get_input_value(result_vec[index], 4, 1),
        }
    }

    fn get_input_value(input: i32, position: u32, count: u32) -> i32 {
        (input / i32::pow(10, position)) % i32::pow(10, count)
    }

    fn get_parameter_one(self, relative_base: i32, result_vec: &Vec<i32>) -> i32 {
        let offset = 1;
        match self.paramode_one {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            2 => {
                result_vec
                    [result_vec[self.instruction_index + offset + relative_base as usize] as usize]
            }
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }

    fn get_parameter_two(self, relative_base: i32, result_vec: &Vec<i32>) -> i32 {
        let offset = 2;
        match self.paramode_two {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            2 => {
                result_vec
                    [result_vec[self.instruction_index + offset + relative_base as usize] as usize]
            }
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }

    fn _get_parameter_three(self, relative_base: i32, result_vec: &Vec<i32>) -> i32 {
        let offset = 3;
        match self.paramode_thr {
            0 => result_vec[result_vec[self.instruction_index + offset] as usize],
            1 => result_vec[self.instruction_index + offset],
            2 => {
                result_vec
                    [result_vec[self.instruction_index + offset + relative_base as usize] as usize]
            }
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
