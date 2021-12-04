// ARE THERE ANY REWARDS FOR SLOWEST SOLUTIONS?
#[derive(Debug)]
pub struct Board {
    numbers: Vec<(u32, bool)>,
}

impl Board {
    fn mark(&mut self, x: u32) {
        self.numbers
            .iter_mut()
            .filter(|(e, _)| *e == x)
            .for_each(|(_, mark)| *mark = true);
    }

    fn is_winning(&self) -> bool {
        let mut res = false;

        // shorter as a closure
        let row = |i| self.numbers.iter().skip(i * 5).take(5);
        let col = |i| self.numbers.iter().skip(i).step_by(5);

        for i in 0..5 {
            if row(i).all(|(_, mark)| *mark) || col(i).all(|(_, mark)| *mark) {
                res = true
            };
        }

        res
    }

    fn calc_score(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&(_, mark)| !mark)
            .map(|&(val, _)| val)
            .sum()
    }
}

#[derive(Debug)]
pub struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn play(&mut self) -> u32 {
        for num in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark(*num);
                if board.is_winning() {
                    return board.calc_score() * num;
                }
            }
        }

        return 0;
    }

    fn play2(&mut self) -> u32 {
        let res = 0;
        let mut won = vec![];
        let length = self.boards.len();
        for num in self.numbers.iter() {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if won.contains(&i) {
                    continue;
                }
                board.mark(*num);
                if board.is_winning() {
                    won.push(i);
                    if won.len() == length {
                        return board.calc_score() * num;
                    }
                }
            }
        }
        println!("{:?}", won);

        return res;
    }
}

pub fn parse_bingo(inp: String) -> Bingo {
    let mut input = inp.as_str().split("\n\n");

    let numbers = input
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let boards: Vec<Board> = input
        .map(|board| {
            let board = board
                .split_whitespace()
                .map(|n| (n.parse().unwrap(), false))
                .collect::<Vec<(u32, bool)>>();
            Board { numbers: board }
        })
        .collect::<Vec<Board>>();

    Bingo { numbers, boards }
}

pub fn part1(inp: String) {
    let mut bingo: Bingo = parse_bingo(inp);
    let res = bingo.play();

    println!("{:?}", res);
}

pub fn part2(inp: String) {
    let mut bingo: Bingo = parse_bingo(inp);
    let res = bingo.play2();

    println!("{:?}", res);
}
