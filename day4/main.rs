use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

const SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<i32>>,
    taken: Vec<Vec<bool>>,
}

impl Board {
    fn put(&mut self, draw: i32) -> i32 {
        let mut sum = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.numbers[i][j] == draw {
                    self.taken[i][j] = true;
                }
                if !self.taken[i][j] {
                    sum += self.numbers[i][j];
                }
            }
        }
        let mut multiplier = 0;
        for i in 0..SIZE {
            if self.taken[i].iter().filter(|x| **x).count() == SIZE {
                multiplier = draw;
            }
        }
        for j in 0..SIZE {
            if self.taken.iter().map(|x| x[j]).filter(|x| *x).count() == SIZE {
                multiplier = draw;
            }
        }
        multiplier * sum
    }
}

fn scores() -> Vec<i32> {
    let mut lines = include_str!("input.txt").lines();
    let draws: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    dbg!(&draws);

    let mut numbers: Vec<Vec<i32>> = vec![];
    let mut boards = Vec::new();
    for line in lines {
        if !line.is_empty() {
            numbers.push(
                line.split(' ')
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect(),
            );
        }
        if numbers.len() == SIZE {
            boards.push(Board {
                numbers: numbers.clone(),
                taken: vec![vec![false; SIZE]; SIZE],
            });
            numbers.clear();
        }
    }

    let mut result = vec![];
    let mut next_boards = vec![];

    for draw in &draws {
        for board in &mut boards {
            let score = board.put(*draw);
            if score > 0 {
                result.push(score);
            } else {
                next_boards.push(board.clone());
            }
        }
        boards = next_boards.clone();
        next_boards.clear();
    }

    result
}

fn part_two() {}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    match opts.part {
        1 => println!("{}", scores().first().unwrap()),
        2 => println!("{}", scores().last().unwrap()),
        _ => unreachable!(),
    }
    Ok(())
}
