use std::{
    collections::{HashMap, HashSet},
    env,
    io::{self, BufRead},
};

fn display_beam(m: i64, n: i64, cells: &Beam) {
    for i in 0..m {
        for j in 0..n {
            if !cells.contains_key(&(i, j)) {
                print!(" ");
            } else {
                let cell = &cells[&(i, j)];
                let north = cell.contains(&(-1, 0));
                let south = cell.contains(&(1, 0));
                let west = cell.contains(&(0, 1));
                let east = cell.contains(&(0, -1));
                let char = match (north, south, west, east) {
                    (true, true, true, true) => "┼",
                    (true, true, true, false) => "├",
                    (true, true, false, true) => "┤",
                    (true, false, true, true) => "┴",
                    (false, true, true, true) => "┬",
                    (true, false, false, true) => "┘",
                    (true, false, true, false) => "└",
                    (false, true, false, true) => "┐",
                    (false, true, true, false) => "┌",
                    (true, true, false, false) => "│",
                    (false, false, true, true) => "─",
                    _ => unreachable!("Invalid cell"),
                };
                print!("{}", char);
            }
        }
        println!();
    }
}

type Beam = HashMap<(i64, i64), HashSet<(i64, i64)>>;

fn draw_beam(xs: &[String], init: (i64, i64, i64, i64)) -> Beam {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let mut beams = vec![init];
    let mut seen = HashSet::new();
    let mut cells = HashMap::new();
    while !beams.is_empty() {
        let current = beams.pop().unwrap();
        let (x, y, dx, dy) = current;
        if x < 0 || y < 0 || x >= m || y >= n {
            continue;
        }
        if !seen.insert(current) {
            continue;
        }
        let dirs = cells.entry((x, y)).or_insert(HashSet::new());
        dirs.insert((-dx, -dy));
        match xs[x as usize].as_bytes()[y as usize] {
            b'.' => {
                beams.push((x + dx, y + dy, dx, dy));
                dirs.insert((dx, dy));
            }
            b'-' if dx == 0 => {
                beams.push((x + dx, y + dy, dx, dy));
                dirs.insert((dx, dy));
            }
            b'|' if dy == 0 => {
                beams.push((x + dx, y + dy, dx, dy));
                dirs.insert((dx, dy));
            }
            b'-' => {
                beams.push((x, y + 1, 0, 1));
                beams.push((x, y - 1, 0, -1));
                dirs.insert((0, 1));
                dirs.insert((0, -1));
            }
            b'|' => {
                beams.push((x + 1, y, 1, 0));
                beams.push((x - 1, y, -1, 0));
                dirs.insert((1, 0));
                dirs.insert((-1, 0));
            }
            b'\\' => {
                let (dx2, dy2) = (dy, dx);
                beams.push((x + dx2, y + dy2, dx2, dy2));
                dirs.insert((dx2, dy2));
            }
            b'/' => {
                let (dx2, dy2) = (-dy, -dx);
                beams.push((x + dx2, y + dy2, dx2, dy2));
                dirs.insert((dx2, dy2));
            }
            _ => panic!("Invalid character"),
        }
    }
    cells
}

fn solve1(xs: &[String], display: bool) -> u64 {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let cells = draw_beam(xs, (0, 0, 0, 1));
    if display {
        display_beam(m, n, &cells);
    }
    cells.len() as u64
}

fn solve2(xs: &[String], display: bool) -> u64 {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let mut result: Option<Beam> = None;
    for i in 0..m {
        let cells = draw_beam(xs, (i, 0, 0, 1));
        if result.as_ref().map(|x| x.len()).unwrap_or_default() < cells.len() {
            result = Some(cells);
        }
        let cells = draw_beam(xs, (i, n - 1, 0, -1));
        if result.as_ref().map(|x| x.len()).unwrap_or_default() < cells.len() {
            result = Some(cells);
        }
    }
    for j in 0..n {
        let cells = draw_beam(xs, (0, j, 1, 0));
        if result.as_ref().map(|x| x.len()).unwrap_or_default() < cells.len() {
            result = Some(cells);
        }
        let cells = draw_beam(xs, (m - 1, j, -1, 0));
        if result.as_ref().map(|x| x.len()).unwrap_or_default() < cells.len() {
            result = Some(cells);
        }
    }
    let result = result.unwrap();
    if display {
        display_beam(m, n, &result);
    }
    result.len() as u64
}

fn main() {
    let display = env::args()
        .nth(1)
        .map(|x| x == "--display")
        .unwrap_or_default();
    let vec: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();
    println!("Part 1: {}", solve1(&vec, display));
    println!("Part 2: {}", solve2(&vec, display));
}
