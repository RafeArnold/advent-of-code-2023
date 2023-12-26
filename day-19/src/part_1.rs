use crate::common::Instruction::{Accept, Jump, Reject};
use crate::common::{parse_workflow, Instruction, Rule, Workflow};
use std::collections::HashMap;

pub fn run(input: &str) -> usize {
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

type Part = [usize; 4];
