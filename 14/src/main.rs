use {aoclib::Grid, std::collections::HashMap};

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 136;
const EXAMPLE_OUTPUT_2: u64 = 64;

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
    let mut grid = Grid::parse(input.as_bytes());
    move_north(&mut grid);
    calculate_total_load(&grid)
}

fn part2(input: &str) -> u64 {
    let mut history = HashMap::new();
    let mut grid = Grid::parse(input.as_bytes());

    let mut counter = 0;
    let cycle_len = loop {
        if let Some(prev_index) = history.insert(grid.clone(), counter) {
            break counter - prev_index;
        }
        cycle(&mut grid);
        counter += 1;
    };

    let rem_cycles = (1_000_000_000 - counter) % cycle_len;

    for _ in 0..rem_cycles {
        cycle(&mut grid);
    }

    calculate_total_load(&grid)
}

fn cycle(grid: &mut Grid<u8>) {
    move_north(grid);
    move_west(grid);
    move_south(grid);
    move_east(grid);
}

fn move_north(grid: &mut Grid<u8>) {
    for x in 0..grid.width() {
        let mut next_north_pos = None;
        for y in 0..grid.height() {
            match grid.get(x, y) {
                b'#' => {
                    next_north_pos = None;
                }
                b'.' => {
                    if next_north_pos.is_none() {
                        next_north_pos = Some(y);
                    }
                }
                b'O' => {
                    if let Some(next_north_pos) = next_north_pos.as_mut() {
                        *grid.get_mut(x, *next_north_pos) = b'O';
                        *grid.get_mut(x, y) = b'.';
                        *next_north_pos += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn move_south(grid: &mut Grid<u8>) {
    for x in 0..grid.width() {
        let mut next_south_pos = None;
        for y in (0..grid.height()).rev() {
            match grid.get(x, y) {
                b'#' => {
                    next_south_pos = None;
                }
                b'.' => {
                    if next_south_pos.is_none() {
                        next_south_pos = Some(y);
                    }
                }
                b'O' => {
                    if let Some(next_south_pos) = next_south_pos.as_mut() {
                        *grid.get_mut(x, *next_south_pos) = b'O';
                        *grid.get_mut(x, y) = b'.';
                        *next_south_pos -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn move_west(grid: &mut Grid<u8>) {
    for y in 0..grid.height() {
        let mut next_west_pos = None;
        for x in 0..grid.width() {
            match grid.get(x, y) {
                b'#' => {
                    next_west_pos = None;
                }
                b'.' => {
                    if next_west_pos.is_none() {
                        next_west_pos = Some(x);
                    }
                }
                b'O' => {
                    if let Some(next_west_pos) = next_west_pos.as_mut() {
                        *grid.get_mut(*next_west_pos, y) = b'O';
                        *grid.get_mut(x, y) = b'.';
                        *next_west_pos += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn move_east(grid: &mut Grid<u8>) {
    for y in 0..grid.height() {
        let mut next_west_pos = None;
        for x in (0..grid.width()).rev() {
            match grid.get(x, y) {
                b'#' => {
                    next_west_pos = None;
                }
                b'.' => {
                    if next_west_pos.is_none() {
                        next_west_pos = Some(x);
                    }
                }
                b'O' => {
                    if let Some(next_west_pos) = next_west_pos.as_mut() {
                        *grid.get_mut(*next_west_pos, y) = b'O';
                        *grid.get_mut(x, y) = b'.';
                        *next_west_pos -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn calculate_total_load(grid: &Grid<u8>) -> u64 {
    let height = grid.height() as u64;
    grid.rows()
        .enumerate()
        .map(|(y, row)| {
            (height - (y as u64)) * row.into_iter().filter(|val| **val == b'O').count() as u64
        })
        .sum()
}
