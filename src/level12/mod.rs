use std::fs;
use std::error::Error;
use std::collections::HashSet;
use std::cmp::{max, min};
 
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
 
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
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
    let mut states: [HashSet<Vec<(i32, i32)>>; 3] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new()
    ];
    for i in 0..3 {
        states[i].insert(moons.iter().map(|m| (m.coordinates[i], m.velocity[i])).collect());
    }
    let mut cycles = [0,0,0];
    for i in 1.. {
        step(&mut moons);
        for j in 0..3 {
            if states[j].contains(&moons.iter().map(|m| (m.coordinates[j], m.velocity[j])).collect::<Vec<(i32, i32)>>()) {
                cycles[j] = i;
            }
        }
        if cycles.iter().all(|&x| x != 0) {
            break;
        }
    }
    println!("{:?}", cycles);
    println!("{}", lcm(lcm(cycles[1], cycles[2]), cycles[0])/2);
    Ok(())
}
