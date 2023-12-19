use aoclib::Grid;

const EXAMPLE_INPUT_1: &str = include_str!("../example_input_1");
const EXAMPLE_OUTPUT_1: u64 = 405;

const EXAMPLE_OUTPUT_2: u64 = 400;

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
        .split("\n\n")
        .map(|grid| {
            let grid = Grid::parse(grid.as_bytes());

            let rows_above_mirror = rows_mirror_axes(&grid).next();
            let cols_left_of_mirror = || cols_mirror_axes(&grid).next();

            rows_above_mirror
                .map(|val| 100 * val)
                .or_else(cols_left_of_mirror)
                .unwrap() as u64
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|grid| {
            let mut grid = Grid::parse(grid.as_bytes());

            let cols_unchanged = cols_mirror_axes(&grid).next();
            let rows_unchanged = rows_mirror_axes(&grid).next();

            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let old_val = *grid.get(x, y);
                    let new_val = match old_val {
                        b'#' => b'.',
                        b'.' => b'#',
                        _ => unreachable!(),
                    };

                    *grid.get_mut(x, y) = new_val;

                    let num_cols_left =
                        cols_mirror_axes(&grid).find(|col| Some(*col) != cols_unchanged);
                    let num_rows_above =
                        rows_mirror_axes(&grid).find(|row| Some(*row) != rows_unchanged);

                    *grid.get_mut(x, y) = old_val;

                    if let Some(num_cols_left) = num_cols_left {
                        return num_cols_left as u64;
                    }

                    if let Some(num_rows_above) = num_rows_above {
                        return 100 * num_rows_above as u64;
                    }
                }
            }

            unreachable!();
        })
        .sum()
}

fn cols_mirror_axes(grid: &Grid<u8>) -> impl Iterator<Item = usize> + '_ {
    let cols: Vec<_> = grid.cols().collect();

    (0..(cols.len() - 1)).filter_map(move |mirror_col| {
        let num_cols_left = mirror_col + 1;
        let num_cols_right = cols.len() - num_cols_left;
        let num_cols_mirrored = std::cmp::min(num_cols_left, num_cols_right);
        for col in (mirror_col + 1 - num_cols_mirrored)..=mirror_col {
            let mirrored_col = 2 * mirror_col - col + 1;
            if !cols[col].clone().into_iter().eq(cols[mirrored_col].clone()) {
                return None;
            }
        }

        Some(mirror_col + 1)
    })
}

fn rows_mirror_axes(grid: &Grid<u8>) -> impl Iterator<Item = usize> + '_ {
    let rows: Vec<_> = grid.rows().collect();

    (0..(rows.len() - 1)).filter_map(move |mirror_row| {
        let num_rows_above = mirror_row + 1;
        let num_rows_below = rows.len() - num_rows_above;
        let num_rows_mirrored = std::cmp::min(num_rows_above, num_rows_below);
        for row in (mirror_row + 1 - num_rows_mirrored)..=mirror_row {
            let mirrored_row = 2 * mirror_row - row + 1;
            if !rows[row].clone().into_iter().eq(rows[mirrored_row].clone()) {
                return None;
            }
        }

        Some(mirror_row + 1)
    })
}
