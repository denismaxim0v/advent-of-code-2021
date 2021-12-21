use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn new(starting: u32) -> Self {
        Self {
            position: starting,
            score: 0,
        }
    }

    fn roll(&mut self, roll: u32) -> u32 {
        self.position = (self.position + roll - 1) % 10 + 1;
        self.score += self.position;
        self.score
    }
}

fn dice() -> HashMap<usize, usize> {
    let mut vs = HashMap::new();
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                *vs.entry(r1 + r2 + r3).or_insert(0) += 1;
            }
        }
    }
    vs
}

pub fn part1(inp: String) {
    let (s1, s2) = parse_input(inp);
    let mut p1 = Player::new(s1);
    let mut p2 = Player::new(s2);
    let mut die = 1;
    let mut rolls = 0;
    let mut roll = || {
        rolls += 1;
        let temp = die;
        let ndie = die % 100 + 1;
        die = ndie;
        temp
    };
    while p1.roll(roll() + roll() + roll()) < 1000 && p2.roll(roll() + roll() + roll()) < 1000 {}
    let res = std::cmp::min(p1.score, p2.score) * rolls;
    println!("{}", res);
}

pub fn part2(inp: String) {
    let (s1, s2) = parse_input(inp);
    let dice = dice();
    let mut p1wins = 0;
    let mut p2wins = 0;
    let mut universes = HashMap::new();
    universes.insert((Player::new(s1), Player::new(s2)), 1);
    while !universes.is_empty() {
        for ((p1, p2), cnt) in std::mem::take(&mut universes) {
            for (&d1, &dcnt1) in dice.iter() {
                let mut p1 = p1.clone();
                if p1.roll(d1 as u32) >= 21 {
                    p1wins += cnt * dcnt1;
                    continue;
                }
                for (&d2, &dcnt2) in dice.iter() {
                    let mut p2 = p2.clone();
                    if p2.roll(d2 as u32) >= 21 {
                        p2wins += cnt * dcnt1 * dcnt2;
                        continue;
                    }
                    *universes.entry((p1.clone(), p2)).or_insert(0) += cnt * dcnt1 * dcnt2;
                }
            }
        }
    }
    let res = std::cmp::max(p1wins, p2wins);
    println!("{}", res);
}

fn parse_input(input: String) -> (u32, u32) {
    let input: Vec<u32> = input
        .trim()
        .lines()
        .map(|l| {
            let temp: Vec<&str> = l.trim().split(":").collect();
            temp[1].trim().parse::<u32>().unwrap()
        })
        .collect();

    (input[0], input[1])
}
