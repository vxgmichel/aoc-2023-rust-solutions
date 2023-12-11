use std::io::{self, BufRead};

fn single_dimension(xs: &[u64], expansion: u64) -> u64 {
    let mut stars = 0;
    let mut count = 0;
    let mut result = 0;
    for &x in xs {
        count += (if x == 0 { expansion } else { 1 }) * stars;
        result += count * x;
        stars += x;
    }
    result
}

fn solve(xs: &[Vec<bool>], expansion: u64) -> u64 {
    let mut x_sums = vec![0; xs.len()];
    let mut y_sums = vec![0; xs[0].len()];
    for (i, row) in xs.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            x_sums[i] += value as u64;
            y_sums[j] += value as u64;
        }
    }
    single_dimension(&x_sums, expansion) + single_dimension(&y_sums, expansion)
}

fn main() {
    let vec: Vec<Vec<bool>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().map(|x| x == '#').collect())
        .collect();
    println!("Part 1: {}", solve(&vec, 2));
    println!("Part 2: {}", solve(&vec, 1000000));
}
