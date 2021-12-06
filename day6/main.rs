const LAST_DAY: usize = 257;

fn main() {
    let timers: Vec<_> = include_str!("input.txt")
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let mut spawns = vec![0; LAST_DAY];
    spawns[0] = timers.len();
    for mut timer in timers {
        while timer + 1 < LAST_DAY {
            spawns[timer + 1] += 1;
            timer += 7;
        }
    }
    for day in 1..LAST_DAY {
        let mut increment = 9;
        while day + increment < LAST_DAY {
            spawns[day + increment] += spawns[day];
            increment += 7;
        }
    }
    println!(
        "Part 1: {}, Part 2: {}",
        spawns.iter().take(81).sum::<usize>(),
        spawns.iter().take(257).sum::<usize>()
    );
}
