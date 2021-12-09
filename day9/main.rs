use itertools::Itertools;
use std::collections;

fn main() {
    let heights = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            x.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    let x_max = heights.len();
    let y_max = heights[0].len();
    let maybe = |x: i32, y: i32, dx: i32, dy: i32| {
        if 0 <= x + dx && x + dx < x_max as i32 && 0 <= y + dy && y + dy < y_max as i32 {
            Some(((x + dx) as usize, (y + dy) as usize))
        } else {
            None
        }
    };
    let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let lower_than = |x: i32, y: i32, dx: i32, dy: i32| match maybe(x, y, dx, dy) {
        Some((xx, yy)) => heights[x as usize][y as usize] < heights[xx][yy],
        None => true,
    };
    let low_points = (0..heights.len())
        .cartesian_product(0..heights[0].len())
        .filter(|(x, y)| {
            deltas.iter().fold(true, |acc, (dx, dy)| {
                acc && lower_than(*x as i32, *y as i32, *dx, *dy)
            })
        })
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        low_points
            .iter()
            .map(|(x, y)| heights[*x][*y] + 1)
            .sum::<i32>()
    );
    let mut visited = vec![vec![false; y_max as usize]; x_max as usize];
    let mut bfs = |x: usize, y: usize| {
        let mut result = 0;
        let mut queue = collections::VecDeque::from([(x, y)]);
        visited[x][y] = true;
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            result += 1;
            for (dx, dy) in deltas {
                if let Some((xx, yy)) = maybe(x as i32, y as i32, dx, dy) {
                    if !visited[xx][yy]
                        && heights[xx][yy] < 9
                        && lower_than(x as i32, y as i32, dx, dy)
                    {
                        queue.push_back((xx, yy));
                        visited[xx][yy] = true;
                    }
                }
            }
        }
        result
    };
    let mut basin_sizes = low_points
        .iter()
        .map(|(x, y)| bfs(*x, *y))
        .collect::<Vec<_>>();
    basin_sizes.sort();
    println!(
        "Part 2: {}",
        basin_sizes.iter().rev().take(3).product::<i32>()
    );
}
