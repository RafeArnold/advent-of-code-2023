use cached::proc_macro::cached;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    parse_input(input).map(arrangements).sum()
}

fn run_2(input: &str) -> usize {
    parse_input(input)
        .map(|(conditions, groups)| {
            (
                conditions
                    .repeat(5)
                    .chunks_exact(conditions.len())
                    .collect::<Vec<_>>()
                    .join(&Condition::Unknown),
                groups.repeat(5),
            )
        })
        .map(arrangements)
        .sum()
}

fn arrangements((conditions, groups): (Vec<Condition>, Vec<u8>)) -> usize {
    arrangements0(conditions.to_vec(), groups.to_vec(), None)
}

#[cached]
fn arrangements0(
    conditions: Vec<Condition>,
    groups: Vec<u8>,
    current_group_size: Option<u8>,
) -> usize {
    match conditions.get(0) {
        None => {
            if groups.is_empty() || (groups.len() == 1 && current_group_size == Some(groups[0])) {
                // We've reached the end of the row and all the groups have been filled.
                1
            } else {
                // We've reached the end of the row, but we have not filled all the groups.
                0
            }
        }
        Some(Condition::Operational) => {
            if let Some(current_group_size) = current_group_size {
                if groups[0] > current_group_size {
                    // The current group has not been filled.
                    0
                } else {
                    // We've reached the end of the current group.
                    arrangements0(conditions[1..].to_vec(), groups[1..].to_vec(), None)
                }
            } else {
                // Nothing else to check here. Just move on.
                arrangements0(conditions[1..].to_vec(), groups, None)
            }
        }
        Some(Condition::Damaged) => {
            if let Some(current_group_size) = current_group_size {
                if groups[0] == current_group_size {
                    // The current group is overfilled.
                    0
                } else {
                    // Carry on moving through the current group.
                    arrangements0(
                        conditions[1..].to_vec(),
                        groups,
                        Some(current_group_size + 1),
                    )
                }
            } else if groups.is_empty() {
                // There are no more groups.
                0
            } else {
                // We've entered a new group.
                arrangements0(conditions[1..].to_vec(), groups, Some(1))
            }
        }
        Some(Condition::Unknown) => {
            if let Some(current_group_size) = current_group_size {
                if groups[0] > current_group_size {
                    // The current group has not been filled yet, so the current spring has to be damaged.
                    arrangements0(
                        conditions[1..].to_vec(),
                        groups,
                        Some(current_group_size + 1),
                    )
                } else {
                    // The current group has been filled, so the current spring has to be operational.
                    arrangements0(conditions[1..].to_vec(), groups[1..].to_vec(), None)
                }
            } else {
                // We are not in a group, so the current spring could be either operational or damaged (if there are remaining groups).
                arrangements0(conditions[1..].to_vec(), groups.clone(), None)
                    + if groups.is_empty() {
                        // There are no more groups, so the spring cannot be damaged.
                        0
                    } else {
                        // There are more groups, so the spring could be damaged.
                        arrangements0(conditions[1..].to_vec(), groups.to_vec(), Some(1))
                    }
            }
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Vec<Condition>, Vec<u8>)> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> (Vec<Condition>, Vec<u8>) {
    let (conditions, groups) = line.split_once(' ').unwrap();
    let conditions = conditions
        .bytes()
        .map(|b| match b {
            b'.' => Condition::Operational,
            b'#' => Condition::Damaged,
            b'?' => Condition::Unknown,
            _ => panic!("Unrecognised condition: {}", b),
        })
        .collect();
    let groups = groups.split(',').map(|s| s.parse().unwrap()).collect();
    (conditions, groups)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 21);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 525152);
    }

    #[test]
    fn arrangements_test() {
        assert_eq!(arrangements(parse_line("?###???????? 3,2,1")), 10);
    }
}
