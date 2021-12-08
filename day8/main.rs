use itertools::Itertools;
use std::collections;

fn solve(cypher: &Vec<&str>) -> i32 {
    let inputs = cypher.iter().take(cypher.len() - 4).collect::<Vec<_>>();
    let outputs = cypher.iter().skip(cypher.len() - 4).collect::<Vec<_>>();
    let get_with_len = |len| -> Vec<_> { inputs.iter().filter(|x| x.len() == len).collect() };
    let one = get_with_len(2)[0];
    let seven = get_with_len(3)[0];
    let four = get_with_len(4)[0];
    let difference = |small: &str, large: &str| -> char {
        for ch in large.chars() {
            if !small.contains(ch) {
                return ch;
            }
        }
        unreachable!();
    };
    let mut mapping = collections::HashMap::new();
    mapping.insert(difference(one, seven), 'a');
    let two_three_five = get_with_len(5);
    assert_eq!(two_three_five.len(), 3);
    for ch in two_three_five[0].chars() {
        if !mapping.contains_key(&ch)
            && two_three_five.iter().filter(|x| x.contains(ch)).count() == two_three_five.len()
        {
            if four.contains(ch) {
                mapping.insert(ch, 'd');
            } else {
                mapping.insert(ch, 'g');
            }
        }
    }
    for ch in four.chars() {
        if !mapping.contains_key(&ch) && !one.contains(ch) {
            mapping.insert(ch, 'b');
        }
    }
    for digit in two_three_five {
        let mut count = 0;
        let mut missing_char = '0';
        for ch in digit.chars() {
            if mapping.contains_key(&ch) {
                count += 1;
            } else {
                missing_char = ch;
            }
        }
        if count == 4 {
            mapping.insert(missing_char, 'f');
            break;
        }
    }
    for ch in one.chars() {
        if !mapping.contains_key(&ch) {
            mapping.insert(ch, 'c');
            break;
        }
    }
    for ch in 'a'..='g' {
        if !mapping.contains_key(&ch) {
            mapping.insert(ch, 'e');
            break;
        }
    }
    let digit_mapping = collections::HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);
    let mut result = 0;
    for output in outputs {
        let digit = output
            .chars()
            .map(|ch| mapping[&ch])
            .sorted()
            .collect::<String>();
        result = result * 10 + &digit_mapping[&digit as &str];
    }
    result
}

fn main() {
    let inputs: Vec<_> = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            x.split('|')
                .flat_map(|x| x.trim().split(' '))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        inputs
            .iter()
            // Take the last 4 "words", i.e. the "outputs".
            .map(|x| x.iter().skip(x.len() - 4))
            .flatten()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count()
    );
    let mut result = 0;
    for cypher in &inputs {
        result += solve(&cypher);
    }
    println!("Part 2: {}", result);
}
