use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut rocks = parse_input(input);
    roll(&mut rocks);
    load(rocks)
}

fn run_2(input: &str) -> usize {
    let rocks = parse_input(input);
    let width = rocks[0].len();
    let mut rocks = rocks.concat();
    let mut mem = HashMap::<Vec<u8>, usize>::new();
    for i in 0.. {
        mem.insert(rocks.clone(), i);
        cycle(&mut rocks, width);
        if let Some(cycle_start) = mem.get(&rocks) {
            let idx = cycle_start + (1_000_000_000 - cycle_start) % (i + 1 - cycle_start);
            rocks = mem.into_iter().find(|(_, value)| *value == idx).unwrap().0;
            break;
        }
    }
    load(
        rocks
            .chunks_exact(width)
            .map(|line| line.to_vec())
            .collect::<Vec<_>>(),
    )
}

fn cycle(rocks: &mut Vec<u8>, width: usize) {
    for row_idx in 0..rocks.len() / width {
        for col_idx in 0..width {
            if rocks[row_idx * width + col_idx] == b'O' {
                let mut to_idx = row_idx;
                for north_idx in (0..row_idx).rev() {
                    if rocks[north_idx * width + col_idx] == b'.' {
                        to_idx = north_idx;
                    } else {
                        break;
                    }
                }
                rocks[row_idx * width + col_idx] = b'.';
                rocks[to_idx * width + col_idx] = b'O';
            }
        }
    }
    for col_idx in 0..width {
        for row_idx in 0..rocks.len() / width {
            if rocks[row_idx * width + col_idx] == b'O' {
                let mut to_idx = col_idx;
                for west_idx in (0..col_idx).rev() {
                    if rocks[row_idx * width + west_idx] == b'.' {
                        to_idx = west_idx;
                    } else {
                        break;
                    }
                }
                rocks[row_idx * width + col_idx] = b'.';
                rocks[row_idx * width + to_idx] = b'O';
            }
        }
    }
    for row_idx in (0..rocks.len() / width).rev() {
        for col_idx in 0..width {
            if rocks[row_idx * width + col_idx] == b'O' {
                let mut to_idx = row_idx;
                for south_idx in row_idx + 1..rocks.len() / width {
                    if rocks[south_idx * width + col_idx] == b'.' {
                        to_idx = south_idx;
                    } else {
                        break;
                    }
                }
                rocks[row_idx * width + col_idx] = b'.';
                rocks[to_idx * width + col_idx] = b'O';
            }
        }
    }
    for col_idx in (0..width).rev() {
        for row_idx in 0..rocks.len() / width {
            if rocks[row_idx * width + col_idx] == b'O' {
                let mut to_idx = col_idx;
                for east_idx in col_idx + 1..width {
                    if rocks[row_idx * width + east_idx] == b'.' {
                        to_idx = east_idx;
                    } else {
                        break;
                    }
                }
                rocks[row_idx * width + col_idx] = b'.';
                rocks[row_idx * width + to_idx] = b'O';
            }
        }
    }
}

fn roll(rocks: &mut Vec<Vec<u8>>) {
    for row_idx in 0..rocks.len() {
        let (above, rest) = rocks.split_at_mut(row_idx);
        for (col_idx, rock) in rest[0].iter_mut().enumerate() {
            if *rock == b'O' {
                let mut to_idx = None;
                for above_idx in (0..row_idx).rev() {
                    if above[above_idx][col_idx] == b'.' {
                        to_idx = Some(above_idx);
                    } else {
                        break;
                    }
                }
                if let Some(to_idx) = to_idx {
                    above[to_idx][col_idx] = b'O';
                    *rock = b'.';
                }
            }
        }
    }
}

fn load(rocks: Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for (distance, row) in rocks.iter().rev().enumerate() {
        for rock in row {
            if *rock == b'O' {
                sum += distance + 1;
            }
        }
    }
    sum
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 136);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 64);
    }
}
