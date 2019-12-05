mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let fuel_mass: u32 = level1::part1()?;
    // println!("{}", fuel_mass);
    // let fuel_mass: u32 = level1::part2()?;
    // println!("{}", fuel_mass);

    // println!("{:?}", level2::part2());
    // level3::part2();
    // level4::part1();
    // level4::part2();
    level5::part2()?;
    Ok(())
}
