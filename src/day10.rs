pub fn part1(inp: String) {
    let input: Vec<Vec<&str>> = inp
        .trim()
        .split("\n")
        .map(|s| s.trim().split("").collect())
        .collect();

    let mut stack = Vec::new();
    let mut res = 0;

    for line in input {
        for val in line {
            match val {
                "{" | "(" | "[" | "<" => stack.push(val),
                _ => match val {
                    "}" => {
                        if stack.is_empty() {
                            break;
                        } else if stack[stack.len() - 1] == "{" {
                            stack.pop();
                        } else {
                            res += 1197;
                            break;
                        }
                    }
                    "]" => {
                        if stack.is_empty() {
                            break;
                        } else if stack[stack.len() - 1] == "[" {
                            stack.pop();
                        } else {
                            res += 57;
                            break;
                        }
                    }
                    ")" => {
                        if stack.is_empty() {
                            break;
                        } else if stack[stack.len() - 1] == "(" {
                            stack.pop();
                        } else {
                            res += 3;
                            break;
                        }
                    }
                    ">" => {
                        if stack.is_empty() {
                            break;
                        } else if stack[stack.len() - 1] == "<" {
                            stack.pop();
                        } else {
                            res += 25137;
                            break;
                        }
                    }
                    _ => continue,
                },
            }
        }
    }

    println!("{}", res);
}
pub fn part2(inp: String) {
    let mut stack: Vec<&str> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();

    for line in inp.lines() {
        let mut removed = false;
        stack.clear();
        for c in line.split("").collect::<Vec<&str>>() {
            match c {
                "(" | "[" | "{" | "<" => stack.push(c),
                ")" => {
                    if stack.pop().unwrap() != "(" {
                        removed = true;
                        break;
                    }
                }
                "]" => {
                    if stack.pop().unwrap() != "[" {
                        removed = true;
                        break;
                    }
                }
                "}" => {
                    if stack.pop().unwrap() != "{" {
                        removed = true;
                        break;
                    }
                }
                ">" => {
                    if stack.pop().unwrap() != "<" {
                        removed = true;
                        break;
                    }
                }
                _ => continue,
            }
        }

        if !removed {
            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                match *c {
                    "(" => score += 1,
                    "[" => score += 2,
                    "{" => score += 3,
                    "<" => score += 4,
                    _ => continue,
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() >> 1]);
}
