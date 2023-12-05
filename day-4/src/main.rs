use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    input.lines().map(read_line).map(wins).map(score).sum()
}

fn run_2(input: &str) -> usize {
    let win_counts = input.lines().map(read_line).map(wins).collect::<Vec<_>>();
    let mut card_counts = vec![1; win_counts.len()];
    for (i, win_count) in win_counts.iter().enumerate() {
        for j in 1..=*win_count {
            card_counts[i + j] += card_counts[i];
        }
    }
    card_counts.iter().sum()
}

fn read_line(line: &str) -> (HashSet<usize>, HashSet<usize>) {
    // Strip prefix.
    let line = line.split_once(": ").unwrap().1;
    let (left, right) = line.split_once(" | ").unwrap();
    (read_side(left), read_side(right))
}

fn read_side(side: &str) -> HashSet<usize> {
    side.split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn wins((a, b): (HashSet<usize>, HashSet<usize>)) -> usize {
    a.intersection(&b).count()
}

fn score(match_count: usize) -> usize {
    if match_count == 0 {
        0
    } else {
        2usize.pow((match_count - 1) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 13);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 30);
    }
}
