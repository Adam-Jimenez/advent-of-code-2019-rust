use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

pub fn part1() -> io::Result<(u32)> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass: u32 = line.parse().unwrap();
        result += mass / 3 - 2;
    }
    Ok(result)
}

pub fn part2() -> io::Result<(u32)> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass: u32 = line.parse().unwrap();
        let fuel_cost = mass / 3 - 2;
        let extra_fuel = extra_fuel(fuel_cost);
        result += fuel_cost;
        result += extra_fuel;
    }
    Ok(result)
}

fn extra_fuel(fuel_mass: u32) -> u32 {
    let mut rem: i32 = fuel_mass as i32;
    let mut extra_fuel: u32 = 0;
    while rem > 0 {
        rem = rem / 3 - 2;
        extra_fuel += cmp::max(rem, 0) as u32;
    }
    extra_fuel
}
