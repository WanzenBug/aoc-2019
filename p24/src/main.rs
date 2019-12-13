use std::collections::{HashMap};
use std::str::FromStr;

const INPUT: &str = include_str!("../INPUT");

type Error = Box<dyn std::error::Error + 'static>;

fn main() -> Result<(), Error> {
    let result = run(INPUT)?;
    println!("result = {}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize, Error> {
    let moons: Result<Vec<_>, _> = input.lines().filter(|s| !s.is_empty()).map(Moon::from_str).collect();
    let moons = moons?;

    let mut xmoons = [moons[0], moons[1], moons[2], moons[3]];
    let mut ymoons = [moons[0], moons[1], moons[2], moons[3]];
    let mut zmoons = [moons[0], moons[1], moons[2], moons[3]];

    let mut states = HashMap::new();
    states.clear();
    let mut counter = 0;
    let resx = loop {
        if counter % 1_000_000 == 0 {
            eprintln!("counter = {:#?}", counter);
        }

        if let Some(_) = states.insert(xmoons, counter) {
            break (counter);
        }
        advance_tick_x(&mut xmoons);
        counter += 1;
    };

    states.clear();
    let mut counter = 0;
    let resy = loop {
        if counter % 1_000_000 == 0 {
            eprintln!("counter = {:#?}", counter);
        }

        if let Some(_) = states.insert(ymoons, counter) {
            break (counter);
        }
        advance_tick_y(&mut ymoons);
        counter += 1;
    };

    states.clear();
    let mut counter = 0;
    let resz = loop {
        if counter % 1_000_000 == 0 {
            eprintln!("counter = {:#?}", counter);
        }

        if let Some(_) = states.insert(zmoons, counter) {
            break (counter);
        }
        advance_tick_z(&mut zmoons);
        counter += 1;
    };

    let f = (resx * resy) / gcd(resx, resy);
    let res = (resz * f) / gcd(resz, f);
    eprintln!("resx = {:#?}", resx);
    eprintln!("resy = {:#?}", resy);
    eprintln!("resz = {:#?}", resz);

    Ok(res)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn advance_tick_x(moons: &mut [Moon]) {
    for midx in 0..moons.len() {
        let (before_moons, rest_moons) = moons.split_at_mut(midx);
        let (to_update, after_moons) = rest_moons.split_first_mut().expect("non-empty input");
        for before in before_moons {
            to_update.apply_gravity_force_x(before);
        }
        for after in after_moons {
            to_update.apply_gravity_force_x(after);
        }
    }

    for m in moons {
        m.advance_x()
    }
}

fn advance_tick_y(moons: &mut [Moon]) {
    for midx in 0..moons.len() {
        let (before_moons, rest_moons) = moons.split_at_mut(midx);
        let (to_update, after_moons) = rest_moons.split_first_mut().expect("non-empty input");
        for before in before_moons {
            to_update.apply_gravity_force_y(before);
        }
        for after in after_moons {
            to_update.apply_gravity_force_y(after);
        }
    }

    for m in moons {
        m.advance_y()
    }
}

fn advance_tick_z(moons: &mut [Moon]) {
    for midx in 0..moons.len() {
        let (before_moons, rest_moons) = moons.split_at_mut(midx);
        let (to_update, after_moons) = rest_moons.split_first_mut().expect("non-empty input");
        for before in before_moons {
            to_update.apply_gravity_force_z(before);
        }
        for after in after_moons {
            to_update.apply_gravity_force_z(after);
        }
    }

    for m in moons {
        m.advance_z()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Velocity {
    dx: isize,
    dy: isize,
    dz: isize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    pub fn apply_gravity_force_x(&mut self, other: &Moon) {
        self.velocity.dx += match std::cmp::Ord::cmp(&self.position.x, &other.position.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    pub fn apply_gravity_force_y(&mut self, other: &Moon) {
        self.velocity.dy += match std::cmp::Ord::cmp(&self.position.y, &other.position.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }
    pub fn apply_gravity_force_z(&mut self, other: &Moon) {
        self.velocity.dz += match std::cmp::Ord::cmp(&self.position.z, &other.position.z) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    pub fn advance_x(&mut self) {
        self.position.x += self.velocity.dx;
    }
    pub fn advance_y(&mut self) {
        self.position.y += self.velocity.dy;
    }
    pub fn advance_z(&mut self) {
        self.position.z += self.velocity.dz;
    }
}

impl FromStr for Moon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let xstr = parts.next().ok_or_else(|| "Invalid format")?;
        let ystr = parts.next().ok_or_else(|| "Invalid format")?;
        let zstr = parts.next().ok_or_else(|| "Invalid format")?;
        let x = xstr[3..].parse()?;
        let y = ystr[3..].parse()?;
        let z = zstr[3..].trim_matches('>').parse()?;

        Ok(Moon {
            position: Position { x, y, z },
            velocity: Velocity { dx: 0, dy: 0, dz: 0 },
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_parse() {
        let moon: Moon = "<x=2, y=-3, z=-14>".parse().unwrap();
        assert_eq!(moon, Moon {
            position: Position { x: 2, y: -3, z: -14 },
            velocity: Velocity { dx: 0, dy: 0, dz: 0 },
        })
    }

    #[test]
    fn test_advance() {
        let mut moon = Moon {
            position: Position { x: 2, y: -3, z: -14 },
            velocity: Velocity { dx: 0, dy: 0, dz: 0 },
        };
        moon.advance();
        assert_eq!(moon, Moon {
            position: Position { x: 2, y: -3, z: -14 },
            velocity: Velocity { dx: 0, dy: 0, dz: 0 },
        });

        moon.velocity = Velocity { dx: 2, dy: 1, dz: -3 };
        moon.advance();
        assert_eq!(moon, Moon {
            position: Position { x: 4, y: -2, z: -17 },
            velocity: Velocity { dx: 2, dy: 1, dz: -3 },
        })
    }

    #[test]
    fn test_repeat() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
        assert_eq!(run(input).unwrap(), 2772);
    }
}
