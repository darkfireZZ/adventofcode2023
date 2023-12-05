const INPUT: &str = include_str!("input");

fn main() {
    part1();
}

fn part1() {
    let parsed_input = parse_input();

    let seeds = parsed_input.seeds;
    let maps = parsed_input.maps;

    let mut current_numbers = seeds;
    for map in maps {
        current_numbers = current_numbers
            .into_iter()
            .map(|number| map.map_to_next(number))
            .collect();
    }

    let solution = current_numbers
        .into_iter()
        .min()
        .expect("iterator is not empty");

    println!("part1: {}", solution);
}

fn parse_input() -> ParsedInput {
    let (seeds_input, map_input) = INPUT.split_once("\n\n").unwrap();

    let seeds = seeds_input
        .split_whitespace()
        .skip(1)
        .map(|seed| u64::from_str_radix(seed, 10).expect("all seeds are numbers"))
        .collect();

    let maps = map_input
        .split("\n\n")
        .map(|map| {
            let (first_line, rem_lines) = map.split_once('\n').unwrap();

            let from_to: Vec<_> = first_line.split(['-', ' ']).collect();

            assert_eq!(from_to.len(), 4);

            let ranges = rem_lines
                .lines()
                .map(|line| {
                    let numbers = line
                        .split_whitespace()
                        .map(|number| u64::from_str_radix(number, 10).unwrap())
                        .collect::<Vec<_>>();

                    assert_eq!(numbers.len(), 3);

                    MapRange {
                        dest_start: numbers[0],
                        source_start: numbers[1],
                        length: numbers[2],
                    }
                })
                .collect();

            Map {
                from: from_to[0],
                to: from_to[2],
                ranges,
            }
        })
        .collect();

    ParsedInput { seeds, maps }
}

#[derive(Debug)]
struct ParsedInput {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    from: &'static str,
    to: &'static str,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map_to_next(&self, number: u64) -> u64 {
        for range in &self.ranges {
            let source_start = range.source_start;
            let source_end = range.source_start + range.length;
            if (source_start..source_end).contains(&number) {
                let index = number - source_start;
                return range.dest_start + index;
            }
        }

        return number;
    }
}

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    length: u64,
}
