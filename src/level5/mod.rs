use std::fs;
use std::error::Error;
use std::io;

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

pub fn part2() -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string("src/level5/input.txt")?;
    let mut opcodes: Vec<i32> = code.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;
    while i < opcodes.len() {
        let mem_debug = opcodes.iter().map(|d| d.to_string()).collect::<Vec<String>>();
        let offset: usize = mem_debug[..i].iter().map(|x| x.len()).fold(0, |acc, val| acc + val) + i;
        println!("{}", mem_debug.join(" "));
        println!("{}^", " ".repeat(offset));
        let operation = parse_opcode(opcodes[i]);
        // 1 = ADD 1 2 into 3
        if operation.opcode == 1 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            let num_3 = opcodes[i+3];
            opcodes[num_3 as usize] = num_1 + num_2;
            i += 4;
        }
        // 2 = MULTIPLY 1 2 into 3
        else if operation.opcode == 2 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            let num_3 = opcodes[i+3];
            opcodes[num_3 as usize] = num_1 * num_2;
            i += 4;
        }
        // 3 = INPUT INTO ADDRESS 1
        else if operation.opcode == 3 {
            let num_1 = opcodes[i+1];
            let mut input = String::new();
            println!("Input:");
            io::stdin().read_line(&mut input)?;
            let input_value: i32 = input.trim().parse()?;
            opcodes[num_1 as usize] = input_value;
            i += 2;
        }
        // 4 = OUTPUT ADDRESS 1
        else if operation.opcode == 4 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            println!("Output: {}", num_1);
            i += 2;
        }
        // 5 = JUMP TO 2 if 1 != 0
        else if operation.opcode == 5 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            println!("IF {} != 0, JUMP TO {}", num_1, num_2);
            if num_1 != 0 {
                i = num_2 as usize;
            } else {
                i += 3;
            }

        }
        // 6 = JUMP TO 2 if 1 == 0
        else if operation.opcode == 6 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            println!("IF {} == 0, JUMP TO {}", num_1, num_2);
            if num_1 == 0 {
                i = num_2 as usize;
            } else {
                i += 3;
            }

        }
        // 7 = LESS THAN 1 < 2, write 1 to 3
        else if operation.opcode == 7 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            let num_3 = opcodes[i+3] as usize;
            println!("{} < {}", num_1, num_2);
            if num_1 < num_2 {
                opcodes[num_3] = 1;
            } else {
                opcodes[num_3] = 0;
            }
            i += 4;
        }
        // 8 = LESS THAN 1 == 2, write 1 to 3
        else if operation.opcode == 8 {
            let num_1 = get_parameter(operation.param_mode_1, opcodes[i+1], &opcodes);
            let num_2 = get_parameter(operation.param_mode_2, opcodes[i+2], &opcodes);
            let num_3 = opcodes[i+3] as usize;
            if num_1 == num_2 {
                opcodes[num_3] = 1;
            } else {
                opcodes[num_3] = 0;
            }
            i += 4;
        }
        // 99 = HALT
        else if operation.opcode == 99 {
            println!("HALT");
            println!("{}", opcodes[0]);
            return Ok(());
        }
        else {
            // invalid opcode
            println!("INVALID OPCODE {}", operation.opcode);
            return Ok(());
        }
    }
    Ok(())
}
