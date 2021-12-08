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
pub fn part2(inp: String) {}
