enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    val: i32,
}

fn parse_command(line: &str) -> Command {
    let mut split = line.split(" ");

    return Command {
        direction: parse_direction(split.next().unwrap()),
        val: split.next().unwrap_or("0").parse::<i32>().unwrap(),
    };
}

fn parse_direction(dir: &str) -> Direction {
    return match dir {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "forward" => Direction::Forward,
        _ => panic!("failed parsing"),
    };
}

pub fn part1(inp: String) {

    let mut commands: Vec<Command> = vec![];

    let raw_commands: Vec<&str> = inp.lines().collect();

    for command in raw_commands {
        commands.push(parse_command(command));
    }

    let mut depth = 0;
    let mut position = 0;

    for command in commands {
        match command.direction {
            Direction::Up => depth -= command.val,
            Direction::Down => depth += command.val,
            Direction::Forward => position += command.val,
        }
    }

    println!("{}", (depth * position));
}

pub fn part2(inp: String) {

    let mut commands: Vec<Command> = vec![];

    let raw_commands: Vec<&str> = inp.lines().collect();

    for command in raw_commands {
        commands.push(parse_command(command));
    }

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for command in commands {
        match command.direction {
            Direction::Up => aim -= command.val,
            Direction::Down => aim += command.val,
            Direction::Forward => {
                position += command.val;
                depth += aim * command.val;
            }
        }
    }

    println!("{}", (depth * position));
}
