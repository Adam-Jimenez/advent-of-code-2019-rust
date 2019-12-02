use std::fs;
use std::error::Error;

pub fn part1() -> Result<(usize), Box<dyn Error>> {
    let code: String = fs::read_to_string("src/level2/input.txt")?;
    let mut opcodes: Vec<usize> = code.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;
    opcodes[1] = 12;
    opcodes[2] = 2;
    while i < opcodes.len() {
        // 1 = ADD 1 2 into 3
        if opcodes[i] == 1 {
            let num_1_location = opcodes[i+1];
            let num_2_location = opcodes[i+2];
            let sum = opcodes[num_1_location] + opcodes[num_2_location];
            let save_location = opcodes[i+3];
            opcodes[save_location] = sum;
            i += 4
        }
        // 2 = MULTIPLY 1 2 into 3
        else if opcodes[i] == 2 {
            let num_1_location = opcodes[i+1];
            let num_2_location = opcodes[i+2];
            let mult = opcodes[num_1_location] * opcodes[num_2_location];
            let save_location = opcodes[i+3];
            opcodes[save_location] = mult;
            i += 4
        }
        // 99 = HALT
        else if opcodes[i] == 99 {
            println!("HALT");
            return Ok((opcodes[0]));
        }
        else {
            panic!("Unknown opcode");
        }
    }
    Ok((opcodes[0]))
}
