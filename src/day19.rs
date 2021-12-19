use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(i32, i32, i32);

fn parse_input(input: String) -> Vec<Vec<Point>> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|line| {
                    let mut coords = line.trim().split(',').map(|c| c.parse::<i32>().unwrap());
                    Point(
                        coords.next().unwrap(),
                        coords.next().unwrap(),
                        coords.next().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

pub fn part1(inp: String) {
    let mut input = parse_input(inp);
    let mut found: HashSet<_> = input.pop().unwrap().into_iter().collect();
    while !input.is_empty() {
        let mut to_remove = None;
        for (si, s) in input.iter().enumerate() {
            println!("{},{:#?}", si, s);
            if let Some((or, tr)) = find_scanner_pos(&found, s, 12) {
                for p in s {
                    found.insert(p.rotate(or).translate(tr));
                }
                to_remove = Some(si);
                break;
            }
        }
        input.remove(to_remove.unwrap());
    }

    println!("{}", found.len())
}

impl Point {
    fn rotate(self, i: usize) -> Point {
        let Point(x, y, z) = self;
        match i {
            0 => Point(x, y, z),
            1 => Point(x, -y, -z),
            2 => Point(x, z, -y),
            3 => Point(x, -z, y),
            4 => Point(-x, y, -z),
            5 => Point(-x, -y, z),
            6 => Point(-x, z, y),
            7 => Point(-x, -z, -y),
            8 => Point(y, z, x),
            9 => Point(y, -z, -x),
            10 => Point(y, x, -z),
            11 => Point(y, -x, z),
            12 => Point(-y, z, -x),
            13 => Point(-y, -z, x),
            14 => Point(-y, x, z),
            15 => Point(-y, -x, -z),
            16 => Point(z, x, y),
            17 => Point(z, -x, -y),
            18 => Point(z, y, -x),
            19 => Point(z, -y, x),
            20 => Point(-z, x, -y),
            21 => Point(-z, -x, y),
            22 => Point(-z, y, x),
            23 => Point(-z, -y, -x),
            _ => panic!("bad rotation"),
        }
    }

    fn translate(self, Point(dx, dy, dz): Point) -> Point {
        let Point(x, y, z) = self;
        Point(x + dx, y + dy, z + dz)
    }
}

fn find_scanner_pos(
    s1: &HashSet<Point>,
    s2: &[Point],
    req_matches: usize,
) -> Option<(usize, Point)> {
    for i in 0..24 {
        let s2_rot: Vec<_> = s2.iter().copied().map(|p| p.rotate(i)).collect();
        for s2_orig in &s2_rot {
            for s1_orig in s1 {
                let dx = s1_orig.0 - s2_orig.0;
                let dy = s1_orig.1 - s2_orig.1;
                let dz = s1_orig.2 - s2_orig.2;
                let tr = Point(dx, dy, dz);

                let same_count = s2_rot
                    .iter()
                    .map(|p| p.translate(tr))
                    .filter(|p| s1.contains(p))
                    .take(12)
                    .count();
                if same_count >= req_matches {
                    return Some((i, tr));
                }
            }
        }
    }

    None
}

pub fn part2(inp: String) {
    let mut input = parse_input(inp);
    let mut found: HashSet<_> = input.pop().unwrap().into_iter().collect();
    let mut scanner_locs: Vec<Point> = Vec::new();
    while !input.is_empty() {
        let mut to_remove = None;
        for (si, s) in input.iter().enumerate() {
            if let Some((or, tr)) = find_scanner_pos(&found, s, 12) {
                scanner_locs.push(tr);
                for p in s {
                    found.insert(p.rotate(or).translate(tr));
                }
                to_remove = Some(si);
                break;
            }
        }
        input.remove(to_remove.unwrap());
    }

    let mut res = 0;
    for sl1 in &scanner_locs {
        for sl2 in &scanner_locs {
            let d = (sl1.0 - sl2.0).abs() + (sl1.1 - sl2.1).abs() + (sl1.2 - sl2.2).abs();
            if d > res {
                res = d;
            }
        }
    }

    println!("{}", res);
}
