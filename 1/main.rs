use std::{fs::File, io::Read, iter::Iterator};

fn main() -> std::io::Result<()> {
    let input_file_path = parse_args();
    let mut input_file = File::open(input_file_path)?;

    let input = {
        let mut buffer = String::new();
        input_file.read_to_string(&mut buffer)?;
        buffer
    };

    let part1_sum = sum_of_calibration_values_part1(&input);
    let part2_sum = sum_of_calibration_values_part2(&input);
    println!(
        "\
        sum of calibration values:\n\
        part 1: {}\n\
        part 2: {}",
        part1_sum, part2_sum
    );

    Ok(())
}

fn sum_of_calibration_values_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut matches = line.matches(|c: char| c.is_digit(10));
            let first = matches
                .next()
                .expect("every line contains at least 1 digit");
            let last = matches.last().unwrap_or(first);

            let first = u32::from_str_radix(&first, 10).expect("first is a digit");
            let last = u32::from_str_radix(&last, 10).expect("last is a digit");

            first * 10 + last
        })
        .sum()
}

fn sum_of_calibration_values_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = first_digit_part_2(&line);
            let last = last_digit_part_2(&line);

            first * 10 + last
        })
        .sum()
}

fn first_digit_part_2(line: &str) -> u32 {
    let mut first_index = line
        .find(|c: char| c.is_digit(10))
        .expect("every line contains at least 1 digit");
    let mut first_digit =
        u32::from_str_radix(&line[first_index..first_index + 1], 10).expect("is a digit");
    for digit in 1..10u32 {
        if let Some(index) = line.find(DIGIT_NAMES[digit as usize]) {
            if index < first_index {
                first_digit = digit;
                first_index = index;
            }
        }
    }

    first_digit
}

fn last_digit_part_2(line: &str) -> u32 {
    let mut last_index = line
        .rfind(|c: char| c.is_digit(10))
        .expect("every line contains at least 1 digit");
    let mut last_digit =
        u32::from_str_radix(&line[last_index..last_index + 1], 10).expect("is a digit");
    for digit in 1..10u32 {
        if let Some(index) = line.rfind(DIGIT_NAMES[digit as usize]) {
            if index > last_index {
                last_digit = digit;
                last_index = index;
            }
        }
    }

    last_digit
}

const DIGIT_NAMES: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_args() -> String {
    let args = std::env::args();

    if args.len() != 2 {
        panic!("expected 2 arguments, got {}", args.len());
    }

    let input_file_path = args.skip(1).next().expect("there are 2 arguments");

    input_file_path
}
