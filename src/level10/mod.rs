use std::fs;
use std::error::Error;
use std::f32::consts::PI;

pub fn part1() -> Result<(), Box<dyn Error>>{
    let input = fs::read_to_string("./src/level10/input.txt")?;
    let input: Vec<Vec<char>> = input.trim().split("\n").map(|l| l.chars().collect()).collect();
    let mut max = 0;
    for (i,l) in input.iter().enumerate() {
        for (j, &val) in l.iter().enumerate() {
            if val == '#' {
                let mut angles: Vec<f32> = vec![];
                for (m,l) in input.iter().enumerate() {
                    for (n, &val) in l.iter().enumerate() {
                        if val == '#' && !(i == m && j == n){
                            let h: f32 = m as f32 -i as f32;
                            let w: f32 = n as f32 -j as f32;
                            let angle = h.atan2(w);
                            if !angles.contains(&angle) {
                                angles.push(angle);
                            }
                        }
                    }
                }
                max = std::cmp::max(max, angles.len());
            }
        }
    }
    println!("{}", max);
    Ok(())
}
