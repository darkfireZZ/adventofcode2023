const INPUT: &str = include_str!("input");

fn main() {
    let part1: u32 = INPUT
        .lines()
        .map(|line| {
            let integers: [u32; 36] = line
                .split_whitespace()
                .map(|entry| entry.trim_end_matches(':'))
                .filter_map(|number| u32::from_str_radix(number, 10).ok())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();

            let my_numbers = &integers[1..11];
            let winning_numbers = &integers[11..36];

            let mut incr = 1;
            let mut value = 0;
            for winning_number in winning_numbers {
                if my_numbers.contains(winning_number) {
                    value += incr;
                    incr = value;
                }
            }

            value
        })
        .sum();

    println!("{}", part1);
}
