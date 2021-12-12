use std::collections::{HashMap, HashSet};

const START: &str = "start";
const END: &str = "end";

fn count_paths<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    can_visit_one_small_twice: bool,
) -> Option<usize> {
    let mut q = vec![(START, HashSet::new(), false)];
    let mut count = 0;
    while !q.is_empty() {
        let (current, mut visited, mut visited_twice) = q.pop()?;
        if current == END {
            count += 1;
            continue;
        }
        if current.chars().next()?.is_lowercase() {
            let already_visited = visited.contains(&current);
            if already_visited && (!can_visit_one_small_twice || visited_twice || current == START)
            {
                continue;
            }
            if already_visited {
                visited_twice = true;
            }
            visited.insert(current);
        }
        for neighbour in connections.get(&current)? {
            q.push((neighbour, visited.clone(), visited_twice));
        }
    }

    Some(count)
}

pub fn part1(inp: String) {
    let input: HashMap<&str, Vec<&str>> = inp.lines().fold(HashMap::new(), |mut acc, l| {
        let (from, to) = l.split_once('-').unwrap();
        acc.entry(from).or_insert_with(Vec::new).push(to);
        acc.entry(to).or_insert_with(Vec::new).push(from);

        acc
    });
    let res = count_paths(&input, false);
    println!("{:?}", res);
}

pub fn part2(inp: String) {
    let input: HashMap<&str, Vec<&str>> = inp.lines().fold(HashMap::new(), |mut acc, l| {
        let (from, to) = l.split_once('-').unwrap();
        acc.entry(from).or_insert_with(Vec::new).push(to);
        acc.entry(to).or_insert_with(Vec::new).push(from);

        acc
    });
    let res = count_paths(&input, true);
    println!("{:?}", res);
}
