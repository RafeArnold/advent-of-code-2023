use crate::common::Instruction::{Accept, Jump, Reject};
use crate::common::{parse_workflow, Instruction, Workflow};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let workflows = parse_input(input);
    let part: PossiblePart = [(1, 4000); 4];
    execute(part, &workflows, &Jump("in"))
}

fn execute(
    mut part: PossiblePart,
    workflows: &HashMap<&str, Workflow>,
    instruction: &Instruction,
) -> usize {
    let workflow = match instruction {
        Accept => return possibilities(part),
        Reject => return 0,
        Jump(key) => &workflows[key],
    };
    let mut possibilitites = 0;
    for rule in &workflow.rules {
        let (category_min, category_max) = part[rule.condition.category];
        if category_min > rule.condition.cmp {
            match rule.condition.ordering {
                Ordering::Less => continue, // Will always fail, so continue to the next rule without splitting.
                _ => return possibilitites + execute(part, workflows, &rule.instruction), // Will always succeed, so execute the rule's instruction without splitting.
            }
        } else if category_max < rule.condition.cmp {
            match rule.condition.ordering {
                Ordering::Greater => continue, // Will always fail, so continue to the next rule without splitting.
                _ => return possibilitites + execute(part, workflows, &rule.instruction), // Will always succeed, so execute the rule's instruction without splitting.
            }
        } else {
            // Split and execute each possibility.
            let mut new_part = part;
            match rule.condition.ordering {
                Ordering::Less => {
                    new_part[rule.condition.category].1 = rule.condition.cmp - 1;
                    part[rule.condition.category].0 = rule.condition.cmp;
                }
                _ => {
                    new_part[rule.condition.category].0 = rule.condition.cmp + 1;
                    part[rule.condition.category].1 = rule.condition.cmp;
                }
            }
            possibilitites += execute(new_part, workflows, &rule.instruction);
        }
    }
    possibilitites + execute(part, workflows, &workflow.final_instruction)
}

fn possibilities(possible_part: PossiblePart) -> usize {
    possible_part
        .into_iter()
        .map(|(min, max)| max - min + 1)
        .product()
}

fn parse_input(input: &str) -> HashMap<&str, Workflow> {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    workflows.lines().map(parse_workflow).collect()
}

type PossiblePart = [(usize, usize); 4];
