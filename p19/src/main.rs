use std::error::Error;
use std::str::FromStr;
use std::ops::Sub;
use std::collections::HashSet;

const INPUT: &str = include_str!("../INPUT");

fn main() -> Result<(), Box<dyn Error + 'static>>{
    let result = run(INPUT)?;
    println!("result = {}", result);
    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct AstroidCoords(isize, isize);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Slope(isize, isize);

impl Slope {
    fn new(dx: isize, dy: isize) -> Self {
        fn gcd(mut a: isize, mut b: isize) -> isize {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        match (dx, dy) {
            (0, 0) => Slope(0, 0),
            (0, x) => Slope(0, x.signum()),
            (x, 0) => Slope(x.signum(), 0),
            (x, y) => {
                let g = gcd(x.abs(), y.abs());
                Slope(x / g, y / g)
            }
        }
    }
}

impl Sub for AstroidCoords {
    type Output = Slope;

    fn sub(self, rhs: Self) -> Self::Output {
        Slope::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct AstroidMap {
    astroids: Vec<AstroidCoords>,
}

impl FromStr for AstroidMap {
    type Err = Box<dyn Error + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut astroids = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    astroids.push(AstroidCoords(x as isize, y as isize))
                }
            }
        }

        Ok(AstroidMap { astroids })
    }
}

fn run(input: &str) -> Result<usize, Box<dyn Error + 'static>> {
    let map: AstroidMap = input.parse()?;

    let mut max_visible = 0;
    for loc in map.astroids.iter() {
        let mut possible_slopes = HashSet::new();
        for other in map.astroids.iter().filter(|&x| x != loc) {
            possible_slopes.insert(*other - *loc);
        }

        if possible_slopes.len() > max_visible {
            max_visible = possible_slopes.len();
        }
    }
    Ok(max_visible)
}


#[test]
fn test1() {
    let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
    assert_eq!(run(input).unwrap(), 33);
}

#[test]
fn test2() {
    let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
    assert_eq!(run(input).unwrap(), 35);
}

#[test]
fn test3() {
    let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
    assert_eq!(run(input).unwrap(), 41);
}

#[test]
fn test4() {
    let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
    assert_eq!(run(input).unwrap(), 210);
}


#[test]
fn test_slope() {
    assert_eq!(Slope::new(1, 0), Slope(1, 0));
    assert_eq!(Slope::new(2, 0), Slope(1, 0));
    assert_eq!(Slope::new(0, -1), Slope(0, -1));
    assert_eq!(Slope::new(-1, -1), Slope(-1, -1));
}
