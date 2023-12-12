use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter,
};

fn rec(
    xs: &[u8],
    ys: &[usize],
    safe_index: usize,
    cache: &mut HashMap<(usize, usize, usize), u64>,
    i: usize,
    j: usize,
    c: usize,
) -> u64 {
    // Stop condition
    if i == xs.len() {
        return (j == ys.len() || (j == ys.len() - 1 && ys[j] == c)) as u64;
    }
    if j == ys.len() {
        return (i >= safe_index) as u64;
    }
    // Check cache
    if let Some(&result) = cache.get(&(i, j, c)) {
        return result;
    }
    // Recursion
    let char = xs[i];
    let count = ys[j];
    let result = match (char, c) {
        (b'.', 0) => rec(xs, ys, safe_index, cache, i + 1, j, 0),
        (b'.', _) if c == count => rec(xs, ys, safe_index, cache, i + 1, j + 1, 0),
        (b'.', _) => 0,
        (b'#', _) if c == count => 0,
        (b'#', _) => rec(xs, ys, safe_index, cache, i + 1, j, c + 1),
        (b'?', 0) => {
            rec(xs, ys, safe_index, cache, i + 1, j, 0)
                + rec(xs, ys, safe_index, cache, i + 1, j, 1)
        }
        (b'?', _) if c == count => rec(xs, ys, safe_index, cache, i + 1, j + 1, 0),
        (b'?', _) => rec(xs, ys, safe_index, cache, i + 1, j, c + 1),
        _ => unreachable!(),
    };
    // Populate cache
    cache.insert((i, j, c), result);
    result
}

fn count_arrangements(xs: &[u8], ys: &[usize]) -> u64 {
    let mut cache: HashMap<_, u64> = HashMap::new();
    let safe_index = xs
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == b'#')
        .map(|(i, _)| i + 1)
        .last()
        .unwrap_or(0);
    rec(xs, ys, safe_index, &mut cache, 0, 0, 0)
}

fn solve1(xs: &[(Vec<u8>, Vec<usize>)]) -> u64 {
    xs.iter().map(|(x, y)| count_arrangements(x, y)).sum()
}

fn solve2(xs: &[(Vec<u8>, Vec<usize>)]) -> u64 {
    xs.iter()
        .map(|(x, y)| {
            let mut a: Vec<u8> = vec![];
            for _ in 0..4 {
                a.extend(x);
                a.push(b'?');
            }
            a.extend(x);
            let b: Vec<usize> = iter::repeat(y).take(5).flatten().cloned().collect();
            count_arrangements(&a, &b)
        })
        .sum()
}

fn main() {
    let vec: Vec<(Vec<u8>, Vec<usize>)> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let (a, b) = line.split_once(' ').unwrap();
            let xs = a.as_bytes().to_owned();
            let ys: Vec<usize> = b.split(',').map(|x| x.parse().unwrap()).collect();
            (xs, ys)
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
