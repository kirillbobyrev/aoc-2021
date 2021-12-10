use std::collections;

fn main() {
    let score = collections::HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    let match_closing =
        collections::HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut part_one = 0;
    let mut scores = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            let mut stack = vec![];
            for ch in x.chars() {
                match ch {
                    '(' | '[' | '{' | '<' => stack.push(ch),
                    ')' | ']' | '}' | '>' => match stack.last() {
                        Some(opening) => {
                            if *opening == match_closing[&ch] {
                                stack.pop();
                            } else {
                                part_one += score[&ch];
                                return 0;
                            }
                        }
                        None => return 0,
                    },
                    _ => unreachable!(),
                }
            }
            stack
                .iter()
                .rev()
                .fold(0 as i64, |acc, x| acc * 5 + score[&x])
        })
        .filter(|x| *x > 0)
        .collect::<Vec<_>>();
    scores.sort();
    assert!(scores.len() % 2 == 1);
    println!("Part 1: {}, Part 2: {}", part_one, scores[scores.len() / 2]);
}
