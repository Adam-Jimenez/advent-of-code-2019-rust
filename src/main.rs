mod level1;
use std::io;

fn main() -> Result<(), io::Error> {
    level1::part1()?;
    Ok(())
}
