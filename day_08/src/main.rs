use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO:
// Errors should not be constructed directly from a str

#[derive(Debug, Clone, Copy)]
enum OperationCode {
    NOP,
    ACC,
    JMP
}

struct Instruction {
    operation : OperationCode,
    argument : Option<i32>,
    executed : bool,
}

impl Instruction {
    pub fn new(operation : OperationCode, argument : Option<i32>) -> Instruction {
        Instruction {
            operation : operation,
            argument : argument,
            executed : false,
        }
    }

    fn set_executed(&mut self, value : bool) {
        self.executed = value;
    }

    fn get_operation(&mut self) -> OperationCode {
        return self.operation.clone();
    }

    fn set_operation(&mut self, value : OperationCode) {
        self.operation = value;
    }

    fn run_instruction(&mut self, mut current_pointer : u32, mut accumulator : i32) -> Result<(u32, i32), &'static str> {
        match self.operation {
            OperationCode::NOP => {
                if self.executed { return Err("Instruction already executed")}

                current_pointer = current_pointer + 1;
            }
            OperationCode::JMP => {
                if self.executed { return Err("Instruction already executed")}

                let argument_value : i32 = match self.argument {
                    Some(value) => value,
                    None => return Err("ACC Command not given an argument"),
                };

                current_pointer = (current_pointer as i32 + argument_value) as u32;
            }
            OperationCode::ACC => {
                if self.executed { return Err("Instruction already executed")}

                let argument_value : i32 = match self.argument {
                    Some(value) => value,
                    None => return Err("ACC Command not given an argument"),
                };

                accumulator = accumulator + argument_value;
                current_pointer = current_pointer + 1;
            },
        }

        self.executed = true;

        return Ok((current_pointer, accumulator));
    }
}

struct Machine {
    current_pointer : u32,
    accumulator : i32,
    instructions : Vec<Instruction>
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            current_pointer : 0,
            accumulator : 0,
            instructions : Vec::new()
        }
    }

    fn add_instruction(&mut self, code : &str, argument : i32) -> Result<&'static str, &'static str> {
        let operation_code : OperationCode = match code {
            "nop" => OperationCode::NOP,
            "acc" => OperationCode::ACC,
            "jmp" => OperationCode::JMP,
            _ => return Err("No such operation found!")
        };

        let new_instruction : Instruction = Instruction::new(operation_code, Some(argument));
        self.instructions.push(new_instruction);
        return Ok("Instruction added successfully");
    }

    fn run_machine(&mut self, on_loop_throw_error : bool) -> Result<i32, &'static str> {

        loop {
            let index : usize= self.current_pointer as usize;
            let result : Result<(u32, i32), &'static str> = self.instructions[index]
                .run_instruction(self.current_pointer, self.accumulator);

            let new_values : (u32, i32) = match result {
                Ok(value) => value,
                Err("Instruction already executed") => {
                    if !on_loop_throw_error { break; }
                    else { return Err("Loop found!"); }
                },
                Err(e) => return Err(e),
            };
            
            self.current_pointer = new_values.0;
            self.accumulator = new_values.1;

            if self.current_pointer == self.instructions.len() as u32 { break; }
        }

        return Ok(self.accumulator);
    }

    fn fix_machine(&mut self) -> Result<i32, &'static str> {
        let number_of_instruction : usize = self.instructions.len();
        let mut accumulator_fixed : Option<i32> = None;

        for index in 0..number_of_instruction {
            let prev_code : OperationCode = self.instructions[index].get_operation();

            let next_code : OperationCode = match prev_code {
                OperationCode::ACC => continue,
                OperationCode::JMP => OperationCode::NOP,
                OperationCode::NOP => OperationCode::JMP,
            };

            self.instructions[index].set_operation(next_code);
            let result : Result<i32, &'static str> = self.run_machine(true);

            match result {
                Ok(value) => {
                    accumulator_fixed = Some(value);
                    break;
                },
                Err("Loop found!") => {
                    self.instructions[index].set_operation(prev_code);
                    self.reset_machine();
                },
                Err(e) => return Err(e),
            }
        }

        match accumulator_fixed {
            Some(value) => return Ok(value),
            None => return Err("This machine can't be fixed"),
        };
    }

    fn clean_executed(&mut self) {
        for instruction in self.instructions.iter_mut() {
            instruction.set_executed(false);
        }
    }

    fn reset_machine(&mut self) {
        self.clean_executed();
        self.current_pointer = 0;
        self.accumulator = 0;
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut machine_emulator = Machine::new();

    for line in data.iter() {
        let split : Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        
        let code : &str = split[0];
        let argument : &str = split[1];

        let argument : i32 = match argument.parse() {
            Ok(value) => value,
            Err(e) => panic!(e),
        };

        match machine_emulator.add_instruction(code, argument) {
            Ok(_) => (),
            Err(e) => panic!(e),
        };
    }

    let accumulator_value : i32 = match machine_emulator.run_machine(false) {
        Ok(value) => value,
        Err(e) => panic!(e)
    };

    println!("Machine emulator final accumulator value: {}", accumulator_value);
    machine_emulator.reset_machine();

    let accumulator_value : i32 = match machine_emulator.fix_machine() {
        Ok(value) => value,
        Err(e) => panic!(e)
    };

    println!("Machine emulator final accumulator value (with fix): {}", accumulator_value);
    machine_emulator.reset_machine();
}
