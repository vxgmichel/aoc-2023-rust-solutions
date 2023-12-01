use std::io::{self, BufRead};

fn solve1(xs: &[String]) -> u32 {
    let mut result: u32 = 0;
    for line in xs {
        let values = line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<_>>();
        let value = values.first().unwrap() * 10 + values.last().unwrap();
        result += value;
    }
    result
}

fn solve2(xs: &[String]) -> u32 {
    let mut result: u32 = 0;
    for line in xs {
        let mut indexed_values = line
            .chars()
            .enumerate()
            .filter_map(|(i, x)| x.to_digit(10).map(|x| (i, x)))
            .collect::<Vec<_>>();
        for (i, x) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            let value = (i + 1) as u32;
            indexed_values.extend(line.match_indices(x).map(|(i, _)| (i, value)))
        }
        indexed_values.sort();
        let values = indexed_values
            .iter()
            .map(|(_, digit)| *digit)
            .collect::<Vec<_>>();
        let value = values.first().unwrap() * 10 + values.last().unwrap();
        result += value;
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(|x| x.ok()).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
