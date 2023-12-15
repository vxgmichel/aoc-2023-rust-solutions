use std::io::{self, BufRead};

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, x| (acc.wrapping_add(x).wrapping_mul(17)))
}

fn solve1(xs: &[&str]) -> u64 {
    xs.iter().map(|x| hash(x) as u64).sum()
}

fn solve2(xs: &[&str]) -> u64 {
    let mut boxes: [Vec<(&str, u8)>; 256] = core::array::from_fn(|_| vec![]);
    for x in xs {
        if *x.as_bytes().last().unwrap() == b'-' {
            let a = &x[0..x.len() - 1];
            let box_ = &mut boxes[hash(a) as usize];
            if let Some(i) = box_.iter().position(|&(x, _)| x == a) {
                box_.remove(i);
            }
        } else {
            let (a, b) = x.split_once('=').unwrap();
            let b: u8 = b.parse().unwrap();
            let box_ = &mut boxes[hash(a) as usize];
            match box_.iter().position(|&(x, _)| x == a) {
                Some(i) => {
                    box_[i] = (a, b);
                }
                None => box_.push((a, b)),
            };
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            (i as u64 + 1)
                * box_
                    .iter()
                    .enumerate()
                    .map(|(j, &(_, b))| (j as u64 + 1) * b as u64)
                    .sum::<u64>()
        })
        .sum()
}

fn main() {
    let line: String = io::stdin().lock().lines().next().unwrap().unwrap();
    let xs: Vec<&str> = line.split(',').collect();
    println!("Part 1: {}", solve1(&xs));
    println!("Part 2: {}", solve2(&xs));
}
