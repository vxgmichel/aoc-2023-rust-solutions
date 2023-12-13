use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}

impl Symmetry {
    fn value(&self) -> u64 {
        match self {
            Symmetry::Vertical(x) => *x as u64,
            Symmetry::Horizontal(x) => *x as u64 * 100,
        }
    }
}

fn check_vertical_symmetry(xs: &[Vec<bool>], column: usize) -> bool {
    let offset = 2 * column - 1;
    let n = xs[0].len();
    for x in xs {
        for j in 0..n {
            if offset < j || offset - j >= n {
                continue;
            }
            if x[j] != x[offset - j] {
                return false;
            }
        }
    }
    true
}

fn check_horizontal_symmetry(xs: &[Vec<bool>], row: usize) -> bool {
    let offset = 2 * row - 1;
    let m = xs.len();
    let n = xs[0].len();
    for i in 0..m {
        for j in 0..n {
            if offset < i || offset - i >= m {
                continue;
            }
            if xs[i][j] != xs[offset - i][j] {
                return false;
            }
        }
    }
    true
}

fn find_symmetry(xs: &[Vec<bool>]) -> Option<Symmetry> {
    if let Some(x) = (1..xs.len()).find(|&row| check_horizontal_symmetry(xs, row)) {
        return Some(Symmetry::Horizontal(x));
    }
    if let Some(x) = (1..xs[0].len()).find(|&column| check_vertical_symmetry(xs, column)) {
        return Some(Symmetry::Vertical(x));
    }
    None
}

fn solve1(xs: &[Vec<Vec<bool>>]) -> u64 {
    xs.iter().map(|x| find_symmetry(x).unwrap().value()).sum()
}

fn find_symmetry_after_change(xs: &[Vec<bool>]) -> u64 {
    let mut xs = xs.to_vec();
    let symmetry = find_symmetry(&xs).unwrap();
    for i in 0..xs.len() {
        for j in 0..xs[0].len() {
            xs[i][j] ^= true;
            if let Some(x) = (1..xs.len())
                .filter(|&row| check_horizontal_symmetry(&xs, row))
                .map(Symmetry::Horizontal)
                .find(|&x| x != symmetry)
            {
                return x.value();
            }
            if let Some(x) = (1..xs[0].len())
                .filter(|&column| check_vertical_symmetry(&xs, column))
                .map(Symmetry::Vertical)
                .find(|&x| x != symmetry)
            {
                return x.value();
            }
            xs[i][j] ^= true;
        }
    }
    panic!("No symmetry found");
}

fn solve2(xs: &[Vec<Vec<bool>>]) -> u64 {
    xs.iter().map(|x| find_symmetry_after_change(x)).sum()
}

fn main() {
    let mut xs = vec![];
    loop {
        let vec: Vec<Vec<bool>> = io::stdin()
            .lock()
            .lines()
            .map_while(|x| x.ok())
            .take_while(|x| !x.is_empty())
            .map(|x| x.bytes().map(|x| x == b'#').collect())
            .collect();
        if vec.is_empty() {
            break;
        } else {
            xs.push(vec)
        }
    }
    println!("Part 1: {}", solve1(&xs));
    println!("Part 2: {}", solve2(&xs));
}
