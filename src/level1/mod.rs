use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass: u32 = line.parse().unwrap();
        result += mass / 3 - 2;
    }
    println!("{}",result);
    Ok(())
}
