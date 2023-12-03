use std::{
    fmt::{self, Debug},
    str,
};

const INPUT: &[u8] = include_bytes!("input");

const ROW_LENGTH: usize = 140 + 1;

fn main() {
    let input_with_sentinels = {
        let empty_line = [b'.'; ROW_LENGTH];
        let mut buffer = Vec::with_capacity(INPUT.len() + 3 * ROW_LENGTH);
        buffer.extend_from_slice(&empty_line);
        buffer.extend_from_slice(&empty_line);
        buffer.extend_from_slice(INPUT);
        buffer.extend_from_slice(&empty_line);
        buffer
    };

    let part1_solution = get_sum_of_part_numbers(&input_with_sentinels);
    println!("part 1: {}", part1_solution);

    let part2_solution = get_sum_of_gear_ratios(&input_with_sentinels);
    println!("part 2: {}", part2_solution);
}

fn get_sum_of_gear_ratios(input_with_sentinels: &[u8]) -> u32 {
    let mut sum = 0;
    for index in input_with_sentinels
        .into_iter()
        .enumerate()
        .filter_map(|(index, value)| if *value == b'*' { Some(index) } else { None })
    {
        let adjacent_numbers = adjacent_numbers(index, input_with_sentinels);

        if adjacent_numbers.len() != 2 {
            continue;
        }

        let gear_ratio = adjacent_numbers[0].value() * adjacent_numbers[1].value();
        sum += gear_ratio;
    }

    sum
}

fn adjacent_numbers(gear_index: usize, input_with_sentinels: &[u8]) -> Vec<Number> {
    let left = gear_index - 1;
    let right = gear_index + 1;
    let left_above = gear_index - ROW_LENGTH - 1;
    let left_below = gear_index + ROW_LENGTH - 1;

    let candidates = [
        left_above,
        left_above + 1,
        left_above + 2,
        left,
        right,
        left_below,
        left_below + 1,
        left_below + 2,
    ];

    let mut adjacent_numbers: Vec<Number> = Vec::with_capacity(8);
    for number in candidates
        .into_iter()
        .filter_map(|candidate| expand_number_at(candidate, input_with_sentinels))
    {
        if let Some(prev_number) = adjacent_numbers.last() {
            if number.start_of_number != prev_number.start_of_number {
                adjacent_numbers.push(number);
            }
        } else {
            adjacent_numbers.push(number);
        }
    }

    adjacent_numbers
}

fn expand_number_at(index: usize, input_with_sentinels: &[u8]) -> Option<Number> {
    if input_with_sentinels[index].is_ascii_digit() {
        let start_of_number = 1 + input_with_sentinels[..index]
            .iter()
            .rposition(|c| !c.is_ascii_digit())
            .expect("impossible because of sentinels");
        let end_of_number = index
            + 1
            + input_with_sentinels[index + 1..]
                .iter()
                .position(|c| !c.is_ascii_digit())
                .expect("impossible because of sentinels");

        Some(Number {
            start_of_number,
            end_of_number,
            input_with_sentinels,
        })
    } else {
        None
    }
}

fn get_sum_of_part_numbers(input_with_sentinels: &[u8]) -> u32 {
    let mut index = ROW_LENGTH + 1;

    let mut sum = 0;
    while let Some(start_of_number) = input_with_sentinels[index..]
        .into_iter()
        .position(u8::is_ascii_digit)
    {
        let start_of_number = start_of_number + index;
        let end_of_number = start_of_number
            + input_with_sentinels[start_of_number..]
                .into_iter()
                .position(|c| !c.is_ascii_digit())
                .expect("impossible because of sentinels");

        let number = Number {
            start_of_number,
            end_of_number,
            input_with_sentinels,
        };

        if number.has_adjacent_symbol() {
            sum += number.value();
        }

        index = end_of_number;
    }

    sum
}

#[derive(Clone, Copy)]
struct Number<'a> {
    start_of_number: usize,
    end_of_number: usize,
    input_with_sentinels: &'a [u8],
}

impl Debug for Number<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Number")
            .field("start_of_number", &self.start_of_number)
            .field("end_of_number", &self.end_of_number)
            .finish()
    }
}

impl<'a> Number<'a> {
    fn has_adjacent_symbol(&self) -> bool {
        if is_symbol(self.input_with_sentinels[self.start_of_number - 1]) {
            return true;
        }

        if is_symbol(self.input_with_sentinels[self.end_of_number]) {
            return true;
        }

        let top_row_start = self.start_of_number - ROW_LENGTH - 1;
        let top_row_end = self.end_of_number - ROW_LENGTH + 1;
        if self.input_with_sentinels[top_row_start..top_row_end]
            .into_iter()
            .find(|c| is_symbol(**c))
            .is_some()
        {
            return true;
        }

        let bottom_row_start = self.start_of_number + ROW_LENGTH - 1;
        let bottom_row_end = self.end_of_number + ROW_LENGTH + 1;
        if self.input_with_sentinels[bottom_row_start..bottom_row_end]
            .into_iter()
            .find(|c| is_symbol(**c))
            .is_some()
        {
            return true;
        }

        return false;
    }

    fn value(&self) -> u32 {
        let number_str =
            str::from_utf8(&self.input_with_sentinels[self.start_of_number..self.end_of_number])
                .expect("ascii is always valid UTF-8");
        u32::from_str_radix(number_str, 10).expect("number is valid")
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() & (c != b'.') & (c != b'\n')
}
