fn main() {
    const INPUT: &[u8; 19739] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let width = input
        .iter()
        .position(|c| *c == b'\n')
        .map(|w| w + 1)
        .unwrap_or(input.len());
    let mut sum = 0;
    for (idx, c) in input.iter().enumerate() {
        if is_symbol(*c) {
            // Check above.
            read_vertical(input, idx, width, &mut sum, Vertical::Above);
            // Check below.
            read_vertical(input, idx, width, &mut sum, Vertical::Below);
            // Check left.
            if let Some(left_idx) = idx.checked_sub(1) {
                read_number(input, left_idx, width, &mut sum);
            }
            // Check right.
            if let Some(right_idx) = idx.checked_add(1) {
                read_number(input, right_idx, width, &mut sum);
            }
        }
    }
    sum
}

fn is_symbol(c: u8) -> bool {
    !matches!(c, b'0'..=b'9' | b'.' | b'\n')
}

/// Attempts to read a number above or below `idx` in `input`, including diagonally, and add it to
/// `sum`.
///
/// * `width`: The width of each line of `input`, including the new line character.
fn read_vertical(input: &[u8], idx: usize, width: usize, sum: &mut usize, pole: Vertical) {
    let op = match pole {
        Vertical::Above => usize::checked_sub,
        Vertical::Below => usize::checked_add,
    };
    // Check directly vertical first.
    if let Some(direct_vertical_idx) = op(idx, width) {
        if !read_number(input, direct_vertical_idx, width, sum) {
            // There wasn't a number directly vertical, so check diagonals.
            if idx % width != 0 {
                if let Some(diag_idx) = op(idx, width - 1) {
                    read_number(input, diag_idx, width, sum);
                }
            }
            if idx % width != width - 1 {
                if let Some(diag_idx) = op(idx, width + 1) {
                    read_number(input, diag_idx, width, sum);
                }
            }
        }
    }
}

enum Vertical {
    Above,
    Below,
}

/// Checks if there's a digit at `idx` in `index`, and, if so, reads the entire number and adds it
/// to `sum`.
///
/// * `width`: The width of each line of `input`, including the new line character.
///
/// Returns true if a number was found at the provided idx, and false otherwise.
fn read_number(input: &[u8], idx: usize, width: usize, sum: &mut usize) -> bool {
    if idx >= input.len() || !is_digit(input[idx]) {
        return false;
    }
    let mut first_idx = idx;
    while matches!(
        first_idx
            .checked_sub(1)
            .filter(|idx| idx % width != width - 1)
            .map(|idx| is_digit(input[idx])),
        Some(true)
    ) {
        first_idx -= 1;
    }
    let mut last_idx = idx;
    while matches!(
        last_idx
            .checked_add(1)
            .filter(|idx| idx % width != 0)
            .map(|idx| is_digit(input[idx])),
        Some(true)
    ) {
        last_idx += 1;
    }
    *sum += std::str::from_utf8(&input[first_idx..=last_idx])
        .unwrap()
        .parse::<usize>()
        .unwrap();
    true
}

fn is_digit(c: u8) -> bool {
    matches!(c, b'0'..=b'9')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
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
        assert_eq!(run_1(INPUT), 4361);
    }
}
