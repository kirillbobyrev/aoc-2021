use clap::Parser;
use std::cmp;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

fn main() {
    let opts = Opts::parse();
    let crabs: Vec<_> = include_str!("input.txt")
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut result = (i32::MAX, i32::MAX);
    for centroid in min..=max {
        let candidate = match opts.part {
            1 => (
                crabs.iter().map(|x| i32::abs(x - centroid)).sum::<i32>(),
                centroid,
            ),
            2 => (
                crabs
                    .iter()
                    .map(|x| {
                        let length = i32::abs(x - centroid);
                        (length * (length + 1)) / 2
                    })
                    .sum::<i32>(),
                centroid,
            ),
            _ => unreachable!(),
        };
        result = cmp::min(result, candidate);
    }
    println!("{}, {}", result.0, result.1);
}
