use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use Instruction::{Accept, Jump, Reject};

pub struct Workflow<'a> {
    pub(crate) rules: Vec<Rule<'a>>,
    pub(crate) final_instruction: Instruction<'a>,
}

pub struct Rule<'a> {
    pub(crate) condition: Condition,
    pub(crate) instruction: Instruction<'a>,
}

pub struct Condition {
    pub(crate) category: usize,
    pub(crate) ordering: Ordering,
    pub(crate) cmp: usize,
}

pub enum Instruction<'a> {
    Accept,
    Reject,
    Jump(&'a str),
}

pub fn parse_workflow(line: &str) -> (&str, Workflow) {
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
