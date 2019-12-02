use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn calculate_fuel(mass: f32) -> f32 {
    let fuel = (mass / 3.0).floor() - 2.0;
    if fuel <= 0.0 { fuel }
    else { fuel + calculate_fuel(fuel) }
}

fn main() -> io::Result<()> {
    let file = File::open("day1input.txt")?;
    let required_fuel: f32 = BufReader::new(file)
        .lines()
        .map(|mass| calculate_fuel(mass.unwrap().parse::<f32>().unwrap()))
        .sum();
    println!("Required fuel: {}", required_fuel);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(100756.0), 50346.0);
    }
}
