use once_cell::sync::Lazy;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| if is_possible(line) { Some(i + 1) } else { None })
        .sum()
}

fn run_2(input: &str) -> usize {
    input.lines().map(power).sum()
}

fn is_possible(line: &str) -> bool {
    to_cubes(line).all(|(count, colour)| count <= *MAX_COUNTS.get(colour).unwrap())
}

static MAX_COUNTS: Lazy<HashMap<&str, usize>> =
    Lazy::new(|| HashMap::from([("red", 12), ("green", 13), ("blue", 14)]));

fn power(line: &str) -> usize {
    let counts = to_cubes(line).fold(
        HashMap::<&str, usize>::with_capacity(3),
        |mut acc, (count, colour)| {
            acc.entry(colour)
                .and_modify(|max| {
                    if count > *max {
                        *max = count
                    }
                })
                .or_insert(count);
            acc
        },
    );
    counts.values().product()
}

fn to_cubes(line: &str) -> impl Iterator<Item = (usize, &str)> {
    line.split_once(": ")
        .unwrap()
        .1
        .split("; ")
        .flat_map(|round| {
            round.split(", ").map(|cubes| {
                let (count, colour) = cubes.split_once(' ').unwrap();
                (count.parse().unwrap(), colour)
            })
        })
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 8);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 2286);
    }
}
