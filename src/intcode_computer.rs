#[derive(Debug)]
pub struct Computer {
    registers: Vec<i64>,
    input: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
    pub output: Option<i64>,
    pub stop: bool,
    pub pause: bool,
}

impl Computer {
    pub fn new(input: Vec<i64>, input_registers: &Vec<i64>, min_length: usize) -> Computer {
        let mut inputs = input.clone();
        inputs.reverse();

        let mut input = input_registers.to_vec();
        if min_length != 0 && input.len() <= min_length as usize {
            for _i in input.len()..=min_length as usize {
                input.push(0);
            }
        }

        let computer = Computer {
            registers: input,
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

    pub fn simple(input: Vec<i64>, input_registers: &Vec<i64>) -> i64 {
        Computer::new(input, input_registers, 0).compute()
    }

    pub fn compute(&mut self) -> i64 {
        while !self.stop {
            let opcode = Instruction::new(self.instruction_pointer, &self.registers);
            self.instruction_pointer += self.run_instruction(opcode);
        }

        self.output.unwrap_or(self.registers[0])
    }

    pub fn compute_til_output(&mut self) -> i64 {
        if self.pause {
            self.pause = false;
        }

        while !self.pause {
            let opcode = Instruction::new(self.instruction_pointer, &self.registers);
            self.instruction_pointer += self.run_instruction(opcode);
        }

        self.output.unwrap()
    }

    pub fn add_input(&mut self, input: i64) {
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

                let param_one = opcode.get_parameter_one(self.relative_base, &self.registers);
                let param_two = opcode.get_parameter_two(self.relative_base, &self.registers);

                let offset = 3i64;
                let set_index = match opcode.paramode_thr {
                    0 => self.registers[pointer + offset as usize] as usize,
                    2 => {
                        (self.registers[(pointer as i64 + offset) as usize] + self.relative_base)
                            as usize
                    }
                    _ => panic!("Unable to set for {:?}"),
                };

                self.registers[set_index] = param_one + param_two;
            }
            2 => {
                instruction_count = 4;

                let param_one = opcode.get_parameter_one(self.relative_base, &self.registers);
                let param_two = opcode.get_parameter_two(self.relative_base, &self.registers);

                let offset = 3i64;
                let set_index = match opcode.paramode_thr {
                    0 => self.registers[pointer + offset as usize] as usize,
                    2 => {
                        (self.registers[(pointer as i64 + offset) as usize] + self.relative_base)
                            as usize
                    }
                    _ => panic!("Unable to set for {:?}"),
                };

                self.registers[set_index] = param_one * param_two;
            }
            3 => {
                instruction_count = 2;

                let offset = 1i64;
                let set_index = match opcode.paramode_one {
                    0 => self.registers[pointer + offset as usize] as usize,
                    2 => {
                        (self.registers[(pointer as i64 + offset) as usize] + self.relative_base)
                            as usize
                    }
                    _ => panic!("Unable to set for {:?}"),
                };

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

                let offset = 3i64;
                let set_index = match opcode.paramode_thr {
                    0 => self.registers[pointer + offset as usize] as usize,
                    2 => {
                        (self.registers[(pointer as i64 + offset) as usize] + self.relative_base)
                            as usize
                    }
                    _ => panic!("Unable to set for {:?}"),
                };

                self.registers[set_index] = if set { 1 } else { 0 };
            }
            8 => {
                instruction_count = 4;
                let set = opcode.get_parameter_one(self.relative_base, &self.registers)
                    == opcode.get_parameter_two(self.relative_base, &self.registers);

                let offset = 3i64;
                let set_index = match opcode.paramode_thr {
                    0 => self.registers[pointer + offset as usize] as usize,
                    2 => {
                        (self.registers[(pointer as i64 + offset) as usize] + self.relative_base)
                            as usize
                    }
                    _ => panic!("Unable to set for {:?}"),
                };

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
    fn new(index: usize, result_vec: &Vec<i64>) -> Instruction {
        Instruction {
            instruction_index: index,
            opcode: Instruction::get_input_value(result_vec[index], 0, 2),
            paramode_one: Instruction::get_input_value(result_vec[index], 2, 1),
            paramode_two: Instruction::get_input_value(result_vec[index], 3, 1),
            paramode_thr: Instruction::get_input_value(result_vec[index], 4, 1),
        }
    }

    fn get_input_value(input: i64, position: u32, count: u32) -> i64 {
        (input / i64::pow(10, position)) % i64::pow(10, count)
    }

    fn get_parameter(
        self,
        offset: usize,
        paramode: i64,
        relative_base: i64,
        result_vec: &Vec<i64>,
    ) -> i64 {
        let index = self.instruction_index + offset;

        match paramode {
            0 => result_vec[result_vec[index] as usize],
            1 => result_vec[index],
            2 => result_vec[(result_vec[index] + relative_base) as usize],
            _ => panic!("uncovered parameter mode {:?}", self),
        }
    }

    fn get_parameter_one(self, relative_base: i64, result_vec: &Vec<i64>) -> i64 {
        self.get_parameter(1, self.paramode_one, relative_base, &result_vec)
    }

    fn get_parameter_two(self, relative_base: i64, result_vec: &Vec<i64>) -> i64 {
        self.get_parameter(2, self.paramode_two, relative_base, &result_vec)
    }

    fn _get_parameter_three(self, relative_base: i64, result_vec: &Vec<i64>) -> i64 {
        self.get_parameter(3, self.paramode_thr, relative_base, &result_vec)
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    instruction_index: usize,
    opcode: i64,
    paramode_one: i64,
    paramode_two: i64,
    paramode_thr: i64,
}
