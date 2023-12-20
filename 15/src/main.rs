const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 1320;
const EXAMPLE_OUTPUT_2: u64 = 145;

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
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|step| calculate_hash(step.as_bytes()) as u64)
        .sum()
}

fn part2(input: &str) -> u64 {
    let hash_map = gen_hash_map(input);
    focusing_power(&hash_map)
}

fn focusing_power(hash_map: &HashMap) -> u64 {
    let mut sum = 0;
    for (box_number, box_entry) in hash_map.iter().enumerate() {
        for (slot_number, (_, focal_length)) in box_entry.iter().enumerate() {
            sum += (1 + box_number as u64) * (1 + slot_number as u64) * u64::from(*focal_length);
        }
    }

    sum
}

type HashMap<'a> = [Vec<(&'a str, u8)>; 256];

fn gen_hash_map(input: &str) -> HashMap {
    let mut hash_map = std::array::from_fn(|_| Vec::new());

    for step in input.strip_suffix('\n').unwrap().split(',') {
        let step = parse_step(step);

        let hash = calculate_hash(step.label.as_bytes()) as usize;

        match step.op {
            Operation::Insert { focal_length } => {
                if let Some(existing_lens) = hash_map[hash]
                    .iter_mut()
                    .find(|(label, _)| *label == step.label)
                {
                    existing_lens.1 = focal_length;
                } else {
                    hash_map[hash].push((step.label, focal_length));
                }
            }
            Operation::Remove => {
                if let Some(index) = hash_map[hash]
                    .iter()
                    .position(|(label, _)| *label == step.label)
                {
                    hash_map[hash].remove(index);
                }
            }
        }
    }

    hash_map
}

struct Step<'a> {
    label: &'a str,
    op: Operation,
}

enum Operation {
    Insert { focal_length: u8 },
    Remove,
}

fn parse_step(step: &str) -> Step<'_> {
    if let Some(label) = step.strip_suffix('-') {
        Step {
            label,
            op: Operation::Remove,
        }
    } else {
        let (label, focal_length) = step.split_once('=').expect("contains a =");
        let focal_length = focal_length.parse().expect("is a number");

        Step {
            label,
            op: Operation::Insert { focal_length },
        }
    }
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
