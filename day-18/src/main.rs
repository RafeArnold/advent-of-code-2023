use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    run(input, parse_line_1)
}

fn run_2(input: &str) -> usize {
    run(input, parse_line_2)
}

fn run(input: &str, parse_line: fn(&str) -> Instruction) -> usize {
    let edges = find_edges(input.lines().map(parse_line));
    count_interior(&edges)
}

fn find_edges(instructions: impl Iterator<Item = Instruction>) -> Edges {
    let mut edges = Edges::default();
    let mut pos = (0, 0);
    for (direction, distance) in instructions {
        if direction % 2 == 1 {
            let d = -(direction - 2) % 2;
            let range = (pos.1, pos.1 + d * distance);
            edges
                .vertical_edges
                .entry(pos.0)
                .or_default()
                .insert((min(range.0, range.1), max(range.0, range.1)));
            pos.1 += d * distance;
        } else {
            let d = -(direction - 1) % 2;
            let range = (pos.0, pos.0 + d * distance);
            edges
                .horizontal_edges
                .entry(pos.1)
                .or_default()
                .insert((min(range.0, range.1), max(range.0, range.1)));
            pos.0 += d * distance;
        }
    }
    assert_eq!(pos, (0, 0)); // We're back at the start.
    edges
}

fn count_interior(edges: &Edges) -> usize {
    let mut count = 0;

    let (min_x, _max_x) = edges.horizontal_boundaries();
    let (min_y, max_y) = edges.vertical_boundaries();

    for y in min_y..=max_y {
        let mut inside = false;
        // We're going to assume the edges of the loop do not touch or intersect.
        let mut x = min_x - 1;
        while let Some((&edge_x, &(edge_from_y, edge_to_y))) =
            // Find the first vertical edge that intersects with y.
            edges
                .vertical_edges
                .range(x + 1..)
                .find_map(|(idx, ranges)| {
                    ranges
                        .range(..=(y, max_y))
                        .rfind(|(min, max)| y >= *min && y <= *max)
                        .map(|range| (idx, range))
                })
        {
            if inside {
                // Add the interior space from our current position to the next edge to the count.
                count += (edge_x - x - 1) as usize;
            }
            // Move our current position to the edge.
            x = edge_x;
            // Add the edge's square to the count.
            count += 1;
            if edge_from_y == y || edge_to_y == y {
                // We're on one end of the edge.
                // Find the connected horizontal edge.
                let &(_, connected_horizontal_edge_end) = edges
                    .horizontal_edges
                    .get(&y)
                    .unwrap()
                    .range((x, i32::MIN)..)
                    .next()
                    .unwrap();
                // Add the length of the edge to the count.
                count += (connected_horizontal_edge_end - x) as usize;
                x = connected_horizontal_edge_end;
                // Find the connected vertical edge at the end of the horizontal edge.
                let &(connected_vertical_edge_from_y, connected_vertical_edge_to_y) = edges
                    .vertical_edges
                    .get(&x)
                    .unwrap()
                    .range(..=(y, i32::MAX))
                    .next_back()
                    .unwrap();
                if (edge_from_y == y && connected_vertical_edge_to_y == y)
                    || (edge_to_y == y && connected_vertical_edge_from_y == y)
                {
                    // Both vertical edges continue in the same direction, so we are passing from the outside to the inside, or vice versa.
                    inside = !inside;
                }
            } else {
                // We're in the middle of the edge, so we are definitely passing from the outside to the inside, or vice versa.
                inside = !inside;
            }
        }
    }

    count
}

#[derive(Debug, Default)]
struct Edges {
    horizontal_edges: BTreeMap<i32, BTreeSet<(i32, i32)>>,
    vertical_edges: BTreeMap<i32, BTreeSet<(i32, i32)>>,
}

impl Edges {
    fn vertical_boundaries(&self) -> (i32, i32) {
        Self::boundaries(&self.horizontal_edges)
    }

    fn horizontal_boundaries(&self) -> (i32, i32) {
        Self::boundaries(&self.vertical_edges)
    }

    fn boundaries(edges: &BTreeMap<i32, BTreeSet<(i32, i32)>>) -> (i32, i32) {
        let min = *edges.keys().min().unwrap();
        let max = *edges.keys().max().unwrap();
        (min, max)
    }
}

fn parse_line_1(line: &str) -> Instruction {
    let (direction, rest) = line.split_once(' ').unwrap();
    let (distance, _) = rest.split_once(' ').unwrap();
    let direction = match direction {
        "R" => 0,
        "D" => 1,
        "L" => 2,
        _ => 3,
    };
    (direction, distance.parse().unwrap())
}

fn parse_line_2(line: &str) -> Instruction {
    let (_, instruction) = line.split_once(' ').unwrap();
    let (_, instruction) = instruction.split_once(' ').unwrap();
    let instruction = &instruction[2..instruction.len() - 1];
    let (distance, direction) = instruction.split_at(instruction.len() - 1);
    (
        direction.parse().unwrap(),
        i32::from_str_radix(distance, 16).unwrap(),
    )
}

type Instruction = (i32, i32);

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

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 952408144115);
    }
}
