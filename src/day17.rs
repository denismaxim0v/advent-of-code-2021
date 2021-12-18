use std::str::FromStr;

fn parse_input(inp: String) -> (i64, i64, i64, i64) {
    let input: Vec<(i64, i64)> = inp[13..]
        .split(", ")
        .map(|el| {
            let nums: Vec<i64> = el[2..]
                .split("..")
                .map(|num| <i64 as FromStr>::from_str(num.trim()).unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect();
    (input[0].0, input[0].1, input[1].0, input[1].1)
}

pub fn part1(inp: String) {
    let (xmin, xmax, ymin, ymax) = parse_input(inp);

    let mut max_vy_hit: Option<i64> = None;
    for vy in 0..=1_000 {
        for vx in 1..=xmax {
            let hit = |mut vx: i64, mut vy: i64| {
                let (mut x, mut y) = (0, 0);
                let mut t = 0;
                while vy >= 0 || y >= ymin {
                    t += 1;
                    // Apply velocity.
                    x += vx;
                    y += vy;
                    // Apply drag and gravity.
                    if vx.is_negative() {
                        vx += 1;
                    } else if vx.is_positive() {
                        vx -= 1;
                    }
                    vy -= 1;
                    // Check containment.
                    if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
                        return Some(t);
                    }
                }
                None
            };

            if hit(vx, vy).is_some() {
                max_vy_hit = Some(match max_vy_hit {
                    Some(current) => current.max(vy),
                    None => vy,
                })
            }
        }
    }
    let vy = max_vy_hit.unwrap();
    let res = vy * (vy + 1) / 2;

    println!("{}", res);
}
pub fn part2(inp: String) {
    let (xmin, xmax, ymin, ymax) = parse_input(inp);
    let mut count = 0;
    // this is cheating but it gives the right answer
    for vy in -1_000..=1_000 {
        for vx in 1..=1_000 {
            let hit = |mut vx: i64, mut vy: i64| {
                let (mut x, mut y) = (0, 0);
                let mut t = 0;
                while vy >= 0 || y >= ymin {
                    t += 1;
                    // Apply velocity.
                    x += vx;
                    y += vy;
                    // Apply drag and gravity.
                    if vx.is_negative() {
                        vx += 1;
                    } else if vx.is_positive() {
                        vx -= 1;
                    }
                    vy -= 1;
                    // Check containment.
                    if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
                        return Some(t);
                    }
                }
                None
            };

            if hit(vx, vy).is_some() {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
