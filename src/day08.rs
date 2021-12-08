use std::collections::HashSet;

pub fn part1(inp: String) {
    let lines = inp.trim().lines();

    let mut digit_count = 0;
    for line in lines {
        let mut parts = line.split(" | ");
        let _signal_patterns: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();
        let output: Vec<&str> = parts.next().unwrap().trim().split(" ").collect();

        for digit in output {
            let mut chars = HashSet::new();
            for char in digit.chars() {
                chars.insert(char);
            }

            if chars.len() == 2 || chars.len() == 3 || chars.len() == 4 || chars.len() == 7 {
                digit_count += 1;
            }
        }
    }

    println!("{}", digit_count);
}

pub fn part2(inp: String) {
    let lines = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let patterns = lines
        .iter()
        .map(|v| v[0].split(" ").collect::<Vec<&str>>())
        .map(parse_pattern)
        .collect::<Vec<[&str; 10]>>();

    let res: usize = lines
        .into_iter()
        .map(|v| v[1].split(" ").collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .map(|s| {
                    let mut num = 0;

                    for j in 0..10 {
                        let mut pattern = patterns[i][j].chars();
                        let mut digit = s.chars();

                        if pattern.all(|c| s.contains(c))
                            && digit.all(|c| patterns[i][j].contains(c))
                        {
                            num = j;
                            break;
                        }
                    }

                    num
                })
                .collect::<Vec<usize>>()
        })
        .map(|v| 1000 * v[0] + 100 * v[1] + 10 * v[2] + v[3])
        .sum();

    println!("{}", res);
}

fn parse_pattern(pattern: Vec<&str>) -> [&str; 10] {
    let mut ssd: [char; 7] = ['\0'; 7];
    let mut output = [""; 10];

    for s in pattern.iter() {
        match s.len() {
            2 => output[1] = s,
            3 => output[7] = s,
            4 => output[4] = s,
            7 => output[8] = s,
            _ => {}
        }
    }

    for c in output[7].chars() {
        if !output[4].contains(c) {
            ssd[0] = c;
        }
    }

    {
        let mut count0 = 0;
        let char0 = output[1].chars().nth(0).unwrap();
        let char1 = output[1].chars().nth(1).unwrap();

        for s in pattern.iter() {
            if s.contains(char0) {
                count0 += 1;
            }
        }

        if count0 == 9 {
            ssd[5] = char0;
            ssd[2] = char1;
        } else {
            ssd[5] = char1;
            ssd[2] = char0;
        }
    }

    for s in pattern.iter().filter(|s| s.len() == 5) {
        if !s.contains(ssd[2]) {
            output[5] = s;
        } else {
            if !s.contains(ssd[5]) {
                output[2] = s;
            } else {
                output[3] = s;
            }
        }
    }

    for c in output[2].chars() {
        if !output[3].contains(c) {
            ssd[4] = c;
        }
    }

    for c in output[5].chars() {
        if !output[3].contains(c) {
            ssd[1] = c;
        }
    }

    for s in pattern.iter().filter(|s| s.len() == 6) {
        if !s.contains(ssd[2]) {
            output[6] = s;
        } else {
            if !s.contains(ssd[4]) {
                output[9] = s;
            } else {
                output[0] = s;
            }
        }
    }

    output
}
