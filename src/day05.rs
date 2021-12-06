#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn map_vh(&self, mut map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        if self.x1 != self.x2 && self.y1 != self.y2 {
            return map;
        }
        let (min_x, max_x) = if self.x1 > self.x2 {
            (self.x2, self.x1 + 1)
        } else {
            (self.x1, self.x2 + 1)
        };
        let (min_y, max_y) = if self.y1 > self.y2 {
            (self.y2, self.y1 + 1)
        } else {
            (self.y1, self.y2 + 1)
        };

        for x in min_x..max_x {
            for y in min_y..max_y {
                map[y][x] += 1;
            }
        }
        map
    }

    fn map_dg(&self, mut map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        if self.x1 != self.x2 && self.y1 != self.y2 {
            let (fx, fy, lx, ly) = if self.x1 < self.x2 {
                (self.x1, self.y1, self.x2, self.y2)
            } else {
                (self.x2, self.y2, self.x1, self.y1)
            };
            let slope = (ly as i32 - fy as i32) / (lx as i32 - fx as i32);

            let mut x = fx as i32;
            let mut y = fy as i32;

            while x <= lx as i32 && (y <= ly as i32 || y > ly as i32) {
                map[y as usize][x as usize] += 1;
                x += 1;
                y += slope;
            }

            return map;
        }
        map
    }
}

pub fn part1(inp: String) {
    let lines: Vec<Line> = inp
        .lines()
        .map(|s| {
            let y: Vec<Vec<usize>> = s
                .split(" -> ")
                .map(|s| s.split(",").map(|i| i.parse().unwrap()).collect())
                .collect();
            Line {
                x1: y[0][0],
                y1: y[0][1],
                x2: y[1][0],
                y2: y[1][1],
            }
        })
        .collect();

    let mut map = vec![vec![0; 1000]; 1000];
    for line in lines {
        map = line.map_vh(map);
    }

    let m: Vec<&u32> = map.iter().flatten().collect();

    let sum = m.iter().filter(|b| b >= &&&2).count();

    println!("{:?}", sum);
}

pub fn part2(inp: String) {
    let lines: Vec<Line> = inp
        .lines()
        .map(|s| {
            let y: Vec<Vec<usize>> = s
                .split(" -> ")
                .map(|s| s.split(",").map(|i| i.parse().unwrap()).collect())
                .collect();
            Line {
                x1: y[0][0],
                y1: y[0][1],
                x2: y[1][0],
                y2: y[1][1],
            }
        })
        .collect();

    let mut map = vec![vec![0; 1000]; 1000];
    for line in lines {
        map = line.map_vh(map);
        map = line.map_dg(map);
    }

    let m: Vec<&u32> = map.iter().flatten().collect();

    let sum = m.iter().filter(|b| b >= &&&2).count();

    println!("{:?}", sum);
}
