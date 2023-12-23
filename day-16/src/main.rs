use crate::Direction::{Down, Left, Right, Up};
use std::collections::HashMap;

fn main() {
    const INPUT: &[u8; 12209] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let grid = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let facing = Right;
    let pos = (0, 0);
    run(&grid, facing, pos)
}

fn run_2(input: &[u8]) -> usize {
    let grid = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let width = grid[0].len();
    (0..grid.len())
        .flat_map(|row_idx| {
            [
                run(&grid, Right, (0, row_idx)),
                run(&grid, Left, (width - 1, row_idx)),
            ]
        })
        .chain((0..width).flat_map(|col_idx| {
            [
                run(&grid, Down, (col_idx, 0)),
                run(&grid, Up, (col_idx, grid.len() - 1)),
            ]
        }))
        .max()
        .unwrap()
}

fn run(grid: &[&[u8]], facing: Direction, pos: (usize, usize)) -> usize {
    let mut visited = HashMap::<(usize, usize), Vec<Direction>>::new();
    step(grid, facing, pos, &mut visited);
    visited.len()
}

fn step(
    grid: &[&[u8]],
    facing: Direction,
    pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    let directions = visited.entry(pos).or_default();
    if directions.contains(&facing) {
        return;
    }
    directions.push(facing.clone());
    match grid[pos.1][pos.0] {
        b'/' => match facing {
            Up => try_right(grid, pos, visited),
            Down => try_left(grid, pos, visited),
            Left => try_down(grid, pos, visited),
            Right => try_up(grid, pos, visited),
        },
        b'\\' => match facing {
            Up => try_left(grid, pos, visited),
            Down => try_right(grid, pos, visited),
            Left => try_up(grid, pos, visited),
            Right => try_down(grid, pos, visited),
        },
        b'|' => match facing {
            Up => try_up(grid, pos, visited),
            Down => try_down(grid, pos, visited),
            Left | Right => {
                try_up(grid, pos, visited);
                try_down(grid, pos, visited);
            }
        },
        b'-' => match facing {
            Up | Down => {
                try_left(grid, pos, visited);
                try_right(grid, pos, visited);
            }
            Left => try_left(grid, pos, visited),
            Right => try_right(grid, pos, visited),
        },
        _ => {
            // Must be b'.'.
            match facing {
                Up => try_up(grid, pos, visited),
                Down => try_down(grid, pos, visited),
                Left => try_left(grid, pos, visited),
                Right => try_right(grid, pos, visited),
            }
        }
    }
}

fn try_up(
    grid: &[&[u8]],
    mut pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    if pos.1 > 0 {
        let facing = Up;
        pos.1 -= 1;
        step(grid, facing, pos, visited);
    }
}

fn try_down(
    grid: &[&[u8]],
    mut pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    if pos.1 < grid.len() - 1 {
        let facing = Down;
        pos.1 += 1;
        step(grid, facing, pos, visited);
    }
}

fn try_left(
    grid: &[&[u8]],
    mut pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    if pos.0 > 0 {
        let facing = Left;
        pos.0 -= 1;
        step(grid, facing, pos, visited);
    }
}

fn try_right(
    grid: &[&[u8]],
    mut pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    if pos.0 < grid[0].len() - 1 {
        let facing = Right;
        pos.0 += 1;
        step(grid, facing, pos, visited);
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8; 109] = br#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 46);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 51);
    }
}
