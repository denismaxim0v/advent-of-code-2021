use std::collections::HashMap;

pub fn part1(inp: String) {
    let input: Vec<Vec<usize>> = inp
        .trim()
        .split("\n")
        .map(|s| s.trim().split("").map(|c| c.parse().unwrap_or(0)).collect())
        .collect();
    let mut sum_low_points = 0;
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for row in 0..input.len() as i32 {
        for col in 0..input[row as usize].len() as i32 {
            let v = input[row as usize][col as usize];
            let mut low_neighbors = 0;
            for (offset_row, offset_col) in offsets {
                let r = row + offset_row;
                let c = col + offset_col;
                if ((r < 0 || r >= input.len() as i32) && r != row)
                    || ((c < 0 || c >= input[row as usize].len() as i32) && c != col)
                    || (v < input[r as usize][c as usize])
                {
                    low_neighbors += 1;
                }
            }
            if low_neighbors == 4 {
                sum_low_points += 1 + v;
            }
        }
    }

    println!("{}", sum_low_points);
}

pub fn part2(inp: String) {
    let mut map = HashMap::new();
    let mut basins: HashMap<(i64, i64), usize> = HashMap::new();

    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in inp.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_y = y;
            max_x = x;
            map.insert((y as i64, x as i64), c.to_digit(10).unwrap());
        }
    }

    let mut low_points = Vec::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let y = y as i64;
            let x = x as i64;
            let p = map[&(y, x)];
            let mut low_point = true;
            for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(other) = map.get(&(y + dy, x + dx)) {
                    if *other <= p {
                        low_point = false;
                        break;
                    }
                }
            }
            if low_point {
                low_points.push((y, x));
            }
        }
    }

    for (basin, point) in low_points.into_iter().enumerate() {
        let mut to_fill = vec![point];
        while let Some(point) = to_fill.pop() {
            basins.insert(point, basin);

            let height = map[&point];
            for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let other_point = (point.0 + dy, point.1 + dx);
                if let Some(other) = map.get(&other_point) {
                    if *other > height && *other != 9 && !basins.contains_key(&other_point) {
                        to_fill.push(other_point);
                    }
                }
            }
        }
    }

    let mut basin_sizes: HashMap<usize, u32> = HashMap::new();
    for basin_num in basins.values() {
        *basin_sizes.entry(*basin_num).or_default() += 1;
    }

    let mut basin_sizes: Vec<_> = basin_sizes.into_iter().collect();
    basin_sizes.sort_by_key(|item| item.1);
    basin_sizes.reverse();

    println!("{}", basin_sizes[0].1 * basin_sizes[1].1 * basin_sizes[2].1)
}
