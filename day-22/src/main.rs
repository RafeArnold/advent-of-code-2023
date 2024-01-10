fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks = fall(bricks);
    disintegrate(&bricks)
}

fn disintegrate(bricks: &Vec<Brick>) -> usize {
    let mut bricks_sorted_by_start =
        vec![Vec::new(); bricks.iter().map(|brick| brick.end.z).max().unwrap() + 1];
    let mut bricks_sorted_by_end = bricks_sorted_by_start.clone();
    for brick in bricks {
        bricks_sorted_by_start[brick.start.z].push(brick);
        bricks_sorted_by_end[brick.end.z].push(brick);
    }
    bricks
        .iter()
        .filter(|brick| can_disintegrate(brick, &bricks_sorted_by_start, &bricks_sorted_by_end))
        .count()
}

fn can_disintegrate(
    brick: &Brick,
    bricks_sorted_by_start: &[Vec<&Brick>],
    bricks_sorted_by_end: &[Vec<&Brick>],
) -> bool {
    // Find all the bricks this brick is supporting and check if those bricks have more than one supporting brick.
    for supported_brick in bricks_sorted_by_start
        .get(brick.end.z + 1)
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|other| other.is_z_aligned(brick))
    {
        if count_supports(supported_brick, bricks_sorted_by_end) == 1 {
            // This brick is the only support for at least one other brick.
            return false;
        }
    }
    // All supported bricks have multiple supports, or this brick is not supporting any other bricks.
    true
}

fn count_supports(brick: &Brick, bricks_sorted_by_end: &[Vec<&Brick>]) -> usize {
    bricks_sorted_by_end[brick.start.z - 1]
        .iter()
        .filter(|other| other.is_z_aligned(brick))
        .count()
}

fn fall(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_unstable_by_key(|brick| brick.start.z);
    let mut fallen = Vec::<Brick>::new();
    for mut brick in bricks {
        // Find the highest fallen brick underneath this brick.
        let to_move = if let Some(z) = fallen
            .iter()
            .filter(|fallen_brick| fallen_brick.is_z_aligned(&brick))
            .map(|fallen_brick| fallen_brick.end.z)
            .max()
        {
            // Move this brick down onto the brick underneath.
            brick.start.z - z - 1
        } else {
            brick.start.z - 1
        };
        brick.start.z -= to_move;
        brick.end.z -= to_move;
        fallen.push(brick);
    }
    fallen
}

impl Brick {
    fn is_z_aligned(&self, other: &Brick) -> bool {
        is_aligned((self.start.x, self.end.x), (other.start.x, other.end.x))
            && is_aligned((self.start.y, self.end.y), (other.start.y, other.end.y))
    }
}

fn is_aligned(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            Brick::new(parse_coords(start), parse_coords(end))
        })
        .collect()
}

fn parse_coords(coords: &str) -> Coords {
    let mut coords = coords.split(',').map(|coord| coord.parse().unwrap());
    Coords::new(
        coords.next().unwrap(),
        coords.next().unwrap(),
        coords.next().unwrap(),
    )
}

struct Brick {
    start: Coords,
    end: Coords,
}

impl Brick {
    fn new(start: Coords, end: Coords) -> Self {
        Self { start, end }
    }
}

struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

impl Coords {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 5);
    }
}
