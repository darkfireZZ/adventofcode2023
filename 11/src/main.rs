use aoclib::Grid;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 374;
const EXAMPLE_OUTPUT_2: u64 = 8410;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = sum_of_distances(EXAMPLE_INPUT_1, 2);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = sum_of_distances(INPUT, 2);
    println!("part 1: {}", part1_solution);

    let part2_example = sum_of_distances(EXAMPLE_INPUT_1, 100);
    assert_eq!(part2_example, EXAMPLE_OUTPUT_2);

    let part2_solution = sum_of_distances(INPUT, 1_000_000);
    println!("part 2: {}", part2_solution);
}

fn sum_of_distances(input: &str, expansion_factor: usize) -> u64 {
    let grid = Grid::parse(input.as_bytes());

    let expanded_rows = grid
        .rows()
        .enumerate()
        .filter_map(|(index, row)| row.into_iter().all(|value| *value == b'.').then_some(index));
    let expanded_cols = grid
        .cols()
        .enumerate()
        .filter_map(|(index, col)| col.into_iter().all(|value| *value == b'.').then_some(index));

    let row_expansion = cumulate(expanded_rows, grid.height());
    let col_expansion = cumulate(expanded_cols, grid.width());

    let galaxies: Vec<_> = grid
        .map_indexed(
            |value, x, y| {
                if *value == b'#' {
                    Some((x, y))
                } else {
                    None
                }
            },
        )
        .rows()
        .flatten()
        .flatten()
        .map(|(x, y)| {
            (
                x + (expansion_factor - 1) * col_expansion[*x],
                y + (expansion_factor - 1) * row_expansion[*y],
            )
        })
        .collect();

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
