use bitvec::prelude::*;
use std::collections::BTreeMap;

pub fn part1(inp: String) {
    let input = inp
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>();

    let mut ones: BTreeMap<usize, usize> = BTreeMap::new();
    let mut zeroes: BTreeMap<usize, usize> = BTreeMap::new();

    for item in input {
        let bits = item.view_bits::<Lsb0>();

        for (i, bit) in bits.iter().enumerate() {
            if i >= 12 {
                break;
            }
            if *bit {
                *ones.entry(i).or_insert(0) += 1;
            } else {
                *zeroes.entry(i).or_insert(0) += 1;
            }
        }
    }

    let gamma = {
        let mut b = 0usize;

        ones.iter().zip(zeroes.iter()).for_each(|(ones, zeros)| {
            if ones.1 > zeros.1 {
                b += 1 << ones.0;
            } else {
                b += 0 << zeros.0;
            }
        });

        b
    };

    let epsilon = !gamma & 0xfff;
    println!("{}", gamma * epsilon);
}

pub fn part2(inp: String) {
    let rating = |most_common: bool| -> u32 {
        let mut input: Vec<_> = inp.lines().collect();
        let mut col = 0;
        while input.len() > 1 {
            let ones = input.iter().filter(|l| l.as_bytes()[col] == b'1').count();
            let bit = match (most_common, ones * 2 >= input.len()) {
                (true, true) | (false, false) => b'1',
                _ => b'0',
            };
            input = input
                .into_iter()
                .filter(|l| l.as_bytes()[col] == bit)
                .collect();
            col += 1;
        }
        u32::from_str_radix(input.first().ok_or("empty").unwrap(), 2).unwrap()
    };
    let (oxygen, co2) = (rating(true), rating(false));
    println!("{}", oxygen * co2)
}
