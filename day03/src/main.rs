use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_number(xs: &[String], row: usize, i: usize, j: usize) -> u32 {
    let base_line = &xs[row];
    let (x1, y1) = (row.saturating_sub(1), i.saturating_sub(1));
    let (x2, y2) = ((row + 2).min(xs.len()), (j + 1).min(base_line.len()));
    for x in xs.iter().take(x2).skip(x1) {
        let line = x.as_bytes();
        for y in line.iter().take(y2).skip(y1) {
            if !y.is_ascii_digit() && (*y as char) != '.' {
                return base_line[i..j].parse::<u32>().unwrap();
            }
        }
    }
    0
}

fn solve1(xs: &[String]) -> u32 {
    let mut result = 0;
    for (row, line) in xs.iter().enumerate() {
        let mut i = 0;
        while i < line.len() {
            if !line.as_bytes()[i].is_ascii_digit() {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < line.len() && line.as_bytes()[j].is_ascii_digit() {
                j += 1;
            }
            result += parse_number(xs, row, i, j);
            i = j + 1;
        }
    }
    result
}

fn find_gear(xs: &[String], row: usize, i: usize, j: usize) -> Option<(usize, usize)> {
    let base_line = &xs[row];
    let (x1, y1) = (row.saturating_sub(1), i.saturating_sub(1));
    let (x2, y2) = ((row + 2).min(xs.len()), (j + 1).min(base_line.len()));
    for (x, line) in xs.iter().enumerate().take(x2).skip(x1) {
        if let Some(y) = line[y1..y2].find('*') {
            return Some((x, y1 + y));
        }
    }
    None
}

fn solve2(xs: &[String]) -> u32 {
    let mut result = 0;
    let mut map = HashMap::new();
    for (row, line) in xs.iter().enumerate() {
        let mut i = 0;
        while i < line.len() {
            if !line.as_bytes()[i].is_ascii_digit() {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < line.len() && line.as_bytes()[j].is_ascii_digit() {
                j += 1;
            }
            let value = parse_number(xs, row, i, j);
            if let Some(key) = find_gear(xs, row, i, j) {
                match map.get(&key) {
                    Some(other) => result += value * other,
                    None => {
                        map.insert(key, value);
                    }
                }
            }
            i = j + 1;
        }
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
