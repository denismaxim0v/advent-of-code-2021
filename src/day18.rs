use itertools::Itertools;

#[derive(Debug, Clone)]
enum Element {
    Val(i32),
    Arr(Box<Element>, Box<Element>),
}

fn parse_element(s: &[u8], i: usize) -> (usize, Element) {
    match s[i] {
        b'[' => {
            let (i, e1) = parse_element(s, i + 1);
            let (i, e2) = parse_element(s, i + 1);
            (i + 1, Element::Arr(Box::new(e1), Box::new(e2)))
        }
        _ => (i + 1, Element::Val((s[i] - b'0') as i32)),
    }
}

fn split(e: &Element) -> (bool, Element) {
    match &e {
        Element::Val(x) if *x >= 10 => {
            let a = *x / 2;
            let b = *x - a;
            (
                true,
                Element::Arr(Box::new(Element::Val(a)), Box::new(Element::Val(b))),
            )
        }
        Element::Arr(e1, e2) => {
            let (got_split, a) = split(e1);
            if got_split {
                return (true, Element::Arr(Box::new(a), e2.clone()));
            }
            let (got_split, b) = split(e2);
            (got_split, Element::Arr(e1.clone(), Box::new(b)))
        }
        _ => (false, e.clone()),
    }
}

fn add_left(e: &Element, e2: &Option<Element>) -> Element {
    match (e, e2) {
        (_, None) => e.clone(),
        (Element::Val(x), Some(Element::Val(x2))) => Element::Val(*x + x2),
        (Element::Arr(a, b), _) => Element::Arr(Box::new(add_left(a, e2)), b.clone()),
        _ => panic!(),
    }
}

fn add_right(e: &Element, e2: &Option<Element>) -> Element {
    match (e, e2) {
        (_, None) => e.clone(),
        (Element::Val(x), Some(Element::Val(x2))) => Element::Val(*x + x2),
        (Element::Arr(a, b), _) => Element::Arr(a.clone(), Box::new(add_right(b, e2))),
        _ => panic!(),
    }
}

fn depth(e: &Element) -> usize {
    match e {
        Element::Val(_) => 0,
        Element::Arr(e1, e2) => 1 + std::cmp::max(depth(e1), depth(e2)),
    }
}

fn explode(e: &Element, n: usize) -> (bool, Option<Element>, Element, Option<Element>) {
    match e {
        Element::Arr(e1, e2) => {
            if n == 0 {
                return (true, Some(*e1.clone()), Element::Val(0), Some(*e2.clone()));
            }
            let (exp, left, a, right) = explode(e1, n - 1);
            if exp {
                return (
                    true,
                    left,
                    Element::Arr(Box::new(a), Box::new(add_left(e2, &right))),
                    None,
                );
            }
            let (exp, left, b, right) = explode(e2, n - 1);
            if exp {
                return (
                    true,
                    None,
                    Element::Arr(Box::new(add_right(&a, &left)), Box::new(b)),
                    right,
                );
            }
        }
        _ => {}
    }
    (false, None, e.clone(), None)
}

fn reduce(mut e: Element) -> Element {
    loop {
        let d = depth(&e);
        if d > 4 {
            let (more, _, e2, _) = explode(&e, depth(&e) - 1);
            e = e2;
            if more {
                continue;
            }
        }
        let (more, e2) = split(&e);
        e = e2;
        if !more {
            break e;
        }
    }
}

fn add(e1: &Element, e2: &Element) -> Element {
    reduce(Element::Arr(Box::new(e1.clone()), Box::new(e2.clone())))
}

fn magnitude(e: &Element) -> i32 {
    match e {
        Element::Val(x) => *x,
        Element::Arr(e1, e2) => 3 * magnitude(e1) + 2 * magnitude(e2),
    }
}

pub fn part1(inp: String) {
    let input = inp
        .lines()
        .map(|l| parse_element(l.as_bytes(), 0).1)
        .collect::<Vec<_>>();
    let res = magnitude(
        &input[1..]
            .iter()
            .fold(input[0].clone(), |e1, e2| add(&e1, e2)),
    );

    println!("{}", res);
}

pub fn part2(inp: String) {
    let input = inp
        .lines()
        .map(|l| parse_element(l.as_bytes(), 0).1)
        .collect::<Vec<_>>();
    let res = input
        .iter()
        .tuple_combinations()
        .map(|(e1, e2)| magnitude(&add(e1, e2)))
        .max()
        .unwrap();
    println!("{}", res);
}
