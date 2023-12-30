use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Point = (i32, i32);
type Graph = Vec<[(usize, u32); 5]>;

fn remove_slopes(grid: &mut [Vec<char>]) {
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if *c == 'v' || *c == '^' || *c == '<' || *c == '>' {
                *c = '.';
            }
        }
    }
}

fn follow(
    grid: &[Vec<char>],
    start: (i32, i32),
    then: (i32, i32),
) -> (Point, Point, Vec<Point>, u32) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut previous = start;
    let mut current = then;
    let mut length = 1;
    loop {
        let dirs = match grid[current.0 as usize][current.1 as usize] {
            '<' => vec![(0, -1)],
            '>' => vec![(0, 1)],
            'v' => vec![(1, 0)],
            '^' => vec![(-1, 0)],
            '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            _ => panic!("Invalid path"),
        };
        let nodes = dirs
            .iter()
            .map(|(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|&(x, y)| x >= 0 && y >= 0 && x < m && y < n)
            .filter(|&(x, y)| grid[x as usize][y as usize] != '#')
            .filter(|&(x, y)| (x, y) != previous)
            .collect::<Vec<_>>();
        if nodes.len() != 1 {
            return (previous, current, nodes, length);
        }
        previous = current;
        current = nodes[0];
        length += 1;
    }
}

fn make_graph(grid: &[Vec<char>]) -> (usize, usize, Graph) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let start = (0, 1);
    let then = (1, 1);
    let stop = (m - 1, n - 2);
    let mut indexes: HashMap<(i32, i32), usize> = [(start, 0)].into_iter().collect();
    let mut graph = vec![[(0, 0); 5]];
    let mut queue = vec![(start, then)];
    let mut seen: HashSet<_> = HashSet::new();
    while let Some((node, then)) = queue.pop() {
        if seen.contains(&(node, then)) {
            continue;
        }
        seen.insert((node, then));
        let (_, current, nodes, length) = follow(grid, node, then);
        let node_index = *indexes.get(&node).unwrap();
        let current_index = *indexes.entry(current).or_insert_with(|| {
            graph.push([(0, 0); 5]);
            graph.len() - 1
        });
        for i in 0..4 {
            if graph[node_index][i].1 == 0 {
                graph[node_index][i] = (current_index, length);
                break;
            }
        }
        queue.extend(nodes.iter().map(|&n| (current, n)));
    }
    (0, *indexes.get(&stop).unwrap(), graph)
}

fn rec(
    graph: &Graph,
    node: usize,
    stop: usize,
    seen: &mut Vec<bool>,
    length: u32,
    result: &mut u32,
) {
    if node == stop {
        *result = (*result).max(length);
    }
    seen[node] = true;
    for &(child, child_length) in graph[node].iter() {
        if child_length == 0 {
            break;
        }
        if !seen[child] {
            rec(graph, child, stop, seen, length + child_length, result);
        }
    }
    seen[node] = false;
}

fn solve(grid: &[Vec<char>]) -> u32 {
    let (start, stop, graph) = make_graph(grid);
    let mut seen = vec![false; graph.len()];
    seen[start] = true;
    let mut result = 0;
    rec(&graph, start, stop, &mut seen, 0, &mut result);
    result
}

fn main() {
    let mut vec: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    println!("Part 1: {}", solve(&vec));
    remove_slopes(&mut vec);
    println!("Part 2: {}", solve(&vec));
}
