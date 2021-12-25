fn run_until_deadlock(mut floor: Floor) -> u64 {
    let mut counter = 0;
    let mut moved = true;
    while moved {
        let (f, m) = run_step(floor);
        floor = f;
        moved = m;
        counter += 1;
    }
    counter
}

fn run_step(floor: Floor) -> (Floor, bool) {
    let mut moved = false;
    let width = floor.width;
    let mut buffer: Vec<Tile> = vec![Tile::Empty; floor.tiles.len()];

    for (i, _) in floor
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| **t == Tile::East)
    {
        let x = i % width;
        let y = i / width;
        let target = (x + 1) % width + y * width;
        if floor.tiles[target] == Tile::Empty {
            buffer[target] = Tile::East;
            moved = true;
        } else {
            buffer[i] = Tile::East;
        }
    }
    for (i, _) in floor
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| **t == Tile::South)
    {
        buffer[i] = Tile::South;
    }
    let tiles = buffer;
    let mut buffer = floor.tiles;
    buffer.fill(Tile::Empty);

    for (i, _) in tiles.iter().enumerate().filter(|(_, t)| **t == Tile::South) {
        let target = (i + width) % tiles.len();
        if tiles[target] == Tile::Empty {
            buffer[target] = Tile::South;
            moved = true;
        } else {
            buffer[i] = Tile::South;
        }
    }
    for (i, _) in tiles.iter().enumerate().filter(|(_, t)| **t == Tile::East) {
        buffer[i] = tiles[i];
    }
    (
        Floor {
            width,
            tiles: buffer,
        },
        moved,
    )
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Tile {
    Empty,
    East,
    South,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Floor {
    width: usize,
    tiles: Vec<Tile>,
}

fn parse(input: &str) -> Floor {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "expected non-empty input".to_owned())
        .unwrap()
        .chars()
        .count();
    let tiles = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '.' => Ok(Tile::Empty),
            '>' => Ok(Tile::East),
            'v' => Ok(Tile::South),
            _ => Err(format!("unknown floor tile: '{}'", c)),
        })
        .collect::<Result<Vec<Tile>, String>>()
        .unwrap();

    Floor { width, tiles }
}

pub fn part1(inp: String) {
    let floor = parse(&inp);

    let res = run_until_deadlock(floor);
    println!("{}", res);
}

// auto star
pub fn part2(_inp: String) {}
