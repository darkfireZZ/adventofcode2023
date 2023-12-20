const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 1320;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);
}

fn part1(input: &str) -> u64 {
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|step| calculate_hash(step.as_bytes()) as u64)
        .sum()
}

fn calculate_hash(string: &[u8]) -> u8 {
    let mut hash = 0;
    for byte in string {
        hash = update_hash(hash, *byte);
    }

    hash
}

fn update_hash(hash: u8, new_value: u8) -> u8 {
    let mut tmp = u16::from(hash);
    tmp += u16::from(new_value);
    tmp *= 17;
    (tmp % 256) as u8
}
