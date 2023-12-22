fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| compute_pattern(&rows(pattern), None).unwrap())
        .sum()
}

fn run_2(input: &str) -> usize {
    input.split("\n\n").map(patterns).sum()
}

fn patterns(pattern: &str) -> usize {
    let mut rows = rows(pattern);
    let original = compute_pattern(&rows, None).unwrap();
    for row_idx in 0..rows.len() {
        for col_idx in 0..rows[row_idx].len() {
            flip_mirror(&mut rows, row_idx, col_idx);
            if let Some(num) = compute_pattern(&rows, Some(original)) {
                if num != original {
                    return num;
                }
            }
            flip_mirror(&mut rows, row_idx, col_idx);
        }
    }
    unreachable!()
}

fn compute_pattern(rows: &Vec<Vec<u8>>, ignore: Option<usize>) -> Option<usize> {
    if let Some(num) = find_reflection(
        rows,
        ignore.and_then(|n| if n % 100 == 0 { Some(n / 100) } else { None }),
    ) {
        return Some(num * 100);
    }
    let width = rows[0].len();
    let height = rows.len();
    let cols = (0..width)
        .map(|col_idx| {
            (0..height)
                .map(|row_idx| rows[row_idx][col_idx])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    find_reflection(
        &cols,
        ignore.and_then(|n| if n % 100 != 0 { Some(n) } else { None }),
    )
}

fn find_reflection(grid: &Vec<Vec<u8>>, ignore: Option<usize>) -> Option<usize> {
    'outer: for idx in 1..grid.len() {
        if Some(idx) == ignore {
            continue;
        }
        if grid[idx - 1] == grid[idx] {
            for upper_idx in idx + 1..grid.len() {
                if let Some(lower) = (2 * idx - upper_idx)
                    .checked_sub(1)
                    .map(|lower_idx| &grid[lower_idx])
                {
                    if *lower != grid[upper_idx] {
                        continue 'outer;
                    }
                } else {
                    return Some(idx);
                }
            }
            return Some(idx);
        }
    }
    None
}

fn rows(pattern: &str) -> Vec<Vec<u8>> {
    pattern
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn flip_mirror(rows: &mut [Vec<u8>], row_idx: usize, col_idx: usize) {
    let mirror = &mut rows[row_idx][col_idx];
    match *mirror {
        b'.' => *mirror = b'#',
        b'#' => *mirror = b'.',
        _ => panic!("Unrecognised character"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 405);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 400);
    }
}
