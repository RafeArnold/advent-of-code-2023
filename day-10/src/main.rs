fn main() {
    const INPUT: &[u8; 19739] = include_bytes!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &[u8]) -> usize {
    let tiles = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    let mut position = find_start(&tiles);
    let mut facing = find_first_connected_pipe(position, &tiles);
    match facing {
        Direction::Up => position.0 -= 1,
        Direction::Down => position.0 += 1,
        Direction::Left => position.1 -= 1,
        Direction::Right => position.1 += 1,
    }
    let mut steps = 0;
    loop {
        match tiles[position.0][position.1] {
            b'S' => return steps / 2 + 1,
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
        steps += 1;
    }
}

fn find_first_connected_pipe((row_idx, col_idx): (usize, usize), tiles: &[&[u8]]) -> Direction {
    if let Some(b'F' | b'L' | b'-') = tiles[row_idx].get(col_idx.wrapping_sub(1)) {
        Direction::Left
    } else if let Some(b'J' | b'7' | b'-') = tiles[row_idx].get(col_idx + 1) {
        Direction::Right
    } else {
        Direction::Up
    }
}

fn find_start(tiles: &[&[u8]]) -> (usize, usize) {
    for (row_idx, row) in tiles.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if *tile == b'S' {
                return (row_idx, col_idx);
            }
        }
    }
    unreachable!()
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT_1), 4);
        assert_eq!(run_1(INPUT_2), 8);
    }
}
