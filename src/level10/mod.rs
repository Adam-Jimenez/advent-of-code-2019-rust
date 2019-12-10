use std::fs;
use std::error::Error;
use std::f32::consts::PI;

fn parse_input() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let input = fs::read_to_string("./src/level10/input.txt")?;
    let input: Vec<Vec<char>> = input.trim().split("\n").map(|l| l.chars().collect()).collect();
    Ok(input)
}

pub fn part1() -> Result<((usize,usize)), Box<dyn Error>>{
    let input = parse_input()?;
    let mut max = 0;
    let mut pos = (0,0);
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
                if angles.len() > max {
                    max = angles.len();
                    pos = (i,j);
                }
                max = std::cmp::max(max, angles.len());
            }
        }
    }
    Ok((pos))
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input = parse_input()?;

    let pos = part1()?;
    let mut angles: Vec<(f32, f32, usize, usize)> = vec![];
    for (i,l) in input.iter().enumerate() {
        for (j, &val) in l.iter().enumerate() {
            if val == '#' && !(i == pos.0 && j == pos.1){
                let h: f32 = i as f32 - pos.0 as f32;
                let w: f32 = j as f32 - pos.1 as f32;
                let angle = h.atan2(w);
                angles.push((angle, (h*h + w*w).sqrt(), i, j));
            }
        }
    }
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut i = 0;
    while angles[i].0 < -PI/2f32 {
        i += 1;
    }
    let mut last = angles.remove(i);
    let mut cnt = 1;
    let mut deleted = vec![];
    loop {
        if angles.len() == 0 {
            break;
        }
        if angles[i % angles.len()].0 == last.0 && angles.len() != 1 {
            i += 1
        } else {
            i = i % angles.len();
            last = angles.remove(i % angles.len());
            deleted.push(last);
            cnt += 1;
            if cnt == 200 {
                break;
            }
        }
    }
    draw(input, deleted, pos);
    println!("{:?}", last.3 * 100 + last.2);
    Ok(())
}

fn draw (input: Vec<Vec<char>>, deleted: Vec<(f32, f32, usize, usize)>, pos: (usize, usize)) {
    for (_, _, i2, j2) in deleted {
        for (i,l) in input.iter().enumerate() {
            for (j, val) in l.iter().enumerate() {
                if i == i2 && j == j2 {
                    print!("\x1b[41m")
                } else if i == pos.0 && j == pos.1 {
                    print!("\x1b[42m")
                }
                print!("{}", val);
                print!("\x1b[0m")
            }
            println!();
        }
        std::io::stdin().read_line(&mut String::new());
        print!("\x1b[2J");
    }
}
