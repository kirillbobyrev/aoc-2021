use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Opts {
    #[clap(required(true))]
    part: u32,
}

fn part_one() {
    let input = include_str!("input.txt");
    let line_length = input.lines().nth(0).unwrap().len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..line_length {
        let mut ones_cnt = 0;
        let mut zeros_cnt = 0;
        for line in input.lines() {
            match line.chars().nth_back(bit).unwrap() {
                '0' => zeros_cnt += 1,
                '1' => ones_cnt += 1,
                _ => unreachable!(),
            }
        }
        if ones_cnt > zeros_cnt {
            gamma |= 1 << bit;
        } else {
            epsilon |= 1 << bit;
        }
    }
    println!("{:b} * {:b} = {}", gamma, epsilon, gamma * epsilon);
}

fn filter(bit: usize, values: Vec<&str>, gammas: bool) -> Vec<&str> {
    let mut ones_cnt = 0;
    let mut zeros_cnt = 0;
    for line in &values {
        match line.chars().nth(bit).unwrap() {
            '0' => zeros_cnt += 1,
            '1' => ones_cnt += 1,
            _ => unreachable!(),
        }
    }
    let target_sym = match gammas {
        true => {
            if ones_cnt >= zeros_cnt {
                '1'
            } else {
                '0'
            }
        }
        false => {
            if ones_cnt < zeros_cnt {
                '1'
            } else {
                '0'
            }
        }
    };
    values
        .into_iter()
        .filter(|x| x.chars().nth(bit).unwrap() == target_sym)
        .collect()
}

fn part_two() {
    let input = include_str!("input.txt");
    let line_length = input.lines().nth(0).unwrap().len();
    let mut gammas: Vec<&str> = input.lines().collect();
    let mut epsilons: Vec<&str> = input.lines().collect();
    for bit in 0..line_length {
        if gammas.len() > 1 {
            gammas = filter(bit, gammas, true);
        }
        if epsilons.len() > 1 {
            epsilons = filter(bit, epsilons, false);
        }
    }
    let gamma = isize::from_str_radix(&gammas.first().unwrap(), 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilons.first().unwrap(), 2).unwrap();
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    match opts.part {
        1 => part_one(),
        2 => part_two(),
        _ => unreachable!(),
    }
    Ok(())
}
