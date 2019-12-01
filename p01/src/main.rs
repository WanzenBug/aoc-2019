use std::{
    io::BufRead,
    error::Error
};

const INPUT: &[u8] = include_bytes!("../INPUT");

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut total_fuel = 0;
    for line in INPUT.lines() {
        let mass = line?.parse()?;
        total_fuel += fuel_from_mass(mass);
    }
    println!("{}", total_fuel);
    Ok(())
}

fn fuel_from_mass(mass: usize) -> usize {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuel_calc() {
        let mass_inputs = [12, 14, 1969, 100756];
        let expected_outputs = [2, 2, 654, 33583];

        for (&input, &output) in Iterator::zip(mass_inputs.iter(), expected_outputs.iter()) {
            assert_eq!(fuel_from_mass(input), output);
        }
    }
}
