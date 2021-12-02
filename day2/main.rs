use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;
    for line in include_str!("input.txt").lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let (command, value) = (splits[0], splits[1]);
        let value = value.trim().parse::<i32>()?;
        match (opts.part, command) {
            (1, "forward") => horizontal_position += value,
            (1, "down") => vertical_position += value,
            (1, "up") => vertical_position -= value,
            (2, "forward") => {
                horizontal_position += value;
                vertical_position += value * aim;
            }
            (2, "down") => aim += value,
            (2, "up") => aim -= value,
            _ => panic!("invalid part"),
        }
    }
    println!(
        "{} * {} = {}",
        horizontal_position,
        vertical_position,
        horizontal_position * &vertical_position
    );
    Ok(())
}
