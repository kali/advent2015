extern crate itertools;

use itertools::Itertools;

fn main() {
    let mut values: Vec<usize> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    values.sort();
    println!("{:?}", values);
    let max_possible = (0..)
        .find(|&v| values.iter().take(v).sum::<usize>() > 150)
        .unwrap();
    let count = (0..max_possible)
        .map(|i| {
            (i, values
                .iter()
                .combinations(i)
                .filter(|c| c.iter().cloned().sum::<usize>() == 150)
                .count())
        })
        .inspect(|t| println!("{:?}", t))
        .map(|t| t.1)
        .sum::<usize>();
    println!("{}", count);
}
