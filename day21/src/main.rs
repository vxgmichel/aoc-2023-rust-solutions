use ahash::AHashSet as HashSet;
use std::io::{self, BufRead};

fn count(xs: &[Vec<bool>], start: (usize, usize), steps: u64) -> u64 {
    let parity = steps % 2;
    let mut previous_len = 0;
    let mut previous: HashSet<(usize, usize)> = HashSet::new();
    let mut current: HashSet<(usize, usize)> = [start].into_iter().collect();
    for i in 0..steps {
        if i % 2 == parity && previous_len == current.len() {
            return current.len() as u64;
        }
        previous_len = previous.len();
        for &(i, j) in &current {
            if i > 0 && !xs[i - 1][j] {
                previous.insert((i - 1, j));
            }
            if i < xs.len() - 1 && !xs[i + 1][j] {
                previous.insert((i + 1, j));
            }
            if j > 0 && !xs[i][j - 1] {
                previous.insert((i, j - 1));
            }
            if j < xs[i].len() - 1 && !xs[i][j + 1] {
                previous.insert((i, j + 1));
            }
        }
        (previous, current) = (current, previous);
    }
    current.len() as u64
}

fn solve1(xs: &[Vec<bool>], start: (usize, usize)) -> u64 {
    let steps = 64;
    count(xs, start, steps)
}

fn solve2(xs: &[Vec<bool>], start: (usize, usize)) -> u64 {
    let n = xs.len();
    let steps = 26501365;
    assert_eq!(n, xs[0].len());
    assert_eq!(n % 2, 1);
    assert_eq!(start.0, n / 2);
    assert_eq!(start.1, n / 2);
    let mut full_1 = count(xs, start, steps);
    let mut full_2 = count(xs, start, steps + 1);
    let mut result = full_1;
    let half_length = start.0 as u64 + 1;
    let full_length = 2 * half_length;
    let mut target = steps - full_length;
    for i in 1.. {
        if target >= 2 * n as u64 {
            result += 4 * full_2;
            result += 4 * i * full_1;
            (full_1, full_2) = (full_2, full_1);
        } else {
            result += count(xs, (0, n / 2), target + half_length);
            result += count(xs, (n / 2, 0), target + half_length);
            result += count(xs, (n - 1, n / 2), target + half_length);
            result += count(xs, (n / 2, n - 1), target + half_length);
            result += i * count(xs, (0, 0), target);
            result += i * count(xs, (0, n - 1), target);
            result += i * count(xs, (n - 1, 0), target);
            result += i * count(xs, (n - 1, n - 1), target);
        }
        if target < n as u64 {
            break;
        }
        target -= n as u64;
    }
    if target + half_length >= n as u64 {
        result += count(xs, (0, n / 2), target + half_length - n as u64);
        result += count(xs, (n / 2, 0), target + half_length - n as u64);
        result += count(xs, (n - 1, n / 2), target + half_length - n as u64);
        result += count(xs, (n / 2, n - 1), target + half_length - n as u64);
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(|x| x.ok()).collect();
    let start = vec
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars()
                .enumerate()
                .filter(|&(_, y)| y == 'S')
                .map(move |(j, _)| (i, j))
        })
        .next()
        .unwrap();
    let grid = vec
        .iter()
        .map(|x| x.chars().map(|y| y == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("Part 1: {}", solve1(&grid, start));
    println!("Part 2: {}", solve2(&grid, start));
}
