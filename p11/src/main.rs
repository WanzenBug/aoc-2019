use std::{
    error::Error,
};
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../INPUT");

#[derive(Debug, Default)]
struct OrbitInfo {
    direct_orbit: Option<String>,
}

#[derive(Debug)]
struct OrbitMap(HashMap<String, OrbitInfo>);

impl FromStr for OrbitMap {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, OrbitInfo> = HashMap::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue
            }

            let mut split = line.split(")");
            let center = split.next().ok_or_else(||"Invalid format")?;
            let orbiter = split.next().ok_or_else(||"Invalid format")?;

            map.entry(center.to_string())
                .or_default();

            let orbentry = map.entry(orbiter.to_string())
                .or_default();
            orbentry.direct_orbit = Some(center.to_string());
        }

        Ok(OrbitMap(map))
    }
}

fn run(input: &str) -> Result<usize, Box<dyn Error + 'static>> {
    let map: OrbitMap = input.parse()?;

    let mut sum = 0;
    for mut entry in map.0.values() {
        while let Some(next) = entry.direct_orbit.as_ref() {
            sum += 1;
            entry = map.0.get(next).ok_or_else(|| "Parent entry not found")?;
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let res = run(INPUT)?;
    eprintln!("res = {:#?}", res);
    Ok(())
}

#[test]
fn test_all() {
    const INP: &str = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

    assert_eq!(run(INP).unwrap(), 42);
}
