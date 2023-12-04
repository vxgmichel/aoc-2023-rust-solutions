use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in s.split(", ") {
            let (count, color) = set.split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            match color {
                "blue" => blue = count,
                "green" => green = count,
                "red" => red = count,
                _ => unreachable!(),
            }
        }
        Result::Ok(Self { red, green, blue })
    }
}

impl Set {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn max(&self, other: &Self) -> Self {
        let red = self.red.max(other.red);
        let green = self.green.max(other.green);
        let blue = self.blue.max(other.blue);
        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn solve1(xs: &[String]) -> u32 {
    let mut result = 0;
    for x in xs {
        let (game, values) = x.split_once(": ").unwrap();
        let (_, game_id) = game.split_once(' ').unwrap();
        let game_id = game_id.parse::<u32>().unwrap();
        if values
            .split("; ")
            .all(|x| x.parse::<Set>().unwrap().is_valid())
        {
            result += game_id;
        }
    }
    result
}

fn solve2(xs: &[String]) -> u32 {
    let mut result = 0;
    for x in xs {
        let (_, values) = x.split_once(": ").unwrap();
        let maxi = values
            .split("; ")
            .map(|x| x.parse::<Set>().unwrap())
            .reduce(|x, y| x.max(&y))
            .unwrap();
        result += maxi.power();
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
