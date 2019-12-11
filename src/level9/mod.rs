use std::fs;
use std::error::Error;
use std::io;
use std::io::stdin;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct ExecError {}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Execution error")
    }
}
impl Error for ExecError{}

struct Operation {
    opcode: u32,
    param_mode_1: u32,
    param_mode_2: u32,
    param_mode_3: u32,
}


pub struct Computer {
    eip: usize,
    relative_base: i64,
    memory: Vec<i64>,
    backup_memory: HashMap<usize, i64>
}

impl Computer {
    pub fn new(code_path: &str) -> Self {
        Computer { 
            eip: 0,
            relative_base:0,
            memory: Computer::parse_code(code_path).unwrap(),
            backup_memory: HashMap::new()
        }
    }
    fn parse_opcode(opcode: i64) -> Operation {
        let mut args = [0,0,0,0,0];
        let arg_len = args.len();
        for (i, c) in opcode.to_string().chars().rev().enumerate() {
            args[arg_len-1-i] = c.to_digit(10).unwrap();
        }
        Operation {
            opcode: args[3]*10 + args[4],
            param_mode_1: args[2],
            param_mode_2: args[1],
            param_mode_3: args[0]
        }
    }

    fn get_parameter(&self, mode: u32, param: i64) -> i64 {
        if mode == 0 {
            return self.read_memory( param as usize );
        }
        else if mode == 1 {
            return param
        }
        else if mode == 2 {
            return self.read_memory( (self.relative_base + param) as usize )
        }
        else {
            panic!("Invalid mode");
        }
    }

    pub fn parse_code(path: &str) -> Result<Vec<i64>, Box<dyn Error>> {
        let code: String = fs::read_to_string(path)?;
        Ok(code.trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect())
    }
    fn read_memory(&self, address: usize) -> i64 {
        if address < self.memory.len() {
            return self.memory[address];
        } else {
            return self.backup_memory.get(&address).cloned().unwrap_or(0);
        }
    }
    fn write_memory(&mut self, address: usize, value: i64) {
        if address < self.memory.len() {
            self.memory[address] = value;
        } else {
            self.backup_memory.insert(address, value);
        }
    }
    pub fn execute(&mut self, input_params: &mut VecDeque<i64>) -> Result<(VecDeque<i64>), Box<dyn Error>> {
        let mut output_params: VecDeque<i64> = VecDeque::new();
        while self.eip < self.memory.len() {
            let mem_debug = self.memory.iter().map(|d| d.to_string()).collect::<Vec<String>>();
            let offset: usize = mem_debug[..self.eip].iter().map(|x| x.len()).fold(0, |acc, val| acc + val) + self.eip;
            // println!("{}", mem_debug.join(" "));
            // println!("{}^", " ".repeat(offset));
            // println!("eip: {}", self.eip);
            let operation = Computer::parse_opcode(self.memory[self.eip]);
            // 1 = ADD 1 2 into 3
            if operation.opcode == 1 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory(self.eip+1));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                let mut num_3 = self.read_memory( self.eip+3 );
                if operation.param_mode_3 == 2 {
                    num_3 = num_3 as i64 + self.relative_base;
                }
                // println!("ADD {} {} @ {}", num_1, num_2, num_3);
                self.write_memory( num_3 as usize, num_1 + num_2);
                self.eip += 4;
            }
            // 2 = MULTIPLY 1 2 into 3
            else if operation.opcode == 2 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                let mut num_3 = self.read_memory( self.eip+3 );
                if operation.param_mode_3 == 2 {
                    num_3 = num_3 as i64 + self.relative_base;
                }
                // println!("MULTIPLY {} {} @ {}", num_1, num_2, num_3);
                self.write_memory(num_3 as usize, num_1 * num_2);
                self.eip += 4;
            }
            // 3 = INPUT INTO ADDRESS 1
            else if operation.opcode == 3 {
                let mut num_1 = self.read_memory( self.eip+1 );
                if operation.param_mode_1 == 2 {
                    num_1 = num_1 as i64 + self.relative_base;
                }
                let input_value: i64 = input_params.pop_front().unwrap();
                // println!("Input: {}", input_value);
                self.write_memory( num_1 as usize, input_value);
                self.eip += 2;
            }
            // 4 = OUTPUT ADDRESS 1
            else if operation.opcode == 4 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                output_params.push_back(num_1);
                self.eip += 2;
                // println!("Output: {}", num_1);
                return Ok(output_params);
            }
            // 5 = JUMP TO 2 if 1 != 0
            else if operation.opcode == 5 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                // println!("IF {} != 0, JUMP TO {}", num_1, num_2);
                if num_1 != 0 {
                    self.eip = num_2 as usize;
                } else {
                    self.eip += 3;
                }

            }
            // 6 = JUMP TO 2 if 1 == 0
            else if operation.opcode == 6 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                // println!("IF {} == 0, JUMP TO {}", num_1, num_2);
                if num_1 == 0 {
                    self.eip = num_2 as usize;
                } else {
                    self.eip += 3;
                }

            }
            // 7 = LESS THAN 1 < 2, to 3
            else if operation.opcode == 7 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                let mut num_3 = self.read_memory( self.eip+3 ) as usize;
                if operation.param_mode_3 == 2 {
                    num_3 = (num_3 as i64 + self.relative_base) as usize;
                }
                // println!("{} < {} into {}", num_1, num_2, num_3);
                if num_1 < num_2 {
                    self.write_memory( num_3, 1);
                } else {
                    self.write_memory( num_3, 0);
                }
                self.eip += 4;
            }
            // 8 = 1 == 2 to 3
            else if operation.opcode == 8 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                let num_2 = self.get_parameter(operation.param_mode_2, self.read_memory( self.eip+2 ));
                let mut num_3 = self.read_memory( self.eip+3 ) as usize;
                if operation.param_mode_3 == 2 {
                    num_3 = (num_3 as i64 + self.relative_base) as usize;
                }
                // println!("{} == {} into {}", num_1, num_2, num_3);
                if num_1 == num_2 {
                    self.write_memory( num_3  , 1);
                } else {
                    self.write_memory( num_3, 0);
                }
                self.eip += 4;
            }
            // increment relative base
            else if operation.opcode == 9 {
                let num_1 = self.get_parameter(operation.param_mode_1, self.read_memory( self.eip+1 ));
                self.relative_base += num_1;
                // println!("INCREMENT RELATIVE BASE BY {} = {}", num_1, self.relative_base);
                self.eip += 2;
            }
            // 99 = HALT
            else if operation.opcode == 99 {
                println!("HALT");
                return Ok(output_params);
            }

            else {
                // invalid opcode
                println!("INVALID OPCODE {}", operation.opcode);
                return Err(Box::new(ExecError{}));
            }
            // io::stdin().read_line(&mut String::new());
        }
        Ok(output_params)
    }
}

pub fn part1() {
    let mut computer = Computer { 
        eip: 0,
        relative_base:0,
        memory: Computer::parse_code("src/level9/input.txt").unwrap(),
        backup_memory: HashMap::new()
    };
    let mut input = VecDeque::new();
    input.push_back(2);
    println!("{:?}", computer.execute(&mut input));
}
