use aoclib::Grid;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 405;

const INPUT: &str = include_str!("../input");

fn main() {
    let part1_example = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1_example, EXAMPLE_OUTPUT_1);

    let part1_solution = part1(INPUT);
    println!("part 1: {}", part1_solution);
}

fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|grid| {
            let grid = Grid::parse(grid.as_bytes());

            (100 * num_rows_above_mirror(&grid) + num_cols_left_of_mirror(&grid)) as u64
        })
        .sum()
}

fn num_cols_left_of_mirror(grid: &Grid<u8>) -> usize {
    let cols: Vec<_> = grid.cols().collect();

    let mut counter = 0;
    'outer: for mirror_col in 0..(cols.len() - 1) {
        let num_cols_left = mirror_col + 1;
        let num_cols_right = cols.len() - num_cols_left;
        let num_cols_mirrored = std::cmp::min(num_cols_left, num_cols_right);
        for col in (mirror_col + 1 - num_cols_mirrored)..=mirror_col {
            let mirrored_col = 2 * mirror_col - col + 1;
            if !cols[col].clone().into_iter().eq(cols[mirrored_col].clone()) {
                continue 'outer;
            }
        }

        counter += num_cols_left;
    }

    counter
}

fn num_rows_above_mirror(grid: &Grid<u8>) -> usize {
    let rows: Vec<_> = grid.rows().collect();

    let mut counter = 0;
    'outer: for mirror_row in 0..(rows.len() - 1) {
        let num_rows_above = mirror_row + 1;
        let num_rows_below = rows.len() - num_rows_above;
        let num_rows_mirrored = std::cmp::min(num_rows_above, num_rows_below);
        for row in (mirror_row + 1 - num_rows_mirrored)..=mirror_row {
            let mirrored_row = 2 * mirror_row - row + 1;
            if !rows[row].clone().into_iter().eq(rows[mirrored_row].clone()) {
                continue 'outer;
            }
        }

        counter += num_rows_above;
    }

    counter
}
