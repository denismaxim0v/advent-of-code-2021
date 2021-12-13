use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Fold {
    X(i32),
    Y(i32),
}

fn fold(dots: &mut HashSet<(i32, i32)>, fold: Fold) {
    *dots = dots
        .drain()
        .map(|(x, y)| match fold {
            Fold::X(n) => (n - (x - n).abs(), y),
            Fold::Y(n) => (x, n - (y - n).abs()),
        })
        .collect();
}

pub fn part1(inp: String) {
    let (dots, instructions) = inp.split_once("\n\n").unwrap();

    let mut dots: HashSet<(i32, i32)> = dots
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let folds: Vec<Fold> = instructions
        .lines()
        .map(|l| {
            let n = l[13..].parse().unwrap();

            if l.contains('x') {
                Fold::X(n)
            } else {
                Fold::Y(n)
            }
        })
        .collect();

    let _ = fold(&mut dots, folds[0]);
    println!("{}", dots.len());
}

// JGAJEFKU
pub fn part2(inp: String) {
    let (dots, instructions) = inp.split_once("\n\n").unwrap();

    let mut dots: HashSet<(i32, i32)> = dots
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let folds: Vec<Fold> = instructions
        .lines()
        .map(|l| {
            let n = l[13..].parse().unwrap();

            if l.contains('x') {
                Fold::X(n)
            } else {
                Fold::Y(n)
            }
        })
        .collect();

    for f in folds {
        fold(&mut dots, f);
    }

    let xmax = dots.iter().map(|(x, _)| *x).max().unwrap();
    let ymax = dots.iter().map(|(_, y)| *y).max().unwrap();

    for y in 0..=ymax {
        for x in 0..=xmax {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
