use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn solve1(moves: &str, map: &HashMap<String, (String, String)>) -> u64 {
    let mut current = "AAA";
    let mut result = 0;
    for m in moves.chars().cycle() {
        let (left, right) = map.get(current).unwrap();
        match m {
            'L' => current = left,
            'R' => current = right,
            _ => panic!("Invalid move"),
        }
        result += 1;
        if current == "ZZZ" {
            break;
        }
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn solve2(moves: &str, map: &HashMap<String, (String, String)>) -> u64 {
    let mut result = 1;
    for start in map.keys().filter(|x| x.ends_with('A')) {
        let mut cache = HashMap::new();
        let mut current = start;
        let mut count = 0;
        let mut offset = 0;
        let mut ends = Vec::new();
        for (i, m) in moves.chars().enumerate().cycle() {
            // Update cache
            if cache.contains_key(&(i, current)) {
                offset = *cache.get(&(i, current)).unwrap();
                break;
            }
            cache.insert((i, current), count);
            // Update current and count
            let (left, right) = map.get(current).unwrap();
            match m {
                'L' => current = left,
                'R' => current = right,
                _ => panic!("Invalid move"),
            }
            count += 1;
            // Check for endpoints
            if current.ends_with('Z') {
                ends.push(count);
            }
        }
        assert!(ends.len() == 1);
        let end = ends.pop().unwrap();
        assert_eq!(end, count - offset);
        result = result * end / gcd(result, end);
    }
    result
}

fn main() {
    let moves: String = io::stdin().lock().lines().next().unwrap().unwrap();
    let map: HashMap<String, (String, String)> = io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(|x| {
            let line = x.unwrap();
            let (key, values) = line.split_once(" = (").unwrap();
            let (left, right) = values.split_once(", ").unwrap();
            let x: String = right.chars().take(right.len() - 1).collect();
            (key.to_owned(), (left.to_owned(), x))
        })
        .collect();
    println!("Part 1: {}", solve1(&moves, &map));
    println!("Part 2: {}", solve2(&moves, &map));
}
