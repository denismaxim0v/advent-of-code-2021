use pathfinding::prelude::Matrix;

fn load(input: String) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap()
}

pub fn part1(inp: String) {
    use pathfinding::prelude::dijkstra;

    let input = load(inp);

    let path = dijkstra(
        &(0, 0),
        |pos| {
            input
                .neighbours(*pos, false)
                .map(|p| (p, *input.get(p).unwrap()))
        },
        |pos| *pos == (input.rows - 1, input.columns - 1),
    )
    .unwrap();

    println!("{}", path.1);
}

pub fn part2(inp: String) {
    use pathfinding::prelude::dijkstra;

    let input = load(inp);

    let mut mat = Matrix::new(input.rows * 5, input.columns * 5, 0u32);
    (0..5).for_each(|y| {
        (0..5).for_each(|x| {
            input.indices().for_each(|p| {
                let pos = (p.0 + x * input.rows, p.1 + y * input.columns);
                let v = *input.get(p).unwrap() + x as u32 + y as u32;
                *mat.get_mut(pos).unwrap() = if v > 9 { v - 9 } else { v }
            });
        })
    });

    let path = dijkstra(
        &(0, 0),
        |pos| {
            mat.neighbours(*pos, false)
                .map(|p| (p, *mat.get(p).unwrap()))
        },
        |pos| *pos == (mat.rows - 1, mat.columns - 1),
    )
    .unwrap();

    println!("{}", path.1);
}
