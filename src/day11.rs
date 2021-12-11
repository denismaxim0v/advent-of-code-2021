use std::collections::HashMap;

fn flash(oct: &mut HashMap<(i32, i32), u32>, y: i32, x: i32) -> u64 {
    let mut flashes = 1;
    for (dy, dx) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ] {
        if let Some(val) = oct.get_mut(&(y + dy, x + dx)) {
            *val += 1;
            if *val == 10 {
                flashes += flash(oct, y + dy, x + dx);
            }
        }
    }
    flashes
}

pub fn part1(input: String) {
    let mut oct = HashMap::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            oct.insert((y as i32, x as i32), c.to_digit(10).unwrap());
        }
    }

    let mut flashes = 0;
    for _ in 0..100 {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    *val += 1;
                    if *val == 10 {
                        flashes += flash(&mut oct, y, x);
                    }
                }
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    if *val >= 10 {
                        *val = 0;
                    }
                }
            }
        }
    }

    println!("{}", flashes);
}

pub fn part2(input: String) {
    let mut oct = HashMap::new();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            oct.insert((y as i32, x as i32), c.to_digit(10).unwrap());
        }
    }

    'outer: for i in 1.. {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    *val += 1;
                    if *val == 10 {
                        flash(&mut oct, y, x);
                    }
                }
            }
        }

        let mut is_sync = true;
        for y in 0..10 {
            for x in 0..10 {
                if let Some(val) = oct.get_mut(&(y, x)) {
                    if *val >= 10 {
                        *val = 0;
                    } else {
                        is_sync = false;
                    }
                }
            }
        }

        if is_sync {
            println!("{}", i);
            break 'outer;
        }
    }
}
