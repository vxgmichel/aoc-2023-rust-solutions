use core::panic;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn find_start(xs: &[String]) -> (usize, usize) {
    for (i, line) in xs.iter().enumerate() {
        if let Some(j) = line.find('S') {
            return (i, j);
        }
    }
    panic!("No start found");
}

fn solve1(xs: &[String]) -> u32 {
    let start = find_start(xs);
    let mut previous = start;
    let mut current = (start.0, start.1 + 1);
    let mut length = 1;
    loop {
        let (a, b);
        let (x, y) = current;
        match xs[x].as_bytes()[y] {
            b'-' => {
                a = (x, y - 1);
                b = (x, y + 1);
            }
            b'|' => {
                a = (x - 1, y);
                b = (x + 1, y);
            }
            b'7' => {
                a = (x, y - 1);
                b = (x + 1, y);
            }
            b'L' => {
                a = (x - 1, y);
                b = (x, y + 1);
            }
            b'F' => {
                a = (x + 1, y);
                b = (x, y + 1);
            }
            b'J' => {
                a = (x, y - 1);
                b = (x - 1, y);
            }
            x => panic!("Unknown char {x}"),
        }
        if a == previous {
            previous = current;
            current = b;
        } else if b == previous {
            previous = current;
            current = a;
        } else {
            panic!("No match");
        }
        length += 1;
        if current == start {
            break;
        }
    }
    length / 2
}

fn solve2(xs: &[String]) -> u32 {
    let m = xs.len();
    let n = xs[0].len();
    let start = find_start(xs);
    let mut previous = start;
    let mut current = (start.0, start.1 + 1);
    let mut left_set = HashSet::new();
    let mut right_set = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    loop {
        visited.insert(current);
        let (x, y) = current;
        let left = if y > 0 { Some((x, y - 1)) } else { None };
        let right = if y < n - 1 { Some((x, y + 1)) } else { None };
        let up = if x > 0 { Some((x - 1, y)) } else { None };
        let down = if x < m - 1 { Some((x + 1, y)) } else { None };
        match xs[x].as_bytes()[y] {
            b'-' if previous == left.unwrap() => {
                previous = current;
                current = right.unwrap();
                up.map(|x| left_set.insert(x));
                down.map(|x| right_set.insert(x));
            }
            b'-' if previous == right.unwrap() => {
                previous = current;
                current = left.unwrap();
                down.map(|x| left_set.insert(x));
                up.map(|x| right_set.insert(x));
            }
            b'-' => panic!("No match"),
            b'|' if previous == up.unwrap() => {
                previous = current;
                current = down.unwrap();
                right.map(|x| left_set.insert(x));
                left.map(|x| right_set.insert(x));
            }
            b'|' if previous == down.unwrap() => {
                previous = current;
                current = up.unwrap();
                left.map(|x| left_set.insert(x));
                right.map(|x| right_set.insert(x));
            }
            b'|' => panic!("No match"),
            b'7' if previous == left.unwrap() => {
                previous = current;
                current = down.unwrap();
                up.map(|x| left_set.insert(x));
                right.map(|x| left_set.insert(x));
            }
            b'7' if previous == down.unwrap() => {
                previous = current;
                current = left.unwrap();
                right.map(|x| right_set.insert(x));
                up.map(|x| right_set.insert(x));
            }
            b'7' => panic!("No match"),
            b'L' if previous == up.unwrap() => {
                previous = current;
                current = right.unwrap();
                left.map(|x| right_set.insert(x));
                down.map(|x| right_set.insert(x));
            }
            b'L' if previous == right.unwrap() => {
                previous = current;
                current = up.unwrap();
                down.map(|x| left_set.insert(x));
                left.map(|x| left_set.insert(x));
            }
            b'L' => panic!("No match"),
            b'F' if previous == down.unwrap() => {
                previous = current;
                current = right.unwrap();
                left.map(|x| left_set.insert(x));
                up.map(|x| left_set.insert(x));
            }
            b'F' if previous == right.unwrap() => {
                previous = current;
                current = down.unwrap();
                up.map(|x| right_set.insert(x));
                left.map(|x| right_set.insert(x));
            }
            b'F' => panic!("No match"),
            b'J' if previous == left.unwrap() => {
                previous = current;
                current = up.unwrap();
                right.map(|x| right_set.insert(x));
                down.map(|x| right_set.insert(x));
            }
            b'J' if previous == up.unwrap() => {
                previous = current;
                current = left.unwrap();
                down.map(|x| left_set.insert(x));
                right.map(|x| left_set.insert(x));
            }
            b'J' => panic!("No match"),
            x => panic!("Unknown char {x}"),
        }
        if current == start {
            break;
        }
    }
    if let Some(result) = expand(m, n, &left_set, &visited) {
        return result.len() as u32;
    }
    if let Some(result) = expand(m, n, &right_set, &visited) {
        return result.len() as u32;
    }
    panic!("No solution found");
}

fn expand(
    m: usize,
    n: usize,
    cells: &HashSet<(usize, usize)>,
    border: &HashSet<(usize, usize)>,
) -> Option<HashSet<(usize, usize)>> {
    let mut current: Vec<_> = cells.difference(border).cloned().collect();
    let mut result = HashSet::new();
    while let Some((x, y)) = current.pop() {
        result.insert((x, y));
        if x == 0 || x == m || y == 0 || y == n {
            return None;
        }
        for item in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
            if !border.contains(item) && !result.contains(item) {
                current.push(*item);
            }
        }
    }
    Some(result)
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(|x| x.ok()).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
