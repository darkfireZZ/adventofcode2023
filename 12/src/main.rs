use std::str::FromStr;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 21;
const EXAMPLE_OUTPUT_2: u64 = 525152;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);

    let part2_example = part2(EXAMPLE_INPUT_1);
    assert_eq!(part2_example, EXAMPLE_OUTPUT_2);

    let part2_solution = part2(INPUT);
    println!("part 2: {}", part2_solution);
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (mut arrangement, groups) = parse_arrangement_and_groups(line);
            let unknown_positions = unknown_positions(&arrangement);
            backtrack_combinations(&mut arrangement, &groups, &unknown_positions)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut counter = 0;
    input
        .lines()
        .map(|line| {
            dbg!(counter);
            counter += 1;
            let (arrangement, groups) = parse_arrangement_and_groups(line);

            let mut arrangement: Vec<u8> =
                std::iter::repeat(std::iter::once(b'?').chain(arrangement))
                    .take(5)
                    .flatten()
                    .skip(1)
                    .collect();
            let groups: Vec<usize> = std::iter::repeat(groups).take(5).flatten().collect();

            let unknown_positions = unknown_positions(&arrangement);

            backtrack_combinations(&mut arrangement, &groups, &unknown_positions)
        })
        .sum()
}

fn parse_arrangement_and_groups(input_line: &str) -> (Vec<u8>, Vec<usize>) {
    let (arrangement, groups) = input_line.split_once(' ').unwrap();

    let arrangement = arrangement.as_bytes().to_owned();

    let groups: Vec<usize> = groups
        .split(',')
        .map(|size| usize::from_str(size).unwrap())
        .collect();

    (arrangement, groups)
}

fn unknown_positions(arrangement: &[u8]) -> Vec<usize> {
    arrangement
        .iter()
        .enumerate()
        .filter_map(|(index, val)| (*val == b'?').then_some(index))
        .collect()
}

fn backtrack_combinations(
    arrangement: &mut [u8],
    groups: &[usize],
    unknown_positions: &[usize],
) -> u64 {
    backtrack_combinations_rec(arrangement, groups, unknown_positions, 0, 0)
}

fn backtrack_combinations_rec(
    arrangement: &mut [u8],
    groups: &[usize],
    unknown_positions: &[usize],
    correct_upto: usize,
    depth: usize,
) -> u64 {
    if depth == unknown_positions.len() {
        return if arrangement_matches_groups(&arrangement[correct_upto..], groups) {
            1
        } else {
            0
        };
    }

    let current_pos = unknown_positions[depth];

    let (correct_upto, groups) =
        match start_matches(&arrangement[..current_pos], groups, correct_upto) {
            Some((correct_upto, groups)) => (correct_upto, groups),
            None => {
                return 0;
            }
        };

    let mut count = 0;

    arrangement[current_pos] = b'#';
    count += backtrack_combinations_rec(
        arrangement,
        groups,
        unknown_positions,
        correct_upto,
        depth + 1,
    );
    arrangement[current_pos] = b'.';
    count += backtrack_combinations_rec(
        arrangement,
        groups,
        unknown_positions,
        correct_upto,
        depth + 1,
    );
    arrangement[current_pos] = b'?';

    count
}

fn arrangement_matches_groups(arrangement: &[u8], groups: &[usize]) -> bool {
    arrangement
        .split(|c| *c == b'.')
        .filter(|s| !s.is_empty())
        .map(|group| group.len())
        .eq(groups.iter().copied())
}

fn start_matches<'gr>(
    arrangement: &[u8],
    groups: &'gr [usize],
    mut correct_upto: usize,
) -> Option<(usize, &'gr [usize])> {
    let mut count = 0;
    let mut groups_index = 0;

    #[allow(clippy::mut_range_bound)]
    #[allow(clippy::needless_range_loop)]
    for arr_index in correct_upto..arrangement.len() {
        match arrangement[arr_index] {
            b'#' => {
                count += 1;
            }
            b'.' => {
                if count != 0 {
                    let group_size = *groups.get(groups_index)?;
                    if count != group_size {
                        return None;
                    } else {
                        groups_index += 1;
                        count = 0;
                    }
                }
                correct_upto = arr_index + 1;
            }
            _ => unreachable!(),
        }
    }

    Some((correct_upto, &groups[groups_index..]))
}
