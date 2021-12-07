pub fn part1(inp: String) {
    let mut input: Vec<isize> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();

    input.sort();
    let mid = input.len() / 2;
    let median = input[mid];

    let res: isize = input.iter().map(|num| (median - num).abs()).sum();
    println!("{:?}", res);
}

pub fn part2(inp: String) {
    let input: Vec<isize> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();

    let avg: isize = input.iter().sum::<isize>() / input.len() as isize;

    let calc = |i| {
        let mut dist = 0;
        for x in 1..i + 1 {
            dist = dist + x;
        }
        dist
    };

    let res: isize = input.iter().map(|i| calc((avg - i).abs())).sum();

    println!("{:?}", res);
}
