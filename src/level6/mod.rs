use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
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
        let planets: Vec<&str> = line.split(")").collect();
        let orbited = planets[0].to_owned();
        let orbiting = planets[1].to_owned();
        map.entry(orbiting.clone()).or_insert(vec![]);
        let children = map.entry(orbited).or_insert(vec![]);
        children.push(orbiting);
    }
    Ok(map)
}

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let root = "COM";
    let input_map = parse_input("src/level6/input.txt")?;
    Ok(visit(root, 0, &input_map))
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input_map = parse_input("src/level6/input.txt")?;
    let parent_map = parent_map(&input_map);
    let your_ancestors = get_ancestors("YOU", &parent_map);
    let san_ancestors = get_ancestors("SAN", &parent_map);
    let your_ancestors: HashSet<&str> = HashSet::from_iter(your_ancestors.iter().cloned());
    let san_ancestors: HashSet<&str> = HashSet::from_iter(san_ancestors.iter().cloned());
    let difference: HashSet<_> = your_ancestors.symmetric_difference(&san_ancestors).collect();
    println!("{:?}", difference);
    println!("{:?}", difference.len());
    Ok(())
}


fn get_ancestors<'a>(target_node: &str, parent_map: &'a HashMap<String, String>) -> Vec<&'a str> {
    let direct_parent = match parent_map.get(target_node) {
        Some(x) => x,
        None => return vec![]
    };
    let mut ancestors: Vec<&str> = vec![direct_parent];
    ancestors.extend(&get_ancestors(&direct_parent, &parent_map));
    ancestors
}
fn parent_map(map: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut parents: HashMap<String, String> = HashMap::new();
    for (parent, children) in map {
        for child in children {
            parents.insert(child.clone(), parent.clone());
        }
    }
    parents
}

fn visit(node: &str, depth: u32, map: &HashMap<String, Vec<String>>) -> u32 {
    let depth = depth + 1;
    let mut sum = 0;
    for child in &map[node] {
        sum += depth;
        sum += visit(child, depth, &map);
    }
    sum
}
