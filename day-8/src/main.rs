use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let (steps, nodes) = parse_input(input);
    let mut current_node = nodes["AAA"];
    for (steps_taken, step) in steps.iter().cycle().enumerate() {
        let next_node = match step {
            Step::L => current_node.0,
            Step::R => current_node.1,
        };
        if next_node == "ZZZ" {
            return steps_taken + 1;
        }
        current_node = nodes[next_node];
    }
    unreachable!()
}

fn parse_input(input: &str) -> (Vec<Step>, HashMap<&str, (&str, &str)>) {
    let (steps, nodes) = input.split_once("\n\n").unwrap();
    let steps = steps
        .bytes()
        .map(|c| match c {
            b'L' => Step::L,
            b'R' => Step::R,
            _ => panic!("Unrecognised step {c}"),
        })
        .collect();
    let nodes = nodes.lines().map(parse_node).collect();
    (steps, nodes)
}

fn parse_node(node: &str) -> (&str, (&str, &str)) {
    let key = &node[0..3];
    let left = &node[7..10];
    let right = &node[12..15];
    (key, (left, right))
}

enum Step {
    L,
    R,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        const INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        const INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run_1(INPUT_1), 2);
        assert_eq!(run_1(INPUT_2), 6);
    }
}
