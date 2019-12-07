use std::fs;
use std::error::Error;
use std::io;
use std::io::stdin;
use std::collections::VecDeque;
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

fn parse_opcode(opcode: i32) -> Operation {
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

fn get_parameter(mode: u32, param: i32, memory: &Vec<i32>) -> i32 {
    if mode == 0 {
        return memory[param as usize];
    }
    else if mode == 1 {
        return param
    }
    else {
        panic!("Invalid mode");
    }
}

fn parse_code() -> Result<Vec<i32>, Box<dyn Error>> {
    let code: String = fs::read_to_string("src/level7/input.txt")?;
    Ok(code.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect())
}

struct Computer {
    eip: usize,
    memory: Vec<i32>
}

impl Computer {
    pub fn execute(&mut self, input_params: &mut VecDeque<i32>) -> Result<(VecDeque<i32>), Box<dyn Error>> {
        let mut output_params: VecDeque<i32> = VecDeque::new();
        while self.eip < self.memory.len() {
            let mem_debug = self.memory.iter().map(|d| d.to_string()).collect::<Vec<String>>();
            let offset: usize = mem_debug[..self.eip].iter().map(|x| x.len()).fold(0, |acc, val| acc + val) + self.eip;
            println!("{}", mem_debug.join(" "));
            println!("{}^", " ".repeat(offset));
            let operation = parse_opcode(self.memory[self.eip]);
            // 1 = ADD 1 2 into 3
            if operation.opcode == 1 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                let num_3 = self.memory[self.eip+3];
                println!("ADD {} {} @ {}", num_1, num_2, num_3);
                self.memory[num_3 as usize] = num_1 + num_2;
                self.eip += 4;
            }
            // 2 = MULTIPLY 1 2 into 3
            else if operation.opcode == 2 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                let num_3 = self.memory[self.eip+3];
                println!("MULTIPLY {} {} @ {}", num_1, num_2, num_3);
                self.memory[num_3 as usize] = num_1 * num_2;
                self.eip += 4;
            }
            // 3 = INPUT INTO ADDRESS 1
            else if operation.opcode == 3 {
                let num_1 = self.memory[self.eip+1];
                let input_value: i32 = input_params.pop_front().unwrap();
                println!("Input: {}", input_value);
                self.memory[num_1 as usize] = input_value;
                self.eip += 2;
            }
            // 4 = OUTPUT ADDRESS 1
            else if operation.opcode == 4 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                output_params.push_back(num_1);
                self.eip += 2;
                println!("Output: {}", num_1);
                return Ok(output_params);
            }
            // 5 = JUMP TO 2 if 1 != 0
            else if operation.opcode == 5 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                println!("IF {} != 0, JUMP TO {}", num_1, num_2);
                if num_1 != 0 {
                    self.eip = num_2 as usize;
                } else {
                    self.eip += 3;
                }

            }
            // 6 = JUMP TO 2 if 1 == 0
            else if operation.opcode == 6 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                println!("IF {} == 0, JUMP TO {}", num_1, num_2);
                if num_1 == 0 {
                    self.eip = num_2 as usize;
                } else {
                    self.eip += 3;
                }

            }
            // 7 = LESS THAN 1 < 2, write 1 to 3
            else if operation.opcode == 7 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                let num_3 = self.memory[self.eip+3] as usize;
                println!("{} < {}", num_1, num_2);
                if num_1 < num_2 {
                    self.memory[num_3] = 1;
                } else {
                    self.memory[num_3] = 0;
                }
                self.eip += 4;
            }
            // 8 = LESS THAN 1 == 2, write 1 to 3
            else if operation.opcode == 8 {
                let num_1 = get_parameter(operation.param_mode_1, self.memory[self.eip+1], &self.memory);
                let num_2 = get_parameter(operation.param_mode_2, self.memory[self.eip+2], &self.memory);
                let num_3 = self.memory[self.eip+3] as usize;
                println!("{} == {}", num_1, num_2);
                if num_1 == num_2 {
                    self.memory[num_3] = 1;
                } else {
                    self.memory[num_3] = 0;
                }
                self.eip += 4;
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

pub fn try_amplifiers() -> Result<(), Box<dyn Error>>{
    let mut values = VecDeque::new();
    for i in 5..10 {
        values.push_back(i)
    }
    let permutations = permute(&mut vec![], &mut values);
    // let permutations = vec![vec![9,8,7,6,5]];
    let mut max = 0;
    for permutation in permutations {
        let mut out = 0;
        let mut computers: Vec<Computer> = permutation.iter().map(|i| Computer { eip: 0, memory: parse_code().unwrap() } ).collect();
        for i in (0..permutation.len()) {
            let phase = permutation[i];
            let mut input = VecDeque::new();
            input.push_back(phase);
            input.push_back(out);
            println!("Executing PC {}", i);
            let mut exec_out = computers[i].execute(&mut input)?;
            println!("Executing PC {} finished", i);
            out = exec_out.pop_front().unwrap();
        }
        for i in (0..permutation.len()).cycle() {
            let mut input = VecDeque::new();
            input.push_back(out);
            println!("Executing PC {}", i);
            let mut exec_out = computers[i].execute(&mut input)?;
            println!("Executing PC {} finished", i);
            out = if let Some(output) = exec_out.pop_front() {
                output
            } else {
                break
            }
        }
        max = std::cmp::max(max, out);
    }
    println!("{}", max);
    Ok(())
}

fn permute(used: &mut Vec<i32>, unused: &mut VecDeque<i32>) -> Vec<Vec<i32>> {
    if unused.is_empty() {
        return vec![used.clone()];
    } else {
        let mut permutations = Vec::new();
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permutations.extend(permute(used, unused));
            unused.push_back(used.pop().unwrap());
        }
        return permutations;
    }
}
