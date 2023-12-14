use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn roll_north(xs: &mut [Vec<u8>]) {
    for j in 0..xs[0].len() {
        let mut k = 0;
        for i in 0..xs.len() {
            match xs[i][j] {
                b'.' => continue,
                b'#' => {
                    k = i;
                }
                b'O' if i == k => {}
                b'O' => {
                    xs[k][j] = b'O';
                    xs[i][j] = b'.';
                }
                _ => panic!("Invalid char"),
            }
            k += 1;
        }
    }
}

fn roll_west(xs: &mut [Vec<u8>]) {
    for x in xs {
        let mut k = 0;
        for j in 0..x.len() {
            match x[j] {
                b'.' => continue,
                b'#' => {
                    k = j;
                }
                b'O' if j == k => {}
                b'O' => {
                    x[k] = b'O';
                    x[j] = b'.';
                }
                _ => panic!("Invalid char"),
            }
            k += 1;
        }
    }
}

fn roll_south(xs: &mut [Vec<u8>]) {
    for j in 0..xs[0].len() {
        let mut k = xs.len() - 1;
        for i in (0..xs.len()).rev() {
            match xs[i][j] {
                b'.' => continue,
                b'O' if i == k => {}
                b'O' => {
                    xs[k][j] = b'O';
                    xs[i][j] = b'.';
                }
                b'#' => {
                    k = i;
                }
                _ => panic!("Invalid char"),
            }
            k = k.saturating_sub(1)
        }
    }
}

fn roll_east(xs: &mut [Vec<u8>]) {
    for x in xs {
        let mut k = x.len() - 1;
        for j in (0..x.len()).rev() {
            match x[j] {
                b'.' => continue,
                b'O' if j == k => {}
                b'O' => {
                    x[k] = b'O';
                    x[j] = b'.';
                }
                b'#' => {
                    k = j;
                }
                _ => panic!("Invalid char"),
            }
            k = k.saturating_sub(1);
        }
    }
}

fn weight(xs: &[Vec<u8>]) -> u64 {
    xs.iter()
        .enumerate()
        .map(|(i, x)| x.iter().filter(|&y| *y == b'O').count() as u64 * (xs.len() - i) as u64)
        .sum()
}

fn solve1(xs: &[Vec<u8>]) -> u64 {
    let mut xs = xs.to_vec();
    roll_north(&mut xs);
    weight(&xs)
}

fn solve2(xs: &[Vec<u8>]) -> u64 {
    let mut xs = xs.to_vec();
    let mut map = HashMap::new();
    let mut weights = vec![];
    while !map.contains_key(&xs) {
        map.insert(xs.clone(), weights.len());
        weights.push(weight(&xs));
        roll_north(&mut xs);
        roll_west(&mut xs);
        roll_south(&mut xs);
        roll_east(&mut xs);
    }
    let offset = map[&xs];
    let cycle = weights.len() - offset;
    let index = offset + (1_000_000_000 - offset) % cycle;
    weights[index]
}
fn main() {
    let vec: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().as_bytes().to_owned())
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
