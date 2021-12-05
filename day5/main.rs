use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

const SIZE: usize = 1000;

fn main() {
    let opts = Opts::parse();
    let re = Regex::new(r"(^\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut board = [[0; SIZE]; SIZE];
    for line in include_str!("input.txt").lines() {
        let capture = re.captures(line).unwrap();
        let (mut x, mut y, x2, y2) = (
            capture[1].parse::<i32>().unwrap(),
            capture[2].parse::<i32>().unwrap(),
            capture[3].parse::<i32>().unwrap(),
            capture[4].parse::<i32>().unwrap(),
        );
        let (dx, dy) = (num::signum(x2 - x), num::signum(y2 - y));
        // Don't allow diagonal lines in part 1.
        if opts.part == 1 && dx != 0 && dy != 0 {
            continue;
        }
        board[x2 as usize][y2 as usize] += 1;
        while (x, y) != (x2, y2) {
            board[x as usize][y as usize] += 1;
            x += dx;
            y += dy;
        }
    }
    println!("{}", board.iter().flatten().filter(|x| **x > 1).count());
}
