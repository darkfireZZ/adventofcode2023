use std::{
    fs::File,
    io::Read,
    iter::Iterator,
};

fn main() -> std::io::Result<()> {
    let input_file_path = parse_args();
    let mut input_file = File::open(input_file_path)?;
    
    let input = {
        let mut buffer = String::new();
        input_file.read_to_string(&mut buffer)?;
        buffer
    };

    print!("sum of calibration values: {}", sum_of_calibration_values(&input));

    Ok(())
}

fn sum_of_calibration_values(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut matches = line.matches(|c: char| c.is_digit(10));
        let first = matches.next().expect("every line contains at least 1 digit");
        let last = matches.last().unwrap_or(first);

        let first = u32::from_str_radix(&first, 10).expect("first is a digit");
        let last = u32::from_str_radix(&last, 10).expect("last is a digit");

        first * 10 + last
    }).sum()
}

fn parse_args() -> String {
    let args = std::env::args();

    if args.len() != 2 {
        panic!("expected 2 arguments, got {}", args.len());
    }

    let input_file_path = args.skip(1).next().expect("there are 2 arguments");

    input_file_path
}
