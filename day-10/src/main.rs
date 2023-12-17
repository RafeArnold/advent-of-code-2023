use std::collections::HashSet;

fn main() {
    const INPUT: &[u8; 19739] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let tiles = parse_input(input);
    let start = find_start(&tiles);
    let start_facing = find_connected_pipes(start, &tiles);
    find_loop(&tiles, start, start_facing[0].clone()).len() / 2
}

fn run_2(input: &[u8]) -> usize {
    let mut tiles = parse_input(input);
    let start = find_start(&tiles);
    let start_facing = find_connected_pipes(start, &tiles);
    let loop0 = find_loop(&tiles, start, start_facing[0].clone());
    let start_byte = if start_facing.contains(&Direction::Up)
        && start_facing.contains(&Direction::Down)
    {
        b'|'
    } else if start_facing.contains(&Direction::Up) && start_facing.contains(&Direction::Left) {
        b'J'
    } else if start_facing.contains(&Direction::Up) && start_facing.contains(&Direction::Right) {
        b'L'
    } else if start_facing.contains(&Direction::Down) && start_facing.contains(&Direction::Left) {
        b'7'
    } else if start_facing.contains(&Direction::Down) && start_facing.contains(&Direction::Right) {
        b'F'
    } else {
        b'-'
    };
    tiles[start.0][start.1] = start_byte;
    let mut count = 0;
    for (row_idx, row) in tiles.iter().enumerate() {
        let mut inside = false;
        let mut other_end: Option<u8> = None;
        for col_idx in 0..row.len() {
            if loop0.contains(&(row_idx, col_idx)) {
                match tiles[row_idx][col_idx] {
                    b'|' => inside = !inside,
                    b @ b'L' | b @ b'F' => other_end = Some(b),
                    b'J' => {
                        if other_end.unwrap() == b'F' {
                            inside = !inside
                        }
                        other_end = None;
                    }
                    b'7' => {
                        if other_end.unwrap() == b'L' {
                            inside = !inside
                        }
                        other_end = None;
                    }
                    _ => {}
                }
                continue;
            } else if inside {
                count += 1;
            }
        }
    }
    count
}

fn find_loop(
    tiles: &[Vec<u8>],
    start: (usize, usize),
    start_facing: Direction,
) -> HashSet<(usize, usize)> {
    let mut position = start;
    let mut facing = start_facing;
    match facing {
        Direction::Up => position.0 -= 1,
        Direction::Down => position.0 += 1,
        Direction::Left => position.1 -= 1,
        Direction::Right => position.1 += 1,
    }
    let mut loop0 = HashSet::new();
    loop {
        loop0.insert(position);
        match tiles[position.0][position.1] {
            b'S' => return loop0,
            b'|' => {
                if facing == Direction::Up {
                    position.0 -= 1;
                } else {
                    position.0 += 1;
                }
            }
            b'-' => {
                if facing == Direction::Left {
                    position.1 -= 1;
                } else {
                    position.1 += 1;
                }
            }
            b'L' => {
                if facing == Direction::Left {
                    position.0 -= 1;
                    facing = Direction::Up;
                } else {
                    position.1 += 1;
                    facing = Direction::Right;
                }
            }
            b'J' => {
                if facing == Direction::Right {
                    position.0 -= 1;
                    facing = Direction::Up;
                } else {
                    position.1 -= 1;
                    facing = Direction::Left;
                }
            }
            b'7' => {
                if facing == Direction::Right {
                    position.0 += 1;
                    facing = Direction::Down;
                } else {
                    position.1 -= 1;
                    facing = Direction::Left;
                }
            }
            b'F' => {
                if facing == Direction::Left {
                    position.0 += 1;
                    facing = Direction::Down;
                } else {
                    position.1 += 1;
                    facing = Direction::Right;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn find_connected_pipes((row_idx, col_idx): (usize, usize), tiles: &[Vec<u8>]) -> [Direction; 2] {
    let mut dirs = Vec::with_capacity(2);
    if let Some(b'F' | b'L' | b'-') = tiles[row_idx].get(col_idx.wrapping_sub(1)) {
        dirs.push(Direction::Left)
    }
    if let Some(b'J' | b'7' | b'-') = tiles[row_idx].get(col_idx + 1) {
        dirs.push(Direction::Right)
    }
    if let Some(b'F' | b'7' | b'|') = row_idx
        .checked_sub(1)
        .map(|row_idx| tiles[row_idx][col_idx])
    {
        dirs.push(Direction::Up)
    }
    if let Some(b'L' | b'J' | b'|') = tiles.get(row_idx + 1).map(|row| row[col_idx]) {
        dirs.push(Direction::Down)
    }
    [dirs.pop().unwrap(), dirs.pop().unwrap()]
}

fn find_start(tiles: &[Vec<u8>]) -> (usize, usize) {
    for (row_idx, row) in tiles.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if *tile == b'S' {
                return (row_idx, col_idx);
            }
        }
    }
    unreachable!()
}

fn parse_input(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .split(|b| *b == b'\n')
        .map(|row| row.to_owned())
        .collect::<Vec<_>>()
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT_1: &[u8; 29] = b"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        const INPUT_2: &[u8; 29] = b"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(run_1(INPUT_1), 4);
        assert_eq!(run_1(INPUT_2), 8);
    }

    #[test]
    fn challenge_2() {
        const INPUT_1: &[u8; 107] = b"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        const INPUT_2: &[u8; 98] = b"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        const INPUT_3: &[u8; 209] = b".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        const INPUT_4: &[u8; 209] = b"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(run_2(INPUT_1), 4);
        assert_eq!(run_2(INPUT_2), 4);
        assert_eq!(run_2(INPUT_3), 8);
        assert_eq!(run_2(INPUT_4), 10);
    }
}
