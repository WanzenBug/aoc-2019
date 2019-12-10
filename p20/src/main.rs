use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    error::Error,
    ops::Sub,
    str::FromStr,
};

const INPUT: &str = include_str!("../INPUT");

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let result = run(INPUT, 200)?;
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
    type Output = (Slope, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        let dx = self.0 - rhs.0;
        let dy = self.1 - rhs.1;
        (Slope::new(dx, dy), dx * dx + dy * dy)
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

#[derive(Eq, PartialEq, Debug)]
struct RelativeLocation {
    dist: isize,
    original_pos: AstroidCoords,
}

impl PartialOrd for RelativeLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for RelativeLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

fn slope_order(a: &Slope, b: &Slope) -> Ordering {
    let mut a_angle = f64::atan2(a.0 as f64, -a.1 as f64);
    let mut b_angle = f64::atan2(b.0 as f64, -b.1 as f64);

    if a_angle < 0.0 {
        a_angle += 2.0 * std::f64::consts::PI;
    }

    if b_angle < 0.0 {
        b_angle += 2.0 * std::f64::consts::PI;
    }

    if a_angle < b_angle {
        Ordering::Less
    } else {
        // Equality is not possible, slopes are unique!
        Ordering::Equal
    }
}

fn run(input: &str, nth: usize) -> Result<isize, Box<dyn Error + 'static>> {
    let map: AstroidMap = input.parse()?;

    let mut max_visible = 0;
    let mut location = AstroidCoords(-1, -1);
    let mut relative_locations = HashMap::new();
    for loc in map.astroids.iter() {
        let mut current_relative_positions: HashMap<Slope, BinaryHeap<Reverse<_>>> = HashMap::new();
        for other in map.astroids.iter().filter(|&x| x != loc) {
            let (slope, dist) = *other - *loc;
            current_relative_positions.entry(slope)
                .or_default()
                .push(Reverse(RelativeLocation {
                    dist,
                    original_pos: *other,
                }));
        }

        if current_relative_positions.len() > max_visible {
            max_visible = current_relative_positions.len();
            location = *loc;
            relative_locations = current_relative_positions;
        }
    }
    assert_ne!(location, AstroidCoords(-1, -1));

    let mut keys: Vec<Slope> = relative_locations.keys().cloned().collect();

    keys.sort_by(slope_order);
    let mut counter = 0;
    let mut key_counter = keys.iter().cycle();

    let last_vaporized = loop {
        let to_vaporize = loop {
            let key = key_counter.next().expect("endless cycle");
            if let Some(x) = relative_locations.get_mut(key) {
                if let Some(val) = x.pop() {
                    break val.0;
                }
            }
        };

        counter += 1;
        if counter == nth {
            break to_vaporize.original_pos;
        }
    };


    Ok(last_vaporized.0 * 100 + last_vaporized.1)
}

#[test]
fn test() {
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
    assert_eq!(run(input, 200).unwrap(), 802);
}

#[test]
fn test_small() {
    let input = "###
.#.
.#.";
    assert_eq!(run(input, 1).unwrap(), 100);
    assert_eq!(run(input, 2).unwrap(), 200);
    assert_eq!(run(input, 3).unwrap(), 102);
    assert_eq!(run(input, 4).unwrap(), 0);
}


#[test]
fn test_slope() {
    assert_eq!(Slope::new(1, 0), Slope(1, 0));
    assert_eq!(Slope::new(2, 0), Slope(1, 0));
    assert_eq!(Slope::new(0, -1), Slope(0, -1));
    assert_eq!(Slope::new(-1, -1), Slope(-1, -1));
}

#[test]
fn test_slope_order() {
    let mut slopes = [Slope::new(0, -1), Slope::new(-1, -1), Slope::new(1, 0), Slope::new(1, 1)];
    slopes.sort_by(slope_order);
    assert_eq!(slopes, [Slope(0, -1), Slope(1, 0), Slope(1, 1), Slope(-1, -1)]);
}
