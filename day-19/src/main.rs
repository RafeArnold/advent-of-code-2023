use crate::Instruction::{Accept, Jump, Reject};
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);
    parts
        .into_iter()
        .filter(|part| execute(part, &workflows))
        .map(|part| part.iter().sum::<usize>())
        .sum()
}

fn execute(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut workflow = &workflows["in"];
    loop {
        match workflow.execute(part) {
            Accept => return true,
            Reject => return false,
            Jump(instruction) => workflow = &workflows[instruction],
        }
    }
}

impl<'a> Workflow<'a> {
    fn execute(&self, part: &Part) -> &Instruction<'a> {
        self.rules
            .iter()
            .find_map(|rule| rule.execute(part))
            .unwrap_or(&self.final_instruction)
    }
}

impl<'a> Rule<'a> {
    fn execute(&self, part: &Part) -> Option<&Instruction<'a>> {
        if part[self.condition.category].cmp(&self.condition.cmp) == self.condition.ordering {
            Some(&self.instruction)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows.lines().map(parse_workflow).collect();
    let parts = parts.lines().map(parse_part).collect();

    (workflows, parts)
}

fn parse_workflow(line: &str) -> (&str, Workflow) {
    let (key, rest) = line.split_once('{').unwrap();
    let rules = &rest[..rest.len() - 1];
    let mut rules = rules.split(',');
    let final_instruction = rules.next_back().map(parse_instruction).unwrap();
    let rules = rules.map(parse_rule).collect();
    let workflow = Workflow {
        rules,
        final_instruction,
    };
    (key, workflow)
}

fn parse_rule(rule: &str) -> Rule {
    let (condition, instruction) = rule.split_once(':').unwrap();
    let condition = parse_condition(condition);
    let instruction = parse_instruction(instruction);
    Rule {
        condition,
        instruction,
    }
}

fn parse_condition(condition: &str) -> Condition {
    let bytes = condition.as_bytes();
    let category = match bytes[0] {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        _ => 3,
    };
    let ordering = match bytes[1] {
        b'>' => Greater,
        _ => Less,
    };
    let cmp = condition.split_at(2).1.parse().unwrap();
    Condition {
        category,
        ordering,
        cmp,
    }
}

fn parse_instruction(instruction: &str) -> Instruction {
    match instruction {
        "A" => Accept,
        "R" => Reject,
        _ => Jump(instruction),
    }
}

fn parse_part(part: &str) -> Part {
    let mut categories = part[1..part.len() - 1].split(',').map(parse_category);
    [
        categories.next().unwrap(),
        categories.next().unwrap(),
        categories.next().unwrap(),
        categories.next().unwrap(),
    ]
}

fn parse_category(category: &str) -> usize {
    category[2..].parse().unwrap()
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    final_instruction: Instruction<'a>,
}

struct Rule<'a> {
    condition: Condition,
    instruction: Instruction<'a>,
}

struct Condition {
    category: usize,
    ordering: Ordering,
    cmp: usize,
}

enum Instruction<'a> {
    Accept,
    Reject,
    Jump(&'a str),
}

type Part = [usize; 4];

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 19114);
    }
}
