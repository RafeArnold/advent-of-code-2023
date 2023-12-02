use once_cell::sync::Lazy;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| if is_possible(line) { Some(i + 1) } else { None })
        .sum()
}

fn is_possible(line: &str) -> bool {
    // Strip prefix
    let line = line.split_once(": ").unwrap().1;
    let mut rounds = line.split("; ");
    rounds.all(is_round_possible)
}

fn is_round_possible(round: &str) -> bool {
    let mut cubes = round.split(", ");
    cubes.all(is_cube_count_possible)
}

fn is_cube_count_possible(cubes: &str) -> bool {
    let (count, colour) = cubes.split_once(' ').unwrap();
    count.parse::<u8>().unwrap() <= *MAX_COUNTS.get(colour).unwrap()
}

static MAX_COUNTS: Lazy<HashMap<&str, u8>> =
    Lazy::new(|| HashMap::from([("red", 12), ("green", 13), ("blue", 14)]));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn challenge_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(run_1(input), 8);
    }
}
