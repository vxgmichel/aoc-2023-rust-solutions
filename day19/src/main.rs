use std::str::FromStr;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Rule {
    LessThan(char, u64, String),
    GreaterThan(char, u64, String),
    Jump(String),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(':') {
            Some((a, b)) => {
                let c = a.chars().next().ok_or(())?;
                let cmp = a.chars().nth(1).ok_or(())?;
                let value = a[2..].parse::<u64>().map_err(|_| ())?;
                match cmp {
                    '<' => Self::LessThan(c, value, b.to_string()),
                    '>' => Self::GreaterThan(c, value, b.to_string()),
                    _ => return Err(()),
                }
            }
            None => Self::Jump(s.to_string()),
        })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('{').ok_or(())?;
        let name = a.to_string();
        let rules = b[0..b.len() - 1]
            .split(',')
            .map(Rule::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { name, rules })
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    fn set_rating(&mut self, char: char, value: u64) {
        match char {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => panic!(),
        }
    }

    fn get_rating(&self, char: char) -> u64 {
        match char {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }

    fn count(&self, other: &Self) -> u64 {
        assert!(self.a < other.a);
        assert!(self.m < other.m);
        assert!(self.x < other.x);
        assert!(self.s < other.s);
        (other.a - self.a + 1)
            * (other.m - self.m + 1)
            * (other.x - self.x + 1)
            * (other.s - self.s + 1)
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;
        for part in string[1..string.len() - 1].split(',') {
            let (key, value) = part.split_once('=').ok_or(())?;
            let value = value.parse::<u64>().map_err(|_| ())?;
            match key {
                "x" => x = Some(value),
                "m" => m = Some(value),
                "a" => a = Some(value),
                "s" => s = Some(value),
                _ => return Err(()),
            }
        }
        let x = x.ok_or(())?;
        let m = m.ok_or(())?;
        let a = a.ok_or(())?;
        let s = s.ok_or(())?;
        Ok(Self { x, m, a, s })
    }
}

fn evaluate(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut current = "in";
    while current != "A" && current != "R" {
        let workflow = workflows.get(current).unwrap();
        for rule in &workflow.rules {
            match rule {
                Rule::LessThan(c, value, next) => {
                    if part.get_rating(*c) < *value {
                        current = next;
                        break;
                    }
                }
                Rule::GreaterThan(c, value, next) => {
                    if part.get_rating(*c) > *value {
                        current = next;
                        break;
                    }
                }
                Rule::Jump(next) => {
                    current = next;
                    break;
                }
            }
        }
    }
    current == "A"
}

fn solve1(workflows: &HashMap<String, Workflow>, ratings: &[Part]) -> u64 {
    ratings
        .iter()
        .filter(|x| evaluate(workflows, x))
        .map(|x| x.value())
        .sum()
}

fn rec(workflows: &HashMap<String, Workflow>, name: &str, min_part: Part, max_part: Part) -> u64 {
    if name == "R" {
        return 0;
    }
    if name == "A" {
        return min_part.count(&max_part);
    }
    let mut min_part = min_part;
    let mut max_part = max_part;
    let workflow = workflows.get(name).unwrap();
    let mut result = 0;
    for rule in &workflow.rules {
        match rule {
            Rule::LessThan(c, value, next) => {
                let min_value = min_part.get_rating(*c);
                if min_value < *value {
                    let mut new_max_part = max_part;
                    let max_value = max_part.get_rating(*c);
                    new_max_part.set_rating(*c, max_value.min(value - 1));
                    result += rec(workflows, next, min_part, new_max_part);
                }
                min_part.set_rating(*c, min_value.max(*value));
            }
            Rule::GreaterThan(c, value, next) => {
                let max_value = max_part.get_rating(*c);
                if max_value > *value {
                    let mut new_min_part = min_part;
                    let min_value = min_part.get_rating(*c);
                    new_min_part.set_rating(*c, min_value.max(value + 1));
                    result += rec(workflows, next, new_min_part, max_part);
                }
                max_part.set_rating(*c, max_value.min(*value));
            }
            Rule::Jump(next) => {
                result += rec(workflows, next, min_part, max_part);
            }
        }
    }
    result
}

fn solve2(workflows: &HashMap<String, Workflow>) -> u64 {
    let mini = 1;
    let maxi = 4000;
    let min_part = Part {
        x: mini,
        m: mini,
        a: mini,
        s: mini,
    };
    let max_part = Part {
        x: maxi,
        m: maxi,
        a: maxi,
        s: maxi,
    };
    rec(workflows, "in", min_part, max_part)
}

fn main() {
    let workflows: HashMap<String, Workflow> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let w = Workflow::from_str(&x).unwrap();
            (w.name.clone(), w)
        })
        .collect();
    let ratings: Vec<Part> = io::stdin()
        .lock()
        .lines()
        .map(|x| Part::from_str(&x.unwrap()).unwrap())
        .collect();
    println!("Part 1: {}", solve1(&workflows, &ratings));
    println!("Part 2: {}", solve2(&workflows));
}
