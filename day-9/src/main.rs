fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> i32 {
    run(
        input,
        &|values| values.last().copied(),
        &|next_value, last_value| next_value + last_value,
    )
}

fn run_2(input: &str) -> i32 {
    run(
        input,
        &|values| values.first().copied(),
        &|next_value, first_value| first_value - next_value,
    )
}

fn run<E, O>(input: &str, extract: &E, op: &O) -> i32
where
    E: Fn(Vec<i32>) -> Option<i32>,
    O: Fn(i32, i32) -> i32,
{
    let histories = parse_input(input);
    histories
        .into_iter()
        .map(|history| next_value(history, extract, op))
        .sum()
}

fn next_value<E, O>(history: Vec<i32>, extract: E, op: O) -> i32
where
    E: Fn(Vec<i32>) -> Option<i32>,
    O: Fn(i32, i32) -> i32,
{
    let mut sequences = Vec::new();
    sequences.push(history);
    loop {
        let last_sequence = sequences.last().unwrap();
        let mut next_sequence = Vec::with_capacity(last_sequence.len() - 1);
        for idx in 0..(last_sequence.len() - 1) {
            next_sequence.push(last_sequence[idx + 1] - last_sequence[idx]);
        }
        if next_sequence.iter().all(|val| *val == 0) {
            break;
        }
        sequences.push(next_sequence);
    }
    let mut next_value = 0;
    for value in sequences
        .into_iter()
        .map(|values| extract(values).unwrap())
        .rev()
    {
        next_value = op(next_value, value);
    }
    next_value
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 114);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 2);
    }
}
