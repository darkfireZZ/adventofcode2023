const INPUT: &str = include_str!("input");

fn main() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    let num_winning_numbers: Vec<usize> = INPUT
        .lines()
        .map(|line| {
            let integers: [usize; 36] = line
                .split_whitespace()
                .map(|entry| entry.trim_end_matches(':'))
                .filter_map(|number| usize::from_str_radix(number, 10).ok())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            let my_numbers = &integers[1..11];
            let winning_numbers = &integers[11..36];

            let mut count = 0;
            for winning_number in winning_numbers {
                if my_numbers.contains(winning_number) {
                    count += 1;
                }
            }

            count
        })
        .collect();

    assert_eq!(num_winning_numbers.len(), 209);

    let mut cards = [1; 209];

    let mut part2 = 0;
    for id in 0..209 {
        for incr_idx in (id + 1)..(id + num_winning_numbers[id] + 1) {
            cards[incr_idx] += cards[id];
        }
        part2 += cards[id];
    }

    println!("{}", part2);
}
