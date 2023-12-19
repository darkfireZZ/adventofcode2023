
use aoclib::Grid;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 136;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);
}

fn part1(input: &str) -> u64 {
    let mut grid = Grid::parse(input.as_bytes());
    move_north(&mut grid);
    calculate_total_load(&grid)
}

fn move_north(grid: &mut Grid<u8>) {
    for x in 0..grid.width() {
        let mut next_north_pos = None;
        for y in 0..grid.height() {
            match grid.get(x, y) {
                b'#' => { next_north_pos = None; },
                b'.' => { if next_north_pos == None { next_north_pos = Some(y); } },
                b'O' => {
                    if let Some(next_north_pos) = next_north_pos.as_mut() {
                        *grid.get_mut(x, *next_north_pos) = b'O';
                        *grid.get_mut(x, y) = b'.';
                        *next_north_pos += 1;
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

fn calculate_total_load(grid: &Grid<u8>) -> u64 {
    let height = grid.height() as u64;
    grid.rows().into_iter().enumerate().map(|(y, row)| {
        (height - (y as u64)) * row.into_iter().filter(|val| **val == b'O').count() as u64
    }).sum()
}
