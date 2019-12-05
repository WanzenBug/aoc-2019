use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../INPUT");

type Error = Box<dyn std::error::Error + 'static>;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum WireDirection {
    Up,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct WireSegment {
    start: Point,
    length: isize,
    direction: WireDirection,
}

impl WireSegment {
    fn intersection(&self, other: &WireSegment) -> Option<Point> {
        match (self.direction, other.direction) {
            (WireDirection::Up, WireDirection::Right) => {
                let y_min = std::cmp::min(self.start.y, self.start.y + self.length);
                let y_max = std::cmp::max(self.start.y, self.start.y + self.length);
                let x_min = std::cmp::min(other.start.x, other.start.x + other.length);
                let x_max = std::cmp::max(other.start.x, other.start.x + other.length);

                if y_min < other.start.y && other.start.y < y_max && x_min < self.start.x && self.start.x < x_max {
                    Some(Point{ x: self.start.x, y: other.start.y })
                } else {
                    None
                }
            },
            (WireDirection::Right, WireDirection::Up) => other.intersection(self),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Wire(HashSet<WireSegment>);

impl FromStr for Wire {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Point { x: 0, y: 0 };
        let mut set = HashSet::new();

        for instruction in s.split(",") {
            let (direction, length) = match instruction.split_at(1) {
                ("U", x) => (WireDirection::Up, x.parse::<isize>()?),
                ("D", x) => (WireDirection::Up, -x.parse::<isize>()?),
                ("R", x) => (WireDirection::Right, x.parse::<isize>()?),
                ("L", x) => (WireDirection::Right, -x.parse::<isize>()?),
                x => panic!("Unexpcted fragment: {:?}", x),
            };

            set.insert(WireSegment {
                start,
                length,
                direction,
            });

            start = match direction {
                WireDirection::Up => Point { x: start.x, y: start.y + length },
                WireDirection::Right => Point { x: start.x + length, y: start.y },
            }
        }

        Ok(Wire(set))
    }
}

impl Wire {
    fn intersect(&self, other: &Wire) -> Vec<Point> {
        let mut res = Vec::new();
        for seg1 in self.0.iter() {
            for seg2 in other.0.iter() {
                if let Some(x) = seg1.intersection(seg2) {
                    res.push(x);
                }
            }
        }
        res
    }
}

fn main() -> Result<(), Error> {
    let mut instructions = INPUT.split("\n");
    let wire_1: Wire = instructions.next().ok_or_else(|| "not enough lines")?.parse()?;
    let wire_2: Wire = instructions.next().ok_or_else(|| "not enough lines")?.parse()?;

    let mut intersections = wire_1.intersect(&wire_2);
    intersections.sort_by_key(Point::manhattan);
    eprintln!("intersections = {:#?}", intersections);
    eprintln!("intersections[0].manhattan() = {:#?}", intersections[0].manhattan());
    
    Ok(())
}
