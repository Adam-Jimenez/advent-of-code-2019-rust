use std::error::Error;

fn check_ascending(digits: &Vec<u32>) -> bool {
    for (i, digit) in digits.iter().enumerate().skip(1) {
        if digit < &digits[i-1] {
            return false
        }
    }
    true
}

fn check_doubles(digits: &Vec<u32>) -> bool {
    for (i, digit) in digits.iter().enumerate().skip(1) {
        if digit == &digits[i-1] {
            return true
        }
    }
    false
}

fn check_sequences(digits: &Vec<u32>) -> bool {
    let mut counter = 0;
    for (i, digit) in digits.iter().enumerate().skip(1) {
        if digit == &digits[i-1] {
            counter += 1;
        } else {
            if counter == 1 {
                return true;
            }
            counter = 0;
        }
    }
    return counter == 1
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let range = (367479..893698);
    let mut counter = 0;
    for n in range {
        let digits: Vec<u32> = n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if check_ascending(&digits) && check_doubles(&digits) {
            counter += 1;
        }
    }
    println!("{}", counter);
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let range = (367479..893698);
    let mut counter = 0;
    for n in range {
        let digits: Vec<u32> = n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if check_ascending(&digits) && check_sequences(&digits) {
            counter += 1;
        }
    }
    println!("{}", counter);
    Ok(())
}
