use crate::level9::Computer;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct PaintBot {
    direction: (i32, i32), // (i, j)
    position: (i32, i32) // (i, j)
}

impl PaintBot {
    pub fn new() -> Self {
        PaintBot {
            direction: (-1, 0),
            position: (0, 0)
        }
    }
    fn turn_left(&mut self) {
        let (i, j) = self.direction;
        self.direction = (-j, i);
    }
    fn turn_right(&mut self) {
        let (i, j) = self.direction;
        self.direction = (j, -i);
    }
    fn forward(&mut self) {
        let (i, j) = self.position;
        let (di, dj) = self.direction;
        self.position = (i+di, j+dj);
    }
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let mut grid: HashMap<(i32, i32), i64> = HashMap::new();
    grid.insert((0,0), 1);
    let mut painted: HashMap<(i32, i32), bool> = HashMap::new();
    let mut paint_bot = PaintBot::new();
    let mut states: HashSet<(PaintBot, i64)> = HashSet::new();
    let mut computer = Computer::new("src/level11/input.txt");
    loop {
        let mut input: VecDeque<i64> = VecDeque::new();
        let panel: i64 = *grid.entry(paint_bot.position).or_insert(0);
        input.push_back(panel);
        let mut output = computer.execute(&mut input)?;
        let paint = if let Some(paint) = output.pop_front() {
            paint
        } else {
            break
        };
        grid.insert(paint_bot.position, paint);
        if paint == 1 {
            painted.insert(paint_bot.position, true);
        }
        // if states.contains(&(paint_bot.clone(), paint)) {
        //     println!("{:?}", paint_bot);
        //     break;
        // }
        // states.insert((paint_bot.clone(), paint));
        let mut output = computer.execute(&mut input)?;
        let output = output.pop_front();
        match output {
            Some(0) => paint_bot.turn_left(),
            Some(1) => paint_bot.turn_right(),
            _ => break
        }
        paint_bot.forward();

    }
    draw(grid);
    Ok(())
}

fn draw(grid: HashMap<(i32, i32), i64>) {
    let mut min_i: i32 = 0;
    let mut min_j: i32 = 0;
    let mut max_i: i32 = 0;
    let mut max_j: i32 = 0;
    for &(i, j) in grid.keys() {
        if i < min_i {
            min_i = i;
        }
        if j < min_j {
            min_j = j;
        }
        if i > max_i {
            max_i = i;
        }
        if j > max_j {
            max_j = j;
        }
    }
    let h = max_i - min_i;
    let w = max_j - min_j;
    for i in 0..=h {
        for j in 0..=w {
            let val = *grid.get(&(i-min_i,j-min_j)).unwrap_or(&0);
            if val == 0 {
                print!(" ");
            } else {
                print!("{}", val);
            }
        }
        println!();
    }
}
