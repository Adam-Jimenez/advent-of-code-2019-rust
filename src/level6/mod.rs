use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::error::Error;


fn parse_input(file: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut map = HashMap::new();
    let f = File::open(file)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line.unwrap();
        let planets: Vec<String> = line.split(")").map(|s| s.to_string()).collect();
        let orbited = planets[0].to_owned();
        let orbiting = planets[1].to_owned();
        map.entry(orbiting.clone()).or_insert(vec![]);
        let children = map.entry(orbited).or_insert(vec![]);
        children.push(orbiting);
    }
    Ok(map)
}

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let root = "COM".to_string();
    let input_map = parse_input("src/level6/input.txt")?;
    Ok(visit(root, 0, &input_map))
}

fn visit(node: String, depth: u32, map: &HashMap<String, Vec<String>>) -> u32 {
    let depth = depth + 1;
    let mut sum = 0;
    for child in &map[&node] {
        sum += depth;
        sum += visit(child.to_string(), depth, &map);
    }
    sum
}
