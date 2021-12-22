use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cuboid {
    kind: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl FromStr for Cuboid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(&[' ', '=', ','][..]);
        let kind = itr.next().unwrap() == "on";
        let x = itr
            .nth(1)
            .unwrap()
            .split_once("..")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        let y = itr
            .nth(1)
            .unwrap()
            .split_once("..")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        let z = itr
            .nth(1)
            .unwrap()
            .split_once("..")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        Ok(Self { kind, x, y, z })
    }
}

impl Cuboid {
    fn count(&self, others: &[Self]) -> isize {
        let mut total = self.volume();
        let mut conflicts = Vec::new();

        for item in others {
            if let Some(cx) = range_overlap(item.x, self.x) {
                if let Some(cy) = range_overlap(item.y, self.y) {
                    if let Some(cz) = range_overlap(item.z, self.z) {
                        conflicts.push(Cuboid {
                            kind: !item.kind,
                            x: cx,
                            y: cy,
                            z: cz,
                        });
                    }
                }
            }
        }

        for (idx, item) in conflicts.iter().enumerate() {
            total -= item.count(&conflicts[idx + 1..]);
        }

        total
    }

    fn volume(&self) -> isize {
        [self.x, self.y, self.z]
            .iter()
            .map(|c| c.1 - c.0 + 1)
            .product()
    }
}

fn range_overlap(range: (isize, isize), other: (isize, isize)) -> Option<(isize, isize)> {
    if range.1 < other.0 || range.0 > other.1 {
        None
    } else {
        Some((range.0.max(other.0), range.1.min(other.1)))
    }
}

pub fn part1(inp: String) {
    let inputs = parse_input(inp);
    let res: isize = inputs
        .iter()
        .enumerate()
        .filter(|c| c.1.kind)
        .filter_map(|(i, cuboid)| {
            if [cuboid.x, cuboid.y, cuboid.z]
                .iter()
                .all(|r| range_overlap(*r, (-50, 50)).is_some())
            {
                Some(cuboid.count(&inputs[i + 1..]))
            } else {
                None
            }
        })
        .sum();
    println!("{}", res);
}

pub fn part2(inp: String) {
    let inputs = parse_input(inp);
    let res: isize = inputs
        .iter()
        .enumerate()
        .filter(|c| c.1.kind)
        .map(|(i, cuboid)| cuboid.count(&inputs[i + 1..]))
        .sum();

    println!("{}", res);
}

fn parse_input(inp: String) -> Vec<Cuboid> {
    inp.lines().map(|l| l.parse().unwrap()).collect()
}
