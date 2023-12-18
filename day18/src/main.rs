use std::{
    collections::{HashMap, HashSet},
    env,
    io::{self, BufRead},
};

fn print_border(borders: &HashSet<(i64, i64)>, min_x: i64, max_x: i64, min_y: i64, max_y: i64) {
    for x in (min_x..=max_x).step_by(2) {
        for y in min_y..=max_y {
            let a = borders.contains(&(x, y));
            let b = borders.contains(&(x + 1, y));
            match (a, b) {
                (true, true) => print!("█"),
                (true, false) => print!("▀"),
                (false, true) => print!("▄"),
                (false, false) => print!(" "),
            }
        }
        println!();
    }
}

fn print_area(outside: &HashSet<(i64, i64)>, min_x: i64, max_x: i64, min_y: i64, max_y: i64) {
    for x in (min_x..=max_x).step_by(2) {
        for y in min_y..=max_y {
            let a = !outside.contains(&(x, y));
            let b = !outside.contains(&(x + 1, y));
            match (a, b) {
                (true, true) => print!("█"),
                (true, false) => print!("▀"),
                (false, true) => print!("▄"),
                (false, false) => print!(" "),
            }
        }
        println!();
    }
}

fn solve1(xs: &[(char, u64)], display: bool) -> u64 {
    let mut borders = HashSet::new();
    let mut current = (0, 0);
    for (a, b) in xs {
        let (dx, dy) = match a {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("Invalid direction"),
        };
        borders.insert(current);
        for _ in 0..*b {
            current.0 += dx;
            current.1 += dy;
            borders.insert(current);
        }
    }
    let offset = 10;
    let min_x = borders.iter().map(|(x, _)| x).min().unwrap() - offset;
    let max_x = borders.iter().map(|(x, _)| x).max().unwrap() + offset;
    let min_y = borders.iter().map(|(_, y)| y).min().unwrap() - offset;
    let max_y = borders.iter().map(|(_, y)| y).max().unwrap() + offset;
    if display {
        print_border(&borders, min_x, max_x, min_y, max_y);
    }
    let mut current = vec![(min_x, min_y)];
    let mut outside = HashSet::new();
    while let Some(cell) = current.pop() {
        if cell.0 < min_x || cell.0 > max_x || cell.1 < min_y || cell.1 > max_y {
            continue;
        }
        if borders.contains(&cell) {
            continue;
        }
        if outside.contains(&cell) {
            continue;
        }
        outside.insert(cell);
        current.push((cell.0 + 1, cell.1));
        current.push((cell.0 - 1, cell.1));
        current.push((cell.0, cell.1 + 1));
        current.push((cell.0, cell.1 - 1));
    }
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    if display {
        print_area(&outside, min_x, max_x, min_y, max_y);
    }
    (area - outside.len() as i64) as u64
}

fn solve2(xs: &[(char, u64)]) -> u64 {
    // Collect points
    let mut point_map = HashMap::new();

    // Follow border
    let mut current: (i64, i64) = (0, 0);
    for (a, b) in xs {
        point_map
            .entry(current.0)
            .or_insert_with(HashSet::new)
            .insert(current.1);
        let (dx, dy) = match a {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("Invalid direction"),
        };
        current = (current.0 + dx * *b as i64, current.1 + dy * *b as i64);
    }

    // Sort points
    let mut sorted_points = point_map
        .into_iter()
        .map(|(key, value)| {
            let mut vec: Vec<_> = value.iter().cloned().collect();
            vec.sort();
            (key, vec)
        })
        .collect::<Vec<_>>();
    sorted_points.sort();

    // Prepare result
    let mut result = 0;
    let mut verticals: Vec<i64> = vec![];
    let mut current_row = sorted_points.iter().min().unwrap().0;

    // Loop over rows
    for (row, corners) in sorted_points {
        // Count inside cells over a single row
        let mut inside_count = 0;
        for index in 0..verticals.len().saturating_sub(1) {
            if index % 2 == 0 {
                inside_count += verticals[index + 1] - verticals[index] + 1;
            }
        }

        // Increase result over multiple rows
        result += inside_count * (row - current_row + 1);
        current_row = row;

        // Update verticals and compute overlap
        let mut overlap = 0;
        let mut new_verticals: Vec<i64> = vec![];

        // Keep track of the row state
        let mut inside = false;
        let mut border = false;
        let mut previous_index = i64::MIN;

        // Iterate from both corners and verticals
        let mut corner_iter = corners.iter().cloned();
        let mut vertical_iter = verticals.iter().cloned();
        let mut corner = corner_iter.next();
        let mut vertical = vertical_iter.next();

        while let Some(index) = vertical
            .and_then(|x| corner.map(|y| x.min(y)))
            .or(vertical)
            .or(corner)
        {
            if vertical != corner {
                new_verticals.push(index);
            }
            if inside && !border {
                overlap += index - previous_index + 1;
            }
            if Some(index) == vertical {
                inside = !inside;
                vertical = vertical_iter.next();
            }
            if Some(index) == corner {
                border = !border;
                inside = !inside;
                corner = corner_iter.next();
            }
            previous_index = index;
        }
        verticals = new_verticals;

        // Remove overlap
        result -= overlap;
    }
    result as u64
}

fn parse_hex(s: &str) -> (char, u64) {
    let a = match s.as_bytes()[7] {
        b'0' => 'R',
        b'1' => 'D',
        b'2' => 'L',
        b'3' => 'U',
        _ => panic!("Invalid direction"),
    };
    let b = u64::from_str_radix(&s[2..7], 16).unwrap();
    (a, b)
}
fn main() {
    let display = env::args().nth(1).unwrap_or_default() == "--display";
    let mut vec1: Vec<(char, u64)> = vec![];
    let mut vec2: Vec<(char, u64)> = vec![];
    for x in io::stdin().lock().lines() {
        let line = x.unwrap();
        let args = line.split_whitespace().collect::<Vec<_>>();
        if let [a, b, c] = args.as_slice() {
            let a = a.chars().next().unwrap();
            let b = b.parse::<u64>().unwrap();
            vec1.push((a, b));
            vec2.push(parse_hex(c));
        } else {
            panic!("Invalid input");
        }
    }
    println!("Part 1: {}", solve1(&vec1, display));
    println!("Part 2: {}", solve2(&vec2));
}
