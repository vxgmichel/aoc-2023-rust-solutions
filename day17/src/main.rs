use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Left = 1,
    Right = 2,
    Down = 3,
}

impl Direction {
    const VALUES: [Self; 4] = [Self::Down, Self::Right, Self::Up, Self::Left];

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

fn solve1(xs: &[Vec<u8>]) -> u64 {
    let m = xs.len();
    let n = xs[0].len();
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![[[false; 4]; 4]; n]; m];
    heap.push((0, 0, (0, 0, Direction::Right, 0)));
    heap.push((0, 0, (0, 0, Direction::Down, 0)));

    // Pop from the heap
    while let Some(item) = heap.pop() {
        let (_, score, node) = item;
        let (i, j, d, l) = node;

        // Check if we reached the end position
        if i == m - 1 && j == n - 1 {
            return score;
        }

        // Check if we already visited this node
        if visited[i][j][d as usize][l] {
            continue;
        }
        visited[i][j][d as usize][l] = true;

        // Loop over all possible directions
        for nd in Direction::VALUES {
            // Do not go back
            if nd.opposite() == d {
                continue;
            }

            // Do not go further after 3 steps
            let nl = if nd == d { l + 1 } else { 1 };
            if nl > 3 {
                continue;
            }

            // Do not go outside the grid
            let (ni, nj): (usize, usize) = match nd {
                Direction::Up if i > 0 => (i - 1, j),
                Direction::Left if j > 0 => (i, j - 1),
                Direction::Down if i < m - 1 => (i + 1, j),
                Direction::Right if j < n - 1 => (i, j + 1),
                _ => continue,
            };
            // Push onto the heap
            let new_score = xs[ni][nj] as u64 + score;
            heap.push((-(new_score as i64), new_score, (ni, nj, nd, nl)));
        }
    }
    panic!("No solution found");
}

fn solve2(xs: &[Vec<u8>]) -> u64 {
    let m = xs.len();
    let n = xs[0].len();
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![[[false; 11]; 4]; n]; m];
    heap.push((0, 0, (0, 0, Direction::Right, 0)));
    heap.push((0, 0, (0, 0, Direction::Down, 0)));

    // Pop from the heap
    while let Some(item) = heap.pop() {
        let (_, score, node) = item;
        let (i, j, d, l) = node;

        // Check if we reached the end position
        if i == m - 1 && j == n - 1 && l >= 4 {
            return score;
        }

        // Check if we already visited this node
        if visited[i][j][d as usize][l] {
            continue;
        }
        visited[i][j][d as usize][l] = true;

        // Loop over all possible directions
        for nd in Direction::VALUES {
            // Do not go back
            if nd.opposite() == d {
                continue;
            }

            // Go at least 4 steps
            if l < 4 && nd != d {
                continue;
            }

            // Do not go further than 10 steps
            let nl = if nd == d { l + 1 } else { 1 };
            if nl > 10 {
                continue;
            }

            // Do not go outside the grid
            let (ni, nj): (usize, usize) = match nd {
                Direction::Up if i > 0 => (i - 1, j),
                Direction::Left if j > 0 => (i, j - 1),
                Direction::Down if i < m - 1 => (i + 1, j),
                Direction::Right if j < n - 1 => (i, j + 1),
                _ => continue,
            };
            // Push onto the heap
            let new_score = xs[ni][nj] as u64 + score;
            heap.push((-(new_score as i64), new_score, (ni, nj, nd, nl)));
        }
    }
    panic!("No solution found");
}

fn main() {
    let vec: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
