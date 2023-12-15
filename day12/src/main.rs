use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter,
};

struct Rec<'a, 'b> {
    xs: &'a [u8],
    ys: &'b [usize],
    xs_len: usize,
    ys_len: usize,
    safe_index: usize,
    cache: HashMap<(usize, usize, usize), u64>,
}

impl<'a, 'b> Rec<'a, 'b> {
    fn new(xs: &'a [u8], ys: &'b [usize]) -> Self {
        let safe_index = xs
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'#')
            .map(|(i, _)| i + 1)
            .last()
            .unwrap_or(0);
        let cache = HashMap::new();
        Self {
            xs,
            ys,
            xs_len: xs.len(),
            ys_len: ys.len(),
            safe_index,
            cache,
        }
    }

    fn rec(&mut self, i: usize, j: usize, c: usize) -> u64 {
        // Stop condition
        if i == self.xs_len {
            return (j == self.ys_len || (j == self.ys_len - 1 && self.ys[j] == c)) as u64;
        }
        if j == self.ys_len {
            return (i >= self.safe_index) as u64;
        }
        // Check cache
        if let Some(&result) = self.cache.get(&(i, j, c)) {
            return result;
        }
        // Recursion
        let char = self.xs[i];
        let count = self.ys[j];
        let result = match (char, c) {
            (b'.', 0) => self.rec(i + 1, j, 0),
            (b'.', _) if c == count => self.rec(i + 1, j + 1, 0),
            (b'.', _) => 0,
            (b'#', _) if c == count => 0,
            (b'#', _) => self.rec(i + 1, j, c + 1),
            (b'?', 0) => self.rec(i + 1, j, 0) + self.rec(i + 1, j, 1),
            (b'?', _) if c == count => self.rec(i + 1, j + 1, 0),
            (b'?', _) => self.rec(i + 1, j, c + 1),
            _ => unreachable!(),
        };
        // Populate cache
        self.cache.insert((i, j, c), result);
        result
    }

    fn count_arrangements(xs: &'a [u8], ys: &'b [usize]) -> u64 {
        Self::new(xs, ys).rec(0, 0, 0)
    }
}

fn solve1(xs: &[(Vec<u8>, Vec<usize>)]) -> u64 {
    xs.iter().map(|(x, y)| Rec::count_arrangements(x, y)).sum()
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
            Rec::count_arrangements(&a, &b)
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
