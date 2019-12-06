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
                continue;
            }

            let mut split = line.split(")");
            let center = split.next().ok_or_else(|| "Invalid format")?;
            let orbiter = split.next().ok_or_else(|| "Invalid format")?;

            map.entry(center.to_string())
                .or_default();

            let orbentry = map.entry(orbiter.to_string())
                .or_default();
            orbentry.direct_orbit = Some(center.to_string());
        }

        Ok(OrbitMap(map))
    }
}

impl OrbitMap {
    fn get_centers_for(&self, entry: &str) -> Vec<&str> {
        let mut res = Vec::new();
        let mut current = entry;
        while let Some(center) = self.0.get(current).expect("Map must be consistent").direct_orbit.as_ref() {
            res.push(center.as_str());
            current = center.as_str()
        }
        res
    }
}

fn find_common_ancestor<'a, 'b, 'aa, 'bb>(left: &'a [&'aa str], right: &'b [&'bb str]) -> Option<(&'a [&'aa str], &'b [&'bb str])> {
    for (i, lval) in left.iter().enumerate() {
        for (j, rval) in right.iter().enumerate() {
            if lval == rval {
                return Some((&left[..i + 1], &right[..j+1]))
            }
        }
    }

    None
}

fn run(input: &str) -> Result<usize, Box<dyn Error + 'static>> {
    let map: OrbitMap = input.parse()?;

    let santa_orbits = map.get_centers_for("SAN");
    let you_orbits = map.get_centers_for("YOU");

    let (santa_to_common, you_to_common) = find_common_ancestor(&santa_orbits, &you_orbits).ok_or_else(|| "COM must be common orbit!")?;

    Ok(santa_to_common.len() + you_to_common.len() - 2)
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
K)YOU
I)SAN
";

    assert_eq!(run(INP).unwrap(), 4);
}


#[test]
fn test_find_common_ancestor() {
    let a = ["foo", "bar", "baz"];
    let b = ["b", "ba", "bar"];

    let (left, right) = find_common_ancestor(&a, &b).unwrap();
    assert_eq!(&left, &["foo", "bar"]);
    assert_eq!(&right, &["b", "ba", "bar"]);
}
