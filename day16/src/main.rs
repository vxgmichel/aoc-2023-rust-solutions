use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::io::{self, BufRead};

type Ray = (i64, i64, i64, i64);
type BeamInfo = (HashSet<(i64, i64)>, Option<(Ray, Ray)>);
type Cache = HashMap<Ray, BeamInfo>;

fn compute_beam_info(xs: &[String], init: Ray) -> BeamInfo {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let mut current = init;
    let mut cells = HashSet::new();
    loop {
        let (x, y, dx, dy) = current;
        if x < 0 || y < 0 || x >= m || y >= n {
            break;
        }
        if !cells.is_empty() && current == init {
            break;
        }
        cells.insert((x, y));
        match xs[x as usize].as_bytes()[y as usize] {
            b'.' => {
                current = (x + dx, y + dy, dx, dy);
            }
            b'\\' => {
                let (dx2, dy2) = (dy, dx);
                current = (x + dx2, y + dy2, dx2, dy2);
            }
            b'/' => {
                let (dx2, dy2) = (-dy, -dx);
                current = (x + dx2, y + dy2, dx2, dy2);
            }
            b'-' if dx == 0 => {
                current = (x + dx, y + dy, dx, dy);
            }
            b'|' if dy == 0 => {
                current = (x + dx, y + dy, dx, dy);
            }
            b'-' => {
                let child1 = (x, y + 1, 0, 1);
                let child2 = (x, y - 1, 0, -1);
                return (cells, Some((child1, child2)));
            }
            b'|' => {
                let child1 = (x + 1, y, 1, 0);
                let child2 = (x - 1, y, -1, 0);
                return (cells, Some((child1, child2)));
            }
            _ => panic!("Invalid character"),
        }
    }
    (cells, None)
}

fn get_borders(xs: &[String]) -> Vec<Ray> {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let mut nodes = vec![];
    for i in 0..m {
        nodes.extend([(i, 0, 0, 1), (i, n - 1, 0, -1)]);
    }
    for j in 0..n {
        nodes.extend([(0, j, 1, 0), (m - 1, j, -1, 0)]);
    }
    nodes
}

fn build_cache(xs: &[String]) -> Cache {
    let m = xs.len() as i64;
    let n = xs[0].len() as i64;
    let mut cache = HashMap::new();
    let mut nodes = get_borders(xs);
    for i in 0..m {
        for j in 0..n {
            let c = xs[i as usize].as_bytes()[j as usize];
            if c == b'|' {
                nodes.extend([(i - 1, j, -1, 0), (i + 1, j, 1, 0)]);
            } else if c == b'-' {
                nodes.extend([(i, j - 1, 0, -1), (i, j + 1, 0, 1)]);
            }
        }
    }
    for node in nodes {
        cache.insert(node, compute_beam_info(xs, node));
    }
    cache
}

fn count_cells(init: Ray, cache: &mut Cache) -> u64 {
    let mut result: HashSet<(i64, i64)> = HashSet::new();
    let mut seen = HashSet::new();
    let mut current = vec![init];
    while let Some(node) = current.pop() {
        let (cells, next) = cache.get(&node).unwrap();
        seen.insert(node);
        result.extend(cells);
        if let Some((child1, child2)) = next {
            for child in [child1, child2] {
                if !seen.contains(child) {
                    current.push(*child);
                }
            }
        }
    }
    result.len() as u64
}

fn solve1(cache: &mut Cache) -> u64 {
    count_cells((0, 0, 0, 1), cache)
}

fn solve2(xs: &[String], cache: &mut Cache) -> u64 {
    get_borders(xs)
        .iter()
        .map(|&node| count_cells(node, cache))
        .max()
        .unwrap()
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();
    let mut cache = build_cache(&vec);
    println!("Part 1: {}", solve1(&mut cache));
    println!("Part 2: {}", solve2(&vec, &mut cache));
}
