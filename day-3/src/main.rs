fn main() {
    const INPUT: &[u8; 19739] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let width = width(input);
    let mut sum = 0;
    let mut op = |num| sum += num;
    for (idx, c) in input.iter().enumerate() {
        if is_symbol(*c) {
            read_surrounding_numbers(input, idx, width, &mut op);
        }
    }
    sum
}

fn run_2(input: &[u8]) -> usize {
    let width = width(input);
    let mut sum = 0;
    for (idx, c) in input.iter().enumerate() {
        if *c == b'*' {
            let mut part_numbers = Vec::new();
            let op = |num| part_numbers.push(num);
            read_surrounding_numbers(input, idx, width, op);
            if part_numbers.len() == 2 {
                sum += part_numbers[0] * part_numbers[1];
            }
        }
    }
    sum
}

fn width(input: &[u8]) -> usize {
    input
        .iter()
        .position(|c| *c == b'\n')
        .map(|w| w + 1)
        .unwrap_or(input.len())
}

fn is_symbol(c: u8) -> bool {
    !matches!(c, b'0'..=b'9' | b'.' | b'\n')
}

/// Reads any/all numbers surrounding `idx` in `input`, including diagonally, and performs `op` on
/// each one.
///
/// * `width`: The width of each line of `input`, including the new line character.
fn read_surrounding_numbers<F>(input: &[u8], idx: usize, width: usize, mut op: F)
where
    F: FnMut(usize),
{
    // Check above.
    read_vertical(input, idx, width, &mut op, Vertical::Above);
    // Check below.
    read_vertical(input, idx, width, &mut op, Vertical::Below);
    // Check left.
    if let Some(left_idx) = idx.checked_sub(1) {
        read_number(input, left_idx, width, &mut op);
    }
    // Check right.
    if let Some(right_idx) = idx.checked_add(1) {
        read_number(input, right_idx, width, &mut op);
    }
}

/// Attempts to read any numbers above or below `idx` in `input`, including diagonally, and perform
/// `num_op` on them.
///
/// * `width`: The width of each line of `input`, including the new line character.
fn read_vertical<F>(input: &[u8], idx: usize, width: usize, mut num_op: F, pole: Vertical)
where
    F: FnMut(usize),
{
    let op = match pole {
        Vertical::Above => usize::checked_sub,
        Vertical::Below => usize::checked_add,
    };
    // Check directly vertical first.
    if let Some(direct_vertical_idx) = op(idx, width) {
        if !read_number(input, direct_vertical_idx, width, &mut num_op) {
            // There wasn't a number directly vertical, so check diagonals.
            if idx % width != 0 {
                if let Some(diag_idx) = op(idx, width - 1) {
                    read_number(input, diag_idx, width, &mut num_op);
                }
            }
            if idx % width != width - 1 {
                if let Some(diag_idx) = op(idx, width + 1) {
                    read_number(input, diag_idx, width, &mut num_op);
                }
            }
        }
    }
}

enum Vertical {
    Above,
    Below,
}

/// Checks if there's a digit at `idx` in `index`, and, if so, reads the entire number and performs
/// `op` on the result.
///
/// * `width`: The width of each line of `input`, including the new line character.
fn read_number<F>(input: &[u8], idx: usize, width: usize, mut op: F) -> bool
where
    F: FnMut(usize),
{
    if idx >= input.len() || !input[idx].is_ascii_digit() {
        return false;
    }
    let mut first_idx = idx;
    while matches!(
        first_idx
            .checked_sub(1)
            .filter(|idx| idx % width != width - 1)
            .map(|idx| input[idx].is_ascii_digit()),
        Some(true)
    ) {
        first_idx -= 1;
    }
    let mut last_idx = idx;
    while matches!(
        last_idx
            .checked_add(1)
            .filter(|idx| idx % width != 0)
            .map(|idx| input[idx].is_ascii_digit()),
        Some(true)
    ) {
        last_idx += 1;
    }
    op(std::str::from_utf8(&input[first_idx..=last_idx])
        .unwrap()
        .parse::<usize>()
        .unwrap());
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 109] = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 4361);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 467835);
    }
}
