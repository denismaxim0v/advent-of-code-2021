fn greater_than_previous(
    (previous, total): (Option<usize>, usize),
    current: usize,
) -> (Option<usize>, usize) {
    (
        Some(current),
        if previous.filter(|&previous| current > previous).is_some() {
            total + 1
        } else {
            total
        },
    )
}

pub fn part1(inp: String) {
    let depths = inp
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    let res = depths
        .iter()
        .cloned()
        .fold((None, 0), greater_than_previous)
        .1;

    println!("{}", res);
}
