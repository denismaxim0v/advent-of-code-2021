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
pub fn part2(inp: String) {}
