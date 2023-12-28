use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, BufRead},
};

type Point = (i32, i32, i32);
type Brick = (Point, Point);

type Edges = HashMap<Brick, HashSet<Brick>>;

fn solve(xs: &[Brick]) -> (Edges, Edges) {
    let mut current = HashMap::new();
    let mut supports = HashMap::new();
    let mut supported = HashMap::new();
    let base = ((0, 0, 0), (0, 0, 0));
    for brick in xs {
        let (p1, p2) = *brick;
        let h = (p1.0..=p2.0)
            .flat_map(|x| (p1.1..=p2.1).map(move |y| (x, y)))
            .map(|(x, y)| current.get(&(x, y)).unwrap_or(&base).1 .2)
            .max()
            .unwrap();
        let d = p2.2 - p1.2;
        let new_brick = ((p1.0, p1.1, h + 1), (p2.0, p2.1, h + 1 + d));
        for &support in (p1.0..=p2.0)
            .flat_map(|x| (p1.1..=p2.1).map(move |y| (x, y)))
            .map(|(x, y)| current.get(&(x, y)).unwrap_or(&base))
            .filter(|x| x.1 .2 == h)
        {
            supports
                .entry(support)
                .or_insert_with(HashSet::new)
                .insert(new_brick);
            supported
                .entry(new_brick)
                .or_insert_with(HashSet::new)
                .insert(support);
        }
        for x in p1.0..=p2.0 {
            for y in p1.1..=p2.1 {
                current.insert((x, y), new_brick);
            }
        }
    }
    (supports, supported)
}

fn parse_brick(s: &str) -> Brick {
    let (a, b) = s.split_once('~').unwrap();
    let mut p1 = a.split(',').map(|x| x.parse::<i32>().unwrap());
    let mut p2 = b.split(',').map(|x| x.parse::<i32>().unwrap());
    let p1 = (p1.next().unwrap(), p1.next().unwrap(), p1.next().unwrap());
    let p2 = (p2.next().unwrap(), p2.next().unwrap(), p2.next().unwrap());
    assert!(p1.0 <= p2.0 && p1.1 <= p2.1 && p1.2 <= p2.2);
    (p1, p2)
}

fn solve1(supported: &Edges) -> usize {
    supported.len()
        - supported
            .iter()
            .filter_map(|(_, b)| if b.len() == 1 { b.iter().next() } else { None })
            .filter(|&&x| x.0 .2 != 0)
            .collect::<HashSet<_>>()
            .len()
}

fn solve2(supports: &Edges, supported: &Edges) -> usize {
    let mut result = 0;
    for (brick, children) in supports {
        if brick.0 .2 == 0 {
            continue;
        }
        let mut seen = [*brick].iter().cloned().collect::<HashSet<_>>();
        let mut heap = children
            .iter()
            .map(|&x| (-x.1 .2, x))
            .collect::<BinaryHeap<_>>();
        while let Some((_, x)) = heap.pop() {
            if supported[&x].iter().all(|x| seen.contains(x)) {
                seen.insert(x);
                if let Some(support) = supports.get(&x) {
                    for item in support.iter().map(|&x| (-x.1 .2, x)) {
                        heap.push(item);
                    }
                }
            }
        }
        seen.remove(brick);
        result += seen.len();
    }
    result
}

fn main() {
    let mut vec: Vec<Brick> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_brick(&x.unwrap()))
        .collect();
    vec.sort_by_key(|x| x.0 .2);
    let (supports, supported) = solve(&vec);
    println!("Part 1: {}", solve1(&supported));
    println!("Part 2: {}", solve2(&supports, &supported));
}
