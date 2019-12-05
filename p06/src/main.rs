use std::collections::{HashSet, HashMap, BinaryHeap};
use std::str::FromStr;
use std::collections::hash_map::Entry::Vacant;
use std::cmp::Ordering;

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


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct WireSegment {
    start: Point,
    length: isize,
    direction: WireDirection,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct WireEdge {
    length: usize,
    neighbor: Point,
}

struct WireGraph {
    inner: HashMap<Point, HashSet<WireEdge>>
}

impl WireSegment {
    fn intersection(&self, other: &WireSegment) -> Option<Point> {
        match (self.direction, other.direction) {
            (WireDirection::Up, WireDirection::Right) => {
                let cmp = Point { x: self.start.x, y: other.start.y };

                if self.is_on_segment(cmp) && other.is_on_segment(cmp) {
                    Some(cmp)
                } else {
                    None
                }
            }
            (WireDirection::Right, WireDirection::Up) => other.intersection(self),
            _ => None,
        }
    }

    fn is_on_segment(&self, p: Point) -> bool {
        match self.direction {
            WireDirection::Up => {
                let y_min = std::cmp::min(self.start.y, self.start.y + self.length);
                let y_max = std::cmp::max(self.start.y, self.start.y + self.length);

                p.x == self.start.x && y_min < p.y && p.y < y_max
            }
            WireDirection::Right => {
                let x_min = std::cmp::min(self.start.x, self.start.x + self.length);
                let x_max = std::cmp::max(self.start.x, self.start.x + self.length);

                p.y == self.start.y && x_min < p.x && p.x < x_max
            }
        }
    }

    fn cut_at(&self, mut ps: Vec<Point>) -> Vec<WireSegment> {

        match self.direction {
            WireDirection::Up => ps.sort_by_key(|p| (p.y - self.start.y).abs()),
            WireDirection::Right => ps.sort_by_key(|p| (p.x - self.start.x).abs()),
        }

        let mut start = self.start;
        let end = self.reverse().start;

        let mut res = Vec::new();
        for p in ps {
            res.push(WireSegment {
                start,
                length: (p.x - start.x) + (p.y - start.y),
                direction: self.direction
            });
            start = p;
        }

        res.push(WireSegment {
            start,
            length: (end.x - start.x) + (end.y - start.y),
            direction: self.direction,
        });

        res
    }

    fn reverse(&self) -> WireSegment {
        match self.direction {
            WireDirection::Up => WireSegment {
                start: Point { x: self.start.x, y: self.start.y + self.length },
                length: -self.length,
                direction: WireDirection::Up,
            },
            WireDirection::Right => WireSegment {
                start: Point { x: self.start.x + self.length, y: self.start.y },
                length: -self.length,
                direction: WireDirection::Right,
            }
        }
    }
}

#[derive(Debug, Clone)]
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

    fn cut_at(&mut self, cut_points: &[Point]) {
        let mut to_cut: HashMap<_, Vec<_>> = HashMap::new();
        for seg in self.0.iter() {
            for &p in cut_points {
                if seg.is_on_segment(p) {
                    to_cut.entry(seg.clone())
                        .or_default()
                        .push(p);
                }
            }
        }

        for (seg, cutpoints) in to_cut {
            assert!(self.0.remove(&seg));
            self.0.extend(seg.cut_at(cutpoints));
        }
    }

    fn to_graph(&self) -> WireGraph {
        let mut map: HashMap<_, HashSet<WireEdge>> = HashMap::new();
        for seg in self.0.iter() {
            let start = seg.start;
            let len = seg.length.abs() as usize;
            let end = seg.reverse().start;

            map.entry(start)
                .or_default()
                .insert(WireEdge { length: len, neighbor: end });

            map.entry(end)
                .or_default()
                .insert(WireEdge { length: len, neighbor: start });
        }

        WireGraph { inner: map }
    }
}

impl PartialOrd for WireEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.length.cmp(&other.length))
    }
}

impl Ord for WireEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.cmp(&other.length)
    }
}

impl WireGraph {
    fn dijkstra(&self, targets: &[Point]) -> Vec<usize> {
        let mut dist_map = HashMap::new();
        dist_map.insert(Point { x: 0, y: 0 }, 0);
        let mut candidates = BinaryHeap::new();

        for t in targets {
            assert!(self.inner.contains_key(t));
        }

        for edge in self.inner.get(&Point { x: 0, y: 0 }).expect("0, 0 must be connected!") {
            candidates.push(std::cmp::Reverse(edge.clone()));
        }

        while let Some(std::cmp::Reverse(current)) = candidates.pop() {
            let visit = current.neighbor;
            let add = match dist_map.entry(visit) {
                Vacant(v) => {
                    v.insert(current.length);
                    true
                }
                _ => false,
            };

            if add {
                for edge in self.inner.get(&current.neighbor).expect("Has to have neighbors!") {
                    candidates.push(std::cmp::Reverse(WireEdge {
                        neighbor: edge.neighbor,
                        length: current.length + edge.length,
                    }));
                }
            }
        }


        let mut dists = Vec::new();
        for target in targets {
            dists.push(*dist_map.get(target).expect("Everything is connected!"))
        }

        dists
    }
}


fn main() -> Result<(), Error> {
    let res = run(INPUT)?;
    eprintln!("res = {:#?}", res);
    Ok(())
}

fn run(inp: &str) -> Result<usize, Error> {
    let mut instructions = inp.split("\n");
    let mut wire_1: Wire = instructions.next().ok_or_else(|| "not enough lines")?.parse()?;
    let mut wire_2: Wire = instructions.next().ok_or_else(|| "not enough lines")?.parse()?;

    let intersect = wire_1.intersect(&wire_2);
    let self_intersect_1 = wire_1.intersect(&wire_1);
    let self_intersect_2 = wire_2.intersect(&wire_2);

    wire_1.cut_at(&intersect);
    // wire_1.cut_at(&self_intersect_1);

    wire_2.cut_at(&intersect);
    // wire_2.cut_at(&self_intersect_2);

    let graph_1 = wire_1.to_graph();
    let graph_2 = wire_2.to_graph();

    let v1 =  graph_1.dijkstra(&intersect);
    let v2 = graph_2.dijkstra(&intersect);
    eprintln!("v1 = {:#?}", v1);
    eprintln!("v2 = {:#?}", v2);

    let mut dists: Vec<_> = Iterator::zip(v1.into_iter(), v2.into_iter()).map(|(d1, d2)| d1 + d2).collect();
    eprintln!("dists = {:#?}", dists);

    dists.sort();
    Ok(dists[0])
}

#[test]
fn test_stuff1() {
    let r = run("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap();
    assert_eq!(r, 610);
}

#[test]
fn test_stuff2() {
    let r = run("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();
    assert_eq!(r, 410)
}
