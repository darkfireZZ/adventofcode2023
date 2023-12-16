
use aoclib::Grid;

const EXAMPLE_INPUT_1: &'static str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 374;

const INPUT: &'static str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);
}

fn part1(input: &str) -> u64 {
    let grid = Grid::parse(input.as_bytes());

    let expanded_rows = grid.rows().enumerate().filter_map(|(index, row)| {
        row.into_iter().all(|value| *value == b'.').then_some(index)
    });
    let expanded_cols = grid.cols().enumerate().filter_map(|(index, col)| {
        col.into_iter().all(|value| *value == b'.').then_some(index)
    });

    let row_expansion = cumulate(expanded_rows, grid.height());
    let col_expansion = cumulate(expanded_cols, grid.width());

    let galaxies: Vec<_> = grid.map_indexed(|value, x, y| {
        if *value == b'#' {
            Some((x, y))
        } else {
            None
        }
    }).rows().into_iter().flatten().flatten().map(|(x, y)| {
        (x + col_expansion[*x], y + row_expansion[*y])
    }).collect();
    
    let mut sum = 0;
    for index_1 in 0..galaxies.len() {
        let galaxy_1 = galaxies[index_1];
        for galaxy_2 in &galaxies[index_1 + 1..] {
            sum += galaxy_1.0.abs_diff(galaxy_2.0) + galaxy_1.1.abs_diff(galaxy_2.1);
        }
    }

    sum as u64
}

fn cumulate(iter: impl Iterator<Item = usize>, length: usize) -> Vec<usize> {
    let mut cumulative = vec![0; length];

    for index in iter {
        for value in &mut cumulative[index..] {
            *value += 1;
        }
    }

    cumulative
}
