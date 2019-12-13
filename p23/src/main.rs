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
    let mut moons = moons?;


    for _timestamp in 0..1000 {
        advance_tick(&mut moons);
    }

    Ok(moons.iter().map(Moon::energy).sum())
}

fn advance_tick(moons: &mut [Moon]) {
    for midx in 0..moons.len() {
        let (before_moons, rest_moons) = moons.split_at_mut(midx);
        let (to_update, after_moons) = rest_moons.split_first_mut().expect("non-empty input");
        for before in before_moons {
            to_update.apply_gravity_force(before);
        }
        for after in after_moons {
            to_update.apply_gravity_force(after);
        }
    }

    for m in moons {
        m.advance()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct Velocity {
    dx: isize,
    dy: isize,
    dz: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    pub fn apply_gravity_force(&mut self, other: &Moon) {
        self.velocity.dx += match std::cmp::Ord::cmp(&self.position.x, &other.position.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        self.velocity.dy += match std::cmp::Ord::cmp(&self.position.y, &other.position.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        self.velocity.dz += match std::cmp::Ord::cmp(&self.position.z, &other.position.z) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    pub fn advance(&mut self) {
        self.position.x += self.velocity.dx;
        self.position.y += self.velocity.dy;
        self.position.z += self.velocity.dz;
    }

    pub fn energy(&self) -> usize {
        (
            self.position.x.abs() as usize
                + self.position.y.abs() as usize
                + self.position.z.abs() as usize
        ) * (
            self.velocity.dx.abs() as usize
                + self.velocity.dy.abs() as usize
                + self.velocity.dz.abs() as usize
        )
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
    fn test_energy() {
        let moon = Moon {
            position: Position { x: 2, y: 1, z: -3 },
            velocity: Velocity { dx: -3, dy: -2, dz: 1 },
        };

        assert_eq!(moon.energy(), 36);
    }
}
