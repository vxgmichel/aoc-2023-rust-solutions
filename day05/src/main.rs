use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    source: u64,
    destination: u64,
    range: u64,
}

impl FromStr for RangeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(());
        }

        let destination = parts[0].parse().map_err(|_| ())?;
        let source = parts[1].parse().map_err(|_| ())?;
        let range = parts[2].parse().map_err(|_| ())?;

        Ok(RangeMap {
            source,
            destination,
            range,
        })
    }
}

fn do_map(map: &[RangeMap], value: u64) -> u64 {
    let a = map.partition_point(|a| a.source + a.range <= value);
    if a == map.len() {
        return value;
    }
    let range = &map[a];
    if value < range.source {
        return value;
    }
    value + range.destination - range.source
}

fn map_range(map: &[RangeMap], value: u64, range: u64) -> Vec<(u64, u64)> {
    let i = map.partition_point(|a| a.source + a.range <= value);
    let j = map.partition_point(|a| a.source <= value + range);
    let mut current = value;
    let mut result = vec![];
    for r in &map[i..j] {
        if current < r.source {
            result.push((current, r.source - current));
        }
        let start = current.max(r.source);
        let stop = (r.source + r.range).min(value + range);
        result.push((r.destination + start - r.source, stop - start));
        current = stop
    }
    if current != value + range {
        result.push((current, value + range - current));
    }
    result
}

fn solve1(seeds: &[u64], maps: &[Vec<RangeMap>]) -> u64 {
    seeds
        .iter()
        .map(|&x| maps.iter().fold(x, |value, map| do_map(map, value)))
        .min()
        .unwrap()
}

fn solve2(seeds: &[u64], maps: &[Vec<RangeMap>]) -> u64 {
    let ranges: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| {
            if let [a, b] = chunk {
                (*a, *b)
            } else {
                unreachable!()
            }
        })
        .collect();
    let (result, _) = *maps
        .iter()
        .fold(ranges, |ranges, map| {
            ranges
                .iter()
                .flat_map(|&(x, y)| map_range(map, x, y))
                .collect()
        })
        .iter()
        .min()
        .unwrap();
    result
}

fn main() {
    let seeds: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(1)
        .map_while(|x| x.parse().ok())
        .collect();

    io::stdin().lock().lines().next();
    let mut maps = vec![];
    for _ in 0..7 {
        let mut map: Vec<RangeMap> = io::stdin()
            .lock()
            .lines()
            .skip(1)
            .map_while(|x| x.ok())
            .take_while(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        map.sort();
        maps.push(map);
    }
    println!("Part 1: {}", solve1(&seeds, &maps));
    println!("Part 2: {}", solve2(&seeds, &maps));
}
