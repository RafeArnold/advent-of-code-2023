use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    run(input.as_bytes(), 64)
}

const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn run(input: &[u8], steps: usize) -> usize {
    let (grid, start) = parse_input(input);
    let mut positions = HashSet::new();
    positions.insert(start);
    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for (x, y) in positions {
            for (dx, dy) in MOVES {
                let new_position = (
                    x.checked_add_signed(dx).unwrap(),
                    y.checked_add_signed(dy).unwrap(),
                );
                if !grid[new_position.1][new_position.0] {
                    new_positions.insert(new_position);
                }
            }
        }
        positions = new_positions;
    }
    positions.len()
}

fn parse_input(input: &[u8]) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut start = None;
    let grid = input
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, &b)| {
                    if b == b'S' {
                        start = Some((col_idx, row_idx));
                    }
                    b == b'#'
                })
                .collect()
        })
        .collect();
    (grid, start.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 131] = b"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn challenge_1() {
        assert_eq!(run(INPUT, 6), 16);
    }
}
