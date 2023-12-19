use std::cmp::{max, min};

fn main() {
    const INPUT: &[u8; 19739] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    let universe = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let expanded_rows = universe
        .iter()
        .map(|row| row.iter().all(|b| b == &b'.'))
        .collect::<Vec<_>>();
    let expanded_cols = (0..width)
        .map(|col_idx| universe.iter().map(|row| row[col_idx]).all(|b| b == b'.'))
        .collect::<Vec<_>>();
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, point)| {
                if *point == b'#' {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (galaxy_idx, galaxy) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(galaxy_idx + 1) {
            sum += other.0.abs_diff(galaxy.0);
            sum += other.1.abs_diff(galaxy.1);
            sum += (min(other.0, galaxy.0)..max(other.0, galaxy.0))
                .filter(|row_idx| expanded_rows[*row_idx])
                .count();
            sum += (min(other.1, galaxy.1)..max(other.1, galaxy.1))
                .filter(|col_idx| expanded_cols[*col_idx])
                .count();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 109] = b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 374);
    }
}
