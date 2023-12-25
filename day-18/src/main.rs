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
    let (points, perimeter) = find_points(input.lines().map(parse_line));
    let area = shoelace_formula(&points);
    picks_theorem(area, perimeter)
}

fn find_points(instructions: impl Iterator<Item = Instruction>) -> (Vec<(i64, i64)>, usize) {
    let mut points = Vec::new();
    let mut pos = (0, 0);
    let mut perimeter = 0;
    for (direction, distance) in instructions {
        let dx = -(direction - 1) % 2;
        let dy = -(direction - 2) % 2;
        pos.0 += dx * distance;
        pos.1 += dy * distance;
        points.push(pos);
        perimeter += distance as usize;
    }
    assert_eq!(pos, (0, 0)); // We're back at the start.
    (points, perimeter)
}

fn shoelace_formula(points: &Vec<(i64, i64)>) -> usize {
    (0..points.len())
        .map(|i| {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];
            (y0 + y1) * (x0 - x1)
        })
        .sum::<i64>() as usize
        / 2
}

fn picks_theorem(area: usize, perimeter: usize) -> usize {
    area + perimeter / 2 + 1
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
        i64::from_str_radix(distance, 16).unwrap(),
    )
}

type Instruction = (i64, i64);

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
