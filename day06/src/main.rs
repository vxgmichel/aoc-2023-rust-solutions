use std::io::{self, BufRead};

fn solve1(ts: &[u64], ds: &[u64]) -> u64 {
    let mut result = 1;
    for (&t, &d) in ts.iter().zip(ds) {
        let delta = t.pow(2) as i64 - 4 * d as i64;
        if delta <= 0 {
            continue;
        }
        let mut x1 = ((t as f64 - (delta as f64).sqrt()) / 2_f64).ceil() as u64;
        let mut x2 = ((t as f64 + (delta as f64).sqrt()) / 2_f64).floor() as u64;
        if x1 * (t - x1) == d {
            x1 += 1;
        }
        if x2 * (t - x2) == d {
            x2 -= 1;
        }
        if x1 > x2 {
            continue;
        }
        result *= x2 - x1 + 1;
    }
    result
}

fn solve2(ts: &[u64], ds: &[u64]) -> u64 {
    let t: u64 = (ts.iter().map(|x| x.to_string()))
        .collect::<String>()
        .parse()
        .unwrap();
    let d: u64 = (ds.iter().map(|x| x.to_string()))
        .collect::<String>()
        .parse()
        .unwrap();
    solve1(&[t], &[d])
}

fn main() {
    let ts: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map_while(|x| x.parse().ok())
        .collect();
    let ds: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map_while(|x| x.parse().ok())
        .collect();
    println!("Part 1: {}", solve1(&ts, &ds));
    println!("Part 2: {}", solve2(&ts, &ds));
}
