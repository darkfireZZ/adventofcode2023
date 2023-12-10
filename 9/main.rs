const EXAMPLE_INPUT: &str = include_str!("example_input");
const EXAMPLE_SOLUTION: i64 = 114;
const INPUT: &str = include_str!("input");

fn main() {
    assert_eq!(part1(EXAMPLE_INPUT), EXAMPLE_SOLUTION);

    let p1_sol = part1(INPUT);
    println!("part 1: {}", p1_sol);
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let line = line
                .split_whitespace()
                .map(|number| i64::from_str_radix(number, 10).unwrap())
                .collect();
            predict_value(line)
        })
        .sum()
}

fn predict_value(mut line: Vec<i64>) -> i64 {
    let num_values = line.len();

    let mut count = num_values;
    let mut sum = 0;
    while line.iter().take(count).any(|value| *value != 0) {
        count -= 1;
        for index in 0..count {
            line[index] = line[index + 1] - line[index];
        }
        sum += line[count];
    }

    sum
}
