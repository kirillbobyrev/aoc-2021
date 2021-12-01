use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    filename: String,
    #[clap(required(true))]
    part: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let file = File::open(opts.filename)?;
    let mut state = i32::MAX;
    let numbers: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    let result = match opts.part {
        1 => Ok(numbers
            .iter()
            .map(|x| {
                let val = if &state > x { 0 } else { 1 };
                state = *x;
                val
            })
            .sum::<i32>()),
        2 => Ok(numbers
            .windows(3)
            .map(|x| {
                let sum = &x.iter().sum::<i32>();
                let val = if &state >= sum { 0 } else { 1 };
                state = *sum;
                val
            })
            .sum::<i32>()),
        _ => Err("Unknown part: has to be either 1 or 2"),
    };
    println!("The result is {}", result?);
    Ok(())
}
