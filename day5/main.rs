use clap::Parser;
use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

const SIZE: usize = 1000;

fn delta(from: i32, to: i32) -> i32 {
    match from.cmp(&to) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let input = include_str!("input.txt");
    let re = Regex::new(r"(^\d+),(\d+) -> (\d+),(\d+)$")?;
    let mut board = [[0; SIZE]; SIZE];
    for line in input.lines() {
        let capture = re.captures(line).unwrap();
        let (mut x, mut y, x2, y2) = (
            capture[1].parse::<i32>()?,
            capture[2].parse::<i32>()?,
            capture[3].parse::<i32>()?,
            capture[4].parse::<i32>()?,
        );
        let (dx, dy) = (delta(x, x2), delta(y, y2));
        // Don't allow diagonal lines in part 1.
        if opts.part == 1 && dx != 0 && dy != 0 {
            continue;
        }
        board[x2 as usize][y2 as usize] += 1;
        while x != x2 || y != y2 {
            board[x as usize][y as usize] += 1;
            x += dx;
            y += dy;
        }
    }
    println!("{}", board.iter().flatten().filter(|x| **x > 1).count());
    Ok(())
}
