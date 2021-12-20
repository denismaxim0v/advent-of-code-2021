use std::collections::HashMap;

type Pos = [i32; 2];

pub fn part1(inp: String) {
    let (algorithm, image) = parse_input(inp);
    let res = enhance(image, algorithm, 2);
    println!("{}", res);
}

pub fn part2(inp: String) {
    let (algorithm, image) = parse_input(inp);
    let res = enhance(image, algorithm, 50);
    println!("{}", res);
}

fn enhance(image: HashMap<Pos, char>, algorithm: Vec<char>, n: usize) -> usize {
    let mut image = image.clone();
    let mut background = '.';
    for _ in 0..n {
        let mut new_image = image.clone();
        let min = image_min(&image);
        let max = image_max(&image);
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                let n = kernel(&image, background, [x, y]);
                new_image.insert([x, y], algorithm[n]);
            }
        }
        image = new_image;
        background = if background == '.' {
            algorithm[0]
        } else {
            *algorithm.last().unwrap()
        };
    }

    assert_eq!(background, '.');
    image.values().filter(|&&c| c == '#').count()
}

fn image_min(image: &HashMap<Pos, char>) -> [i32; 2] {
    let min_x = image.keys().map(|&[x, _]| x).min().unwrap();
    let min_y = image.keys().map(|&[_, y]| y).min().unwrap();

    [min_x - 1, min_y - 1]
}

fn image_max(image: &HashMap<Pos, char>) -> [i32; 2] {
    let max_x = image.keys().map(|&[x, _]| x).max().unwrap();
    let max_y = image.keys().map(|&[_, y]| y).max().unwrap();

    [max_x + 1, max_y + 1]
}

fn kernel(image: &HashMap<Pos, char>, background: char, center: Pos) -> usize {
    let mut n = 0;
    for y in center[1] - 1..=center[1] + 1 {
        for x in center[0] - 1..=center[0] + 1 {
            n <<= 1;
            if *image.get(&[x, y]).unwrap_or(&background) == '#' {
                n += 1;
            }
        }
    }

    n
}

fn parse_input(input: String) -> (Vec<char>, HashMap<Pos, char>) {
    let (chunk1, chunk2) = input.split_once("\n\n").unwrap();

    let algorithm = chunk1.lines().flat_map(|line| line.chars()).collect();

    let mut image = HashMap::new();
    for (y, line) in chunk2.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            image.insert([x as i32, y as i32], c);
        }
    }

    (algorithm, image)
}
