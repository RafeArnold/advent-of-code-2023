fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    input.split("\n\n").map(pattern).sum()
}

fn pattern(pattern: &str) -> usize {
    let rows = pattern
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    if let Some(num) = find_reflection(&rows) {
        return 100 * num;
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
    find_reflection(&cols).unwrap()
}

fn find_reflection(grid: &Vec<Vec<u8>>) -> Option<usize> {
    'outer: for idx in 1..grid.len() {
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
}
