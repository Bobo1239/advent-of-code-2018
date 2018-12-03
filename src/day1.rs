#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |a, b| a + b)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut seen = vec![0];
    let mut acc = 0;
    for i in input.iter().cycle() {
        acc += i;
        if seen.contains(&acc) {
            return acc;
        } else {
            seen.push(acc);
        }
    }
    unreachable!();
}