pub fn part1(inp: String) {
    let input: Vec<Vec<usize>> = inp
        .trim()
        .split("\n")
        .map(|s| s.trim().split("").map(|c| c.parse().unwrap_or(0)).collect())
        .collect();
    let mut sum_low_points = 0;
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for row in 0..input.len() as i32 {
        for col in 0..input[row as usize].len() as i32 {
            let v = input[row as usize][col as usize];
            let mut low_neighbors = 0;
            for (offset_row, offset_col) in offsets {
                let r = row + offset_row;
                let c = col + offset_col;
                if ((r < 0 || r >= input.len() as i32) && r != row)
                    || ((c < 0 || c >= input[row as usize].len() as i32) && c != col)
                    || (v < input[r as usize][c as usize])
                {
                    low_neighbors += 1;
                }
            }
            if low_neighbors == 4 {
                sum_low_points += 1 + v;
            }
        }
    }

    println!("{}", sum_low_points);
}
pub fn part2(inp: String) {}
