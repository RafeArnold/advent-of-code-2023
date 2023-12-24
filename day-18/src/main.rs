use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut trench = HashSet::new();
    dig_edge(&mut trench, input);
    dig_interior(&mut trench);
    trench.len()
}

fn dig_edge(trench: &mut HashSet<(i32, i32)>, input: &str) {
    let mut pos = (0, 0);
    trench.insert(pos);
    for line in input.lines() {
        let (direction, rest) = line.split_once(' ').unwrap();
        let (distance, _) = rest.split_once(' ').unwrap();
        let distance = distance.parse().unwrap();
        let (dx, dy) = match direction {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            _ => (1, 0),
        };
        for _ in 0..distance {
            pos.0 += dx;
            pos.1 += dy;
            trench.insert(pos);
        }
    }
    assert_eq!(pos, (0, 0)); // We're back at the start.
}

fn dig_interior(trench: &mut HashSet<(i32, i32)>) {
    let min_x = trench.iter().map(|pos| pos.0).min().unwrap();
    let max_x = trench.iter().map(|pos| pos.0).max().unwrap();
    let min_y = trench.iter().map(|pos| pos.1).min().unwrap();
    let max_y = trench.iter().map(|pos| pos.1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if trench.contains(&(x, y)) {
                continue;
            }
            let mut queue = Vec::new();
            let mut visited = HashSet::new();
            queue.push((x, y));
            loop {
                if let Some(pos) = queue.pop() {
                    if visited.contains(&pos) || trench.contains(&pos) {
                        continue;
                    }
                    if pos.0 < min_x || pos.0 > max_x || pos.1 < min_y || pos.1 > max_y {
                        // We're outside the trench edges.
                        break;
                    }
                    visited.insert(pos);
                    queue.extend(
                        [(0, -1), (0, 1), (-1, 0), (1, 0)]
                            .into_iter()
                            .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)),
                    );
                } else {
                    // We've run out of space to search, and we haven't hit a boundary, so we must be within the trench edges.
                    trench.extend(visited);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 62);
    }
}
