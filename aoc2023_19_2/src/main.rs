use std::collections::HashMap;
use std::io::stdin;

use anyhow::{Context, Error};
use rayon::prelude::*;

#[derive(Default, Debug)]
struct Part {
    a: u16,
    m: u16,
    s: u16,
    x: u16,
}

#[derive(Default, Debug)]
struct Rule {
    operator: u8,
    property: u8,
    target: Vec<u8>,
    threshold: u16,
}

#[derive(Default, Debug)]
struct Workflow {
    rules: Vec<Rule>,
    target: Vec<u8>,
}

fn main() -> Result<(), Error> {
    let mut workflows = HashMap::<Vec<u8>, Workflow>::new();

    let mut boundaries_a = Vec::<u16>::new();
    let mut boundaries_m = Vec::<u16>::new();
    let mut boundaries_s = Vec::<u16>::new();
    let mut boundaries_x = Vec::<u16>::new();

    let mut workflows_over = false;

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
                                .parse()?;
                    }
                }
                if rule_complete {
                    eprintln!("rule {:?}", rule);
                    vec_set_insert(
                        match rule.property {
                            b'a' => &mut boundaries_a,
                            b'm' => &mut boundaries_m,
                            b's' => &mut boundaries_s,
                            b'x' => &mut boundaries_x,
                            _ => panic!("invalid property"),
                        },
                        // make each boundary value be the last number in that range
                        rule.threshold - if rule.operator == b'<' { 1 } else { 0 },
                    );
                    workflow.rules.push(rule);
                }
            }

            workflows.insert(name, workflow);
        } else {
        }
    }

    boundaries_a.push(4000);
    boundaries_m.push(4000);
    boundaries_s.push(4000);
    boundaries_x.push(4000);

    eprintln!("boundaries_a {} {:?}", boundaries_a.len(), boundaries_a);
    eprintln!("boundaries_m {} {:?}", boundaries_m.len(), boundaries_m);
    eprintln!("boundaries_s {} {:?}", boundaries_s.len(), boundaries_s);
    eprintln!("boundaries_x {} {:?}", boundaries_x.len(), boundaries_x);

    let total_combos: u64 = boundaries_a
        .par_iter()
        .enumerate()
        .map(|(i_a, a)| {
            let mut a_combos = 0;
            for (i_m, m) in boundaries_m.iter().enumerate() {
                for (i_s, s) in boundaries_s.iter().enumerate() {
                    for (i_x, x) in boundaries_x.iter().enumerate() {
                        let last_target = run_workflows(
                            &workflows,
                            &Part {
                                a: *a,
                                m: *m,
                                s: *s,
                                x: *x,
                            },
                        );
                        if last_target != b"A" {
                            continue;
                        }

                        let a_before = if i_a == 0 {
                            0
                        } else {
                            boundaries_a[i_a - 1] as u64
                        };
                        let m_before = if i_m == 0 {
                            0
                        } else {
                            boundaries_m[i_m - 1] as u64
                        };
                        let s_before = if i_s == 0 {
                            0
                        } else {
                            boundaries_s[i_s - 1] as u64
                        };
                        let x_before = if i_x == 0 {
                            0
                        } else {
                            boundaries_x[i_x - 1] as u64
                        };

                        let combos = (*a as u64 - a_before)
                            * (*m as u64 - m_before)
                            * (*s as u64 - s_before)
                            * (*x as u64 - x_before);

                        if false {
                            eprintln!(
                                "combos {} a ({}, {}] m ({}, {}] s ({}, {}] x ({}, {}]",
                                combos, a_before, a, m_before, m, s_before, s, x_before, x
                            );
                        }

                        a_combos += combos;
                    }
                }
            }
            a_combos
        })
        .sum();

    println!("{}", total_combos);

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
                b'<' => u16::lt,
                b'>' => u16::gt,
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

fn vec_set_insert<T>(vec: &mut Vec<T>, value: T)
where
    T: Ord,
{
    if let Err(pos) = vec.binary_search(&value) {
        vec.insert(pos, value);
    }
}
