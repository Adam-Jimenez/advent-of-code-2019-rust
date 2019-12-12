use std::fs;
use std::error::Error;

#[derive(PartialEq, Eq, Debug)]
struct Moon {
    coordinates: [i32; 3],
    velocity: [i32; 3]
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            coordinates: [x, y ,z],
            velocity: [0;3]
        }
    }
}

fn step(moons: &mut Vec<Moon>) {
    for ptr in 0..moons.len() {
        let mut velocity = moons[ptr].velocity;
        for moon2 in moons.iter() {
            if moons[ptr] == *moon2 {
                continue;
            }
            for i in 0..3 {
                if moons[ptr].coordinates[i] < moon2.coordinates[i] {
                    velocity[i] += 1
                } else if moons[ptr].coordinates[i] > moon2.coordinates[i] {
                    velocity[i] -= 1
                } else {
                    // no-op
                }
            }
        }
        let mut moon1 = &mut moons[ptr];
        moon1.velocity = velocity;
    }
    for moon in moons {
        for i in 0..3 {
            moon.coordinates[i] += moon.velocity[i];
        }
    }
}

fn energy(moons: &Vec<Moon>) -> i64 {
    let mut sum: i64 = 0;
    for moon in moons {
        let mut pot = 0;
        let mut kin = 0;
        for i in 0..3 {
            pot += moon.coordinates[i].abs();
            kin += moon.velocity[i].abs();
        }
        sum += pot as i64 * kin as i64;
    }
    sum
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/level12/input.txt")?;
    let charset = "0123456789-,\n".to_string();
    let input: String = input.chars().filter(|&c| charset.contains(c)).collect();
    let input: Vec<Vec<i32>> = input.trim().split("\n").map(|l| l.trim().split(",").map(|n| n.parse().unwrap()).collect()).collect();
    let mut moons: Vec<Moon> = input.iter().map(|x| Moon::new(x[0], x[1], x[2])).collect();
    for _ in 0..1000 {
        println!("{:?}", moons);
        step(&mut moons);
    }
    println!("{}", energy(&moons));
    Ok(())
}
