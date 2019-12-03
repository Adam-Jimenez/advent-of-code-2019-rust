use std::fs;
use std::error::Error;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/level3/input.txt")?;
    let wires: Vec<Vec<&str>> = input.trim()
        .split("\n")
        .map(|wire| wire.split(",").collect())
        .collect();
    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let points = get_wire_points(&wire1);
    let points2 = get_wire_points(&wire2);
    let intersections: HashSet<&Point> = points.intersection(&points2).collect();
    let distance1 = get_point_distance(wire1, &intersections);
    let distance2 = get_point_distance(wire2, &intersections);
    let sum_distance: Vec<i32> = intersections.iter()
        .map(|point| distance1[point]+distance2[point])
        .collect();
    println!("{:?}", sum_distance);
    let mut min_index = 0;
    let mut min = sum_distance[0];
    for (i, &val) in sum_distance.iter().enumerate() {
        if val < min {
            min_index = 0;
            min = val;
        }
    }
    println!("{}", min);
    Ok(())
}

fn get_wire_points(wire: &Vec<&str>) -> HashSet<Point> {
    let mut current_position: Point = Point { x: 0, y: 0 };
    let mut points = HashSet::new();
    for translation in wire {
        let direction: char = translation.chars().nth(0).unwrap();
        let norm: i32 = translation[1..].parse().unwrap();
        for _ in 0..norm {
            if direction == 'R' {
                current_position.x += 1;
            } else if direction == 'L' {
                current_position.x -= 1;
            } else if direction == 'U' {
                current_position.y += 1;
            } else if direction == 'D' {
                current_position.y -= 1;
            } else {
                panic!("Invalid direction")
            }
            points.insert(current_position);
        }
    }
    points
}

fn get_point_distance(wire: &Vec<&str>, points: &HashSet<&Point>) -> HashMap<Point, i32> {
    let mut current_position: Point = Point { x: 0, y: 0 };
    let mut steps = 0;
    let mut point_steps = HashMap::new();
    for translation in wire {
        let direction: char = translation.chars().nth(0).unwrap();
        let norm: i32 = translation[1..].parse().unwrap();
        for _ in 0..norm {
            if direction == 'R' {
                current_position.x += 1;
            } else if direction == 'L' {
                current_position.x -= 1;
            } else if direction == 'U' {
                current_position.y += 1;
            } else if direction == 'D' {
                current_position.y -= 1;
            } else {
                panic!("Invalid direction")
            }
            steps += 1;
            if points.contains(&current_position) {
                point_steps.insert(current_position, steps);
            }
        }
    }
    return point_steps;
}
