use std::str;

const INPUT: &[u8] = include_bytes!("input");

const ROW_LENGTH: usize = 140 + 1;

fn main() {
    let input_with_sentinels = {
        let empty_line = [b'.'; 141];
        let mut buffer = Vec::with_capacity(INPUT.len() + 2 * ROW_LENGTH);
        buffer.extend_from_slice(&empty_line);
        buffer.extend_from_slice(INPUT);
        buffer.extend_from_slice(&empty_line);
        buffer
    };

    let sum = get_sum_of_part_numbers(&input_with_sentinels);

    println!("sum of part numbers: {}", sum);
}

fn get_sum_of_part_numbers(input: &[u8]) -> u32 {
    let mut index = ROW_LENGTH + 1;

    let mut sum = 0;
    while let Some(start_of_number) = input[index..].iter().position(u8::is_ascii_digit) {
        let start_of_number = start_of_number + index;
        let end_of_number = start_of_number
            + input[start_of_number..]
                .iter()
                .position(|c| !c.is_ascii_digit())
                .expect("because of the sentinel values, there will never be a number at the end");

        let number = Number {
            start_of_number,
            end_of_number,
            input,
        };

        if number.has_adjacent_symbol() {
            sum += number.value().expect("is a valid number")
        }

        index = end_of_number;
    }

    sum
}

struct Number<'a> {
    start_of_number: usize,
    end_of_number: usize,
    input: &'a [u8],
}

impl<'a> Number<'a> {
    fn has_adjacent_symbol(&self) -> bool {
        if is_symbol(self.input[self.start_of_number - 1]) {
            return true;
        }

        if is_symbol(self.input[self.end_of_number]) {
            return true;
        }

        let top_row_start = self.start_of_number - ROW_LENGTH - 1;
        let top_row_end = self.end_of_number - ROW_LENGTH + 1;
        if self.input[top_row_start..top_row_end]
            .iter()
            .find(|c| is_symbol(**c))
            .is_some()
        {
            return true;
        }

        let bottom_row_start = self.start_of_number + ROW_LENGTH - 1;
        let bottom_row_end = self.end_of_number + ROW_LENGTH + 1;
        if self.input[bottom_row_start..bottom_row_end]
            .iter()
            .find(|c| is_symbol(**c))
            .is_some()
        {
            return true;
        }

        return false;
    }

    fn value(&self) -> Option<u32> {
        let number_str =
            str::from_utf8(&self.input[self.start_of_number..self.end_of_number]).ok()?;
        u32::from_str_radix(number_str, 10).ok()
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() & (c != b'.') & (c != b'\n')
}
