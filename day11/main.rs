const SIZE: usize = 10;
const RANGE: std::ops::Range<usize> = 0..SIZE;
const ITERATIONS: usize = 1000;
const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (0, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
    (1, 0),
    (0, 1),
];

fn main() {
    let mut octopuses: Vec<Vec<_>> = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            x.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut flashes = 0;
    for iteration in 0..ITERATIONS {
        let mut flashing = vec![];
        let mut flashed = [[false; SIZE]; SIZE];
        for x in RANGE {
            for y in RANGE {
                octopuses[x][y] += 1;
                if octopuses[x][y] > 9 {
                    flashing.push((x, y));
                    flashed[x][y] = true;
                }
            }
        }
        while !flashing.is_empty() {
            let (x, y) = flashing.last().unwrap();
            let (x, y) = (*x, *y);
            flashing.pop();
            if iteration < 100 {
                flashes += 1;
            }
            for (dx, dy) in DELTAS {
                let xx = x as i32 + dx;
                let yy = y as i32 + dy;
                if 0 <= xx && xx < SIZE as i32 && 0 <= yy && yy < SIZE as i32 {
                    let (xx, yy) = (xx as usize, yy as usize);
                    octopuses[xx][yy] += 1;
                    if octopuses[xx][yy] > 9 && !flashed[xx][yy] {
                        flashing.push((xx, yy));
                        flashed[xx][yy] = true;
                    }
                }
            }
        }
        for x in RANGE {
            for y in RANGE {
                if flashed[x][y] {
                    octopuses[x][y] = 0;
                }
            }
        }
        if flashed.iter().flatten().fold(true, |acc, x| acc && *x) {
            println!(
                "Part 2: Octopuses flashed simultaneously after {} steps",
                iteration + 1
            );
            break;
        }
    }
    println!("Part 1: Flashes after 100 iterations: {}", flashes);
}
