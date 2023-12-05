use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn solve1(xs: &[String]) -> u32 {
    let mut result = 0;
    for x in xs {
        let (_, x) = x.split_once(": ").unwrap();
        let (a, b) = x.split_once(" | ").unwrap();
        let set_a = a
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect::<HashSet<u32>>();
        let set_b = b
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect::<HashSet<u32>>();
        let count = set_a.intersection(&set_b).count();
        if count > 0 {
            result += 2_u32.pow(count as u32 - 1);
        }
    }
    result
}

fn solve2(xs: &[String]) -> u32 {
    let mut result = 0;
    let mut copies = VecDeque::new();
    for x in xs {
        let (_, x) = x.split_once(": ").unwrap();
        let (a, b) = x.split_once(" | ").unwrap();
        let set_a = a
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect::<HashSet<u32>>();
        let set_b = b
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect::<HashSet<u32>>();
        let value: u32 = copies.pop_front().unwrap_or_default() + 1;
        result += value;
        for i in 0..set_a.intersection(&set_b).count() {
            match copies.get_mut(i) {
                Some(x) => *x += value,
                None => copies.push_back(value),
            }
        }
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(|x| x.ok()).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
