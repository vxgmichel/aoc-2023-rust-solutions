use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;

enum ModuleType {
    Broadcast,
    Flip,
    Conj,
}

struct Module {
    name: String,
    module_type: ModuleType,
    destination: Vec<String>,
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").ok_or(())?;
        let destination = b.split(", ").map(|x| x.to_string()).collect();
        Ok(match &a[0..1] {
            "&" => Module {
                name: a[1..].to_string(),
                module_type: ModuleType::Conj,
                destination,
            },
            "%" => Module {
                name: a[1..].to_string(),
                module_type: ModuleType::Flip,
                destination,
            },
            _ => Module {
                name: a.to_string(),
                module_type: ModuleType::Broadcast,
                destination,
            },
        })
    }
}

fn init_memory(
    map: &HashMap<String, Module>,
) -> (
    HashMap<String, bool>,
    HashMap<String, HashMap<String, bool>>,
) {
    let mut flip = HashMap::new();
    let mut conj = HashMap::new();
    for (name, module) in map {
        match module.module_type {
            ModuleType::Broadcast => (),
            ModuleType::Conj => {
                conj.insert(name.clone(), HashMap::new());
            }
            ModuleType::Flip => {
                flip.insert(name.clone(), false);
            }
        }
    }
    for (name, module) in map {
        for d in &module.destination {
            if let Some(x) = conj.get_mut(d) {
                x.insert(name.clone(), false);
            }
        }
    }
    (flip, conj)
}

fn run_once(
    map: &HashMap<String, Module>,
    flip: &mut HashMap<String, bool>,
    conj: &mut HashMap<String, HashMap<String, bool>>,
    end: &str,
) -> (u64, u64, Vec<bool>) {
    let mut current_pulses = VecDeque::from([("button", "broadcaster", false)]);
    let mut count_low = 1;
    let mut count_high = 0;
    let mut end_pulses = vec![];
    while let Some((from_name, to_name, value)) = current_pulses.pop_front() {
        if to_name == end {
            end_pulses.push(value);
            continue;
        }
        let module = match map.get(to_name) {
            Some(x) => x,
            None => continue,
        };
        let value = match module.module_type {
            ModuleType::Broadcast => value,
            ModuleType::Flip => {
                if value {
                    continue;
                }
                let mem = flip.get_mut(to_name).unwrap();
                *mem ^= true;
                *mem
            }
            ModuleType::Conj => {
                let mem = conj.get_mut(to_name).unwrap();
                *mem.get_mut(from_name).unwrap() = value;
                !mem.values().all(|x| *x)
            }
        };
        current_pulses.extend(
            module
                .destination
                .iter()
                .map(|x| (to_name, x.as_str(), value)),
        );
        match value {
            false => count_low += module.destination.len(),
            true => count_high += module.destination.len(),
        }
    }
    (count_low as u64, count_high as u64, end_pulses)
}

fn solve1(map: &HashMap<String, Module>) -> u64 {
    let (mut flip, mut conj) = init_memory(map);
    let mut count_low = 0;
    let mut count_high = 0;
    for _ in 0..1000 {
        let (a, b, _) = run_once(map, &mut flip, &mut conj, "rx");
        count_low += a;
        count_high += b;
    }
    count_low * count_high
}

fn find_cycle(map: &HashMap<String, Module>, end: &str) -> u64 {
    let (mut flip, mut conj) = init_memory(map);
    let mut results = vec![];
    for i in 0.. {
        let (_, _, c) = run_once(map, &mut flip, &mut conj, end);
        if c.iter().any(|x| !*x) {
            results.push(i);
        }
        if results.len() == 2 {
            break;
        }
    }
    let cycle = results[1] - results[0];
    assert_eq!(results[0], cycle - 1);
    cycle
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

fn solve2(map: &HashMap<String, Module>) -> u64 {
    let mut result = 1;
    let parent = map
        .iter()
        .filter(|(_, v)| &v.destination[0] == "rx")
        .map(|(k, _)| k)
        .next()
        .unwrap();
    let parents = map
        .iter()
        .filter(|(_, v)| &v.destination[0] == parent)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    for end in parents {
        let cycle = find_cycle(map, end);
        result = (result * cycle) / gcd(result, cycle);
    }
    result
}

fn main() {
    let map: HashMap<String, Module> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let module = Module::from_str(&x.unwrap()).unwrap();
            (module.name.to_string(), module)
        })
        .collect();
    println!("Part 1: {}", solve1(&map));
    println!("Part 2: {}", solve2(&map));
}
