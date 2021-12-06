fn simulate_lanternfish(input: &[usize], n: usize) -> usize {
    // Keep track of internal timers
    let mut population = vec![0; 9];
    for &x in input {
        population[x] += 1;
    }

    for _ in 0..n {
        let x = population[0];

        population[0] = population[1];
        population[1] = population[2];
        population[2] = population[3];
        population[3] = population[4];
        population[4] = population[5];
        population[5] = population[6];
        population[6] = population[7] + x;
        population[7] = population[8];
        population[8] = x;
    }

    population.iter().sum()
}

pub fn part1(inp: String) {
    let input: Vec<usize> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();

    let res = simulate_lanternfish(&input, 80);

    println!("{:?}", res);
}
pub fn part2(inp: String) {
    let input: Vec<usize> = inp.trim().split(",").map(|s| s.parse().unwrap()).collect();

    let res = simulate_lanternfish(&input, 256);

    println!("{:?}", res);
}
