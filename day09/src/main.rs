use std::io::{self, BufRead};

fn extrapolate_right(xs: &[i64]) -> i64 {
    let mut current = xs.to_vec();
    let mut result = 0;
    while current.iter().any(|&x| x != 0) {
        result += current.last().unwrap();
        current = current.windows(2).map(|x| x[1] - x[0]).collect();
    }
    result
}

fn extrapolate_left(xs: &[i64]) -> i64 {
    let mut current = xs.to_vec();
    let mut result = 0;
    for sign in [1, -1].iter().cycle() {
        if current.iter().all(|&x| x == 0) {
            break;
        }
        result += sign * current.first().unwrap();
        current = current.windows(2).map(|x| x[1] - x[0]).collect();
    }
    result
}

fn solve1(xs: &[Vec<i64>]) -> i64 {
    xs.iter().map(|x| extrapolate_right(x)).sum()
}

fn solve2(xs: &[Vec<i64>]) -> i64 {
    xs.iter().map(|x| extrapolate_left(x)).sum()
}
fn main() {
    let vec: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
