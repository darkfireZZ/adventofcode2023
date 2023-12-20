const EXAMPLE_INPUT: &str = include_str!("example_input");
const EXAMPLE_SOLUTION_1: i64 = 114;
const EXAMPLE_SOLUTION_2: i64 = 2;
const INPUT: &str = include_str!("input");

fn main() {
    assert_eq!(part1(EXAMPLE_INPUT), EXAMPLE_SOLUTION_1);

    let p1_sol = part1(INPUT);
    println!("part 1: {}", p1_sol);

    assert_eq!(part2(EXAMPLE_INPUT), EXAMPLE_SOLUTION_2);

    let p2_sol = part2(INPUT);
    println!("part 2: {}", p2_sol);
}

fn part1(input: &str) -> i64 {
    parse_lines(input)
        .map(|line| predict_value_right(line))
        .sum()
}

fn part2(input: &str) -> i64 {
    parse_lines(input)
        .map(|mut line| {
            line.reverse();
            predict_value_right(line)
        })
        .sum()
}

fn parse_lines(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|number| i64::from_str_radix(number, 10).unwrap())
            .collect()
    })
}

fn predict_value_right(mut line: Vec<i64>) -> i64 {
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
