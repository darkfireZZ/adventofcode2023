use std::str::FromStr;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 21;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (incomplete_arrangement, groups) = line.split_once(' ').unwrap();
            let all_arrangements = gen_all_arrangements(incomplete_arrangement.as_bytes());

            let groups: Vec<usize> = groups
                .split(',')
                .map(|size| usize::from_str(size).unwrap())
                .collect();

            all_arrangements
                .into_iter()
                .filter(|arr| arrangement_matches_groups(arr, &groups))
                .count() as u64
        })
        .sum()
}

fn gen_all_arrangements(incomplete_arrangement: &[u8]) -> impl Iterator<Item = Vec<u8>> + '_ {
    let unknown_positions: Vec<usize> = incomplete_arrangement
        .iter()
        .enumerate()
        .filter_map(|(index, val)| (*val == b'?').then_some(index))
        .collect();
    let num_unknowns = unknown_positions.len();
    let num_arrangements = 1 << num_unknowns;

    (0..num_arrangements).map(move |arr_id| {
        let mut arrangement = incomplete_arrangement.to_owned();
        for (unknown, unknown_position) in unknown_positions.iter().enumerate() {
            let replacement = if (arr_id >> unknown) & 1 == 0 {
                b'.'
            } else {
                b'#'
            };
            arrangement[*unknown_position] = replacement;
        }
        arrangement
    })
}

fn arrangement_matches_groups(arrangement: &[u8], groups: &[usize]) -> bool {
    arrangement
        .split(|c| *c == b'.')
        .filter(|s| !s.is_empty())
        .map(|group| group.len())
        .eq(groups.iter().copied())
}
