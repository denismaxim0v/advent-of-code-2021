pub fn part1(inp: String) {
    let mut input: Vec<isize> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();

    input.sort();
    let mid = input.len() / 2;
    let median = input[mid];

    let res: isize = input.iter().map(|num| (median - num).abs()).sum();
    println!("{:?}", res);
}
pub fn part2(inp: String) {}
