use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn solve1(grid: &[Vec<char>]) -> u32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let start = (0, 1);
    let stop = (grid.len() as i32 - 1, grid[0].len() as i32 - 2);
    let mut queue = vec![start];
    let mut states = HashMap::new();
    let mut result = 0;
    while !queue.is_empty() {
        let current = *queue.last().unwrap();
        // Stop condition
        if current == stop {
            result = result.max(queue.len() as u32 - 1);
            queue.pop();
            continue;
        }
        // First visit for given length
        if !states.contains_key(&current) {
            let dirs = match grid[current.0 as usize][current.1 as usize] {
                '<' => vec![(0, -1)],
                '>' => vec![(0, 1)],
                'v' => vec![(1, 0)],
                '^' => vec![(-1, 0)],
                '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
                _ => panic!("Invalid char"),
            };
            let children = dirs
                .iter()
                .map(|(dx, dy)| (current.0 + dx, current.1 + dy))
                .filter(|&(x, y)| x >= 0 && y >= 0 && x < m && y < n)
                .filter(|&(x, y)| grid[x as usize][y as usize] != '#')
                .filter(|k| !states.contains_key(k))
                .collect::<Vec<_>>();
            states.insert(current, children);
        }
        // Get next child
        let children = states.get_mut(&current).unwrap();
        if let Some(child) = children.pop() {
            queue.push(child);
        // No child, backtrack
        } else {
            states.remove(&current);
            queue.pop();
        }
    }
    result
}

fn main() {
    let vec: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    println!("Part 1: {}", solve1(&vec));
    // println!("Part 2: {}", solve2(&vec));
}
