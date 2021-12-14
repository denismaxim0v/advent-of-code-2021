use std::collections::HashMap;

pub fn solve(start: &str, rules: &HashMap<String, char>, iters: usize) {
    let mut freqs: HashMap<char, i64> = HashMap::new();
    for c in start.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }
    let mut pairs = HashMap::new();
    for i in 0..start.len() - 1 {
        *pairs
            .entry(start.get(i..i + 2).unwrap().to_string())
            .or_insert(0) += 1;
    }
    for _ in 0..iters {
        let mut pairs_new = pairs.clone();
        for (p, count) in pairs.iter() {
            if let Some(c) = rules.get(p) {
                let chars = p.chars().collect::<Vec<_>>();
                *pairs_new.entry(p.to_string()).or_insert(0) -= count;
                *pairs_new.entry(format!("{}{}", chars[0], c)).or_insert(0) += count;
                *pairs_new.entry(format!("{}{}", c, chars[1])).or_insert(0) += count;
                *freqs.entry(*c).or_insert(0) += count;
            }
        }
        pairs = pairs_new;
    }
    let max = freqs.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = freqs.iter().min_by_key(|(_, v)| *v).unwrap();
    println!("{}", *max.1 as i64 - *min.1 as i64);
}

pub fn parse_inp(inp: &String) -> (&str, HashMap<String, char>) {
    let input = inp.trim().split('\n').collect::<Vec<_>>();
    let start = input[0];
    let rule_slice = &input[2..];
    let mut rules = HashMap::new();
    for rule in rule_slice.iter() {
        let split = rule.split(" -> ").collect::<Vec<_>>();
        rules.insert(split[0].to_string(), split[1].chars().next().unwrap());
    };
    (start, rules)
}

pub fn part1(inp: String) {
    let (start, rules) = parse_inp(&inp);
    solve(start, &rules, 10); 
}
pub fn part2(inp: String) {
   let (start, rules) = parse_inp(&inp);
    solve(start, &rules, 40); 
}