use crate::level9::Computer;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone)]
struct State {
    computer: Computer,
    tick: i32,
    screen: HashMap<(i64,i64), i64>,
    paddle: (i64, i64),
    ball: (i64, i64)
}
impl State {
    fn draw(&self) {
        draw(&self.screen);
        println!("Tick: {}", self.tick);
        println!("Paddle: {:?}", self.paddle);
        println!("Ball: {:?}", self.ball);
    }
}
pub fn part1() -> Result<(), Box<dyn Error>> {
    let mut joystick = VecDeque::new();
    let mut score = 0;
    let mut debug = false;
    let mut current_state: State = State { 
        computer: Computer::new("src/level13/input.txt"),
        tick: 0,
        screen: HashMap::new(),
        paddle: (0,0),
        ball: (0,0)
    };
    let mut states: HashMap<i32, State> = HashMap::new();
    let mut ball_positions: Vec<(i32, i64, i64)> = vec![];
    joystick.push_back(0);
    while let Some(x) = current_state.computer.execute(&mut joystick.clone())?.pop_front() {
        let y = current_state.computer.execute(&mut joystick.clone())?.pop_front().unwrap(); 
        let id = current_state.computer.execute(&mut joystick.clone())?.pop_front().unwrap(); 
        if x == -1 && y == 0 {
            score = id;
            continue;
        } else {
            if id == 3 {
                current_state.paddle = (x,y);
            } else if id == 4 {
                current_state.ball = (x,y);
                current_state.tick += 1;
            } 
            current_state.screen.insert((y,x), id);
        }
        if id != 4 {
            continue;
        }
        if current_state.ball.1 == current_state.paddle.1-1 {
            let (x,y) = current_state.ball;
            ball_positions = ball_positions.into_iter().filter(|(tick,_,_)| *tick != current_state.tick).collect();
            ball_positions.push((current_state.tick, x, y));
            ball_positions.sort();

        }
        if current_state.ball.1 == current_state.paddle.1+1 {
            let delta = ((current_state.paddle.0 - current_state.ball.0).abs() + 4) as i32;
            current_state = states.get(&(std::cmp::max(1, current_state.tick-delta))).unwrap().clone();
            debug = true;
        }
        joystick.clear();
        joystick.push_back(0);
        for &(tick, x, y) in &ball_positions {
            if current_state.tick > tick {
                continue;
            }
            println!("Target: {},{}", x,y);
            joystick.clear();
            if current_state.paddle.0 < x {
                joystick.push_back(1);
            } else if current_state.paddle.0 > x {
                joystick.push_back(-1);
            } else {
                joystick.push_back(0);
            }
            break;
        }
        states.insert(current_state.tick, current_state.clone());
        current_state.draw();
        if debug {
            println!("{:?}", ball_positions);
            // std::io::stdin().read_line(&mut String::new())?;
        }
    }
    println!("SCORE: {}", score);
    Ok(())
}

fn clear() {
    println!("\x1b[2J")
}
fn draw(grid: &HashMap<(i64, i64), i64>) {
    clear();
    let mut min_i: i64 = 0;
    let mut min_j: i64 = 0;
    let mut max_i: i64 = 0;
    let mut max_j: i64 = 0;
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
            } else if val == 1 {
                print!("X")
            } else if val == 2 {
                print!("X")
            } else if val == 3 {
                print!("_")
            } else if val == 4 {
                print!("O")
            } else {
                print!("{}", val);
            }
        }
        println!();
    }
}
