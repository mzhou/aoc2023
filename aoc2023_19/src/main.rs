use std::collections::HashMap;
use std::io::stdin;

use anyhow::{Context, Error};

#[derive(Default, Debug)]
struct Part {
    a: u64,
    m: u64,
    s: u64,
    x: u64,
}

#[derive(Default, Debug)]
struct Rule {
    operator: u8,
    property: u8,
    target: Vec<u8>,
    threshold: u64,
}

#[derive(Default, Debug)]
struct Workflow {
    rules: Vec<Rule>,
    target: Vec<u8>,
}

fn main() -> Result<(), Error> {
    let mut workflows = HashMap::<Vec<u8>, Workflow>::new();

    let mut workflows_over = false;

    let mut total_rating = 0;

    for line in stdin().lines() {
        let line = line?;
        let line_bytes = line.as_bytes();

        if line_bytes.is_empty() {
            workflows_over = true;
            continue;
        }

        if !workflows_over {
            let open_brace_pos = line_bytes
                .iter()
                .position(|b| *b == b'{')
                .context("missing open brace")?;
            let name = line_bytes[0..open_brace_pos].to_owned();
            let workflow_parts =
                line_bytes[open_brace_pos + 1..line_bytes.len() - 1].split(|b| *b == b',');

            let mut workflow = Workflow::default();

            for workflow_bytes in workflow_parts {
                eprintln!("workflow_bytes {}", String::from_utf8_lossy(workflow_bytes));
                let rule_parts = workflow_bytes.split_inclusive(|b| *b == b':');
                let mut first = true;
                let mut rule = Rule::default();
                let mut rule_complete = false;
                for rule_bytes in rule_parts {
                    eprintln!("rule_bytes {}", String::from_utf8_lossy(rule_bytes));
                    if !first {
                        rule.target = rule_bytes.to_owned();
                        rule_complete = true;
                        break;
                    }
                    first = false;
                    if *rule_bytes.last().context("missing rule")? != b':' {
                        // last
                        workflow.target = rule_bytes.to_owned();
                    } else {
                        // before last
                        rule.operator = rule_bytes[1];
                        rule.property = rule_bytes[0];
                        rule.threshold =
                            String::from_utf8_lossy(&rule_bytes[2..rule_bytes.len() - 1])
                                .parse::<u64>()?;
                    }
                }
                if rule_complete {
                    eprintln!("rule {:?}", rule);
                    workflow.rules.push(rule);
                }
            }

            workflows.insert(name, workflow);
        } else {
            let mut sections = line_bytes[1..line_bytes.len() - 1].split(|b| *b == b',');
            let x = String::from_utf8_lossy(&sections.next().context("missing x")?[2..])
                .parse::<u64>()?;
            let m = String::from_utf8_lossy(&sections.next().context("missing m")?[2..])
                .parse::<u64>()?;
            let a = String::from_utf8_lossy(&sections.next().context("missing a")?[2..])
                .parse::<u64>()?;
            let s = String::from_utf8_lossy(&sections.next().context("missing s")?[2..])
                .parse::<u64>()?;
            let part = Part { a, m, s, x };
            let workflow_result = run_workflows(&workflows, &part);
            if workflow_result == b"A" {
                let rating = part.a + part.m + part.s + part.x;
                total_rating += rating;
            }
        }
    }

    println!("{}", total_rating);

    Ok(())
}

fn run_workflows(workflows: &HashMap<Vec<u8>, Workflow>, part: &Part) -> Vec<u8> {
    let mut current_name = b"in".as_slice();
    while let Some(workflow) = workflows.get(current_name) {
        let mut rule_matched = false;
        for rule in workflow.rules.iter() {
            let value = match rule.property {
                b'a' => part.a,
                b'm' => part.m,
                b's' => part.s,
                b'x' => part.x,
                _ => panic!("invalid property"),
            };
            let op = match rule.operator {
                b'<' => u64::lt,
                b'>' => u64::gt,
                _ => panic!("invalid operator"),
            };
            if op(&value, &rule.threshold) {
                current_name = &rule.target;
                rule_matched = true;
                break;
            }
        }
        if !rule_matched {
            current_name = &workflow.target;
        }
    }
    current_name.to_vec()
}
