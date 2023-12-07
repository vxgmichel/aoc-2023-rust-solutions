use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
};

fn card_to_value_1(x: char) -> u8 {
    match x {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value"),
    }
}

fn hand_to_values_1(hand: &str) -> Vec<(u8, u8)> {
    let mut map = HashMap::new();
    for value in hand.chars().map(card_to_value_1) {
        map.entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut result: Vec<(u8, u8)> = map.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    result
}

fn compare_hands_1(a: &str, b: &str) -> Ordering {
    let a_values = hand_to_values_1(a);
    let b_values = hand_to_values_1(b);
    for (x, y) in a_values.iter().zip(b_values) {
        match x.1.cmp(&y.1) {
            Ordering::Equal => continue,
            x => return x,
        }
    }
    for (x, y) in a.chars().zip(b.chars()) {
        match card_to_value_1(x).cmp(&card_to_value_1(y)) {
            Ordering::Equal => continue,
            x => return x,
        }
    }
    Ordering::Equal
}

fn solve1(xs: &[(String, u64)]) -> u64 {
    let mut hands = xs.iter().collect::<Vec<_>>();
    hands.sort_by(|a, b| compare_hands_1(&a.0, &b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, y))| (i as u64 + 1) * y)
        .sum()
}

fn card_to_value_2(x: char) -> u8 {
    match x {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value"),
    }
}

fn hand_to_values_2(hand: &str) -> Vec<(u8, u8)> {
    let mut map = HashMap::new();
    let mut jokers = 0;
    for value in hand.chars().map(card_to_value_2) {
        if value == 1 {
            jokers += 1;
            continue;
        }
        map.entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut result: Vec<(u8, u8)> = map.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    match result.first_mut() {
        Some((_, x)) => *x += jokers,
        None => result.push((1, jokers)),
    }
    result
}

fn compare_hands_2(a: &str, b: &str) -> Ordering {
    let a_values = hand_to_values_2(a);
    let b_values = hand_to_values_2(b);
    for (x, y) in a_values.iter().zip(b_values) {
        match x.1.cmp(&y.1) {
            Ordering::Equal => continue,
            x => return x,
        }
    }
    for (x, y) in a.chars().zip(b.chars()) {
        match card_to_value_2(x).cmp(&card_to_value_2(y)) {
            Ordering::Equal => continue,
            x => return x,
        }
    }
    Ordering::Equal
}

fn solve2(xs: &[(String, u64)]) -> u64 {
    let mut hands = xs.iter().collect::<Vec<_>>();
    hands.sort_by(|a, b| compare_hands_2(&a.0, &b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, y))| (i as u64 + 1) * y)
        .sum()
}

fn main() {
    let vec: Vec<(String, u64)> = io::stdin()
        .lock()
        .lines()
        .map_while(|x| x.ok())
        .map(|x| {
            let (a, b) = x.split_once(' ').unwrap();
            (a.to_string(), b.parse().unwrap())
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
