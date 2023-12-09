use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

use anyhow::{Context, Error};
use itertools::Itertools;
use num::integer::lcm;

fn list_solutions(
    nodes: &HashMap<String, (String, String)>,
    instructions: &[u8],
    start_pos: &str,
) -> Result<Vec<usize>, Error> {
    eprintln!("list_solutions({})", start_pos);
    let mut pos = start_pos;
    let mut seen = HashSet::<(&str, usize)>::new();
    let mut solutions = Vec::<usize>::new();
    let mut steps = 0;
    'outer: loop {
        for i in 0..instructions.len() {
            let instruction = instructions[i];

            if !seen.insert((pos, i)) {
                eprintln!("cycle at {} {}", pos, i);
                break 'outer;
            }

            let node = nodes.get(pos).context("invalid node")?;
            pos = match instruction {
                b'L' => node.0.as_str(),
                b'R' => node.1.as_str(),
                _ => panic!("invalid instruction"),
            };
            eprintln!("now at {}", pos);
            steps += 1;
            if pos.ends_with('Z') {
                eprintln!("solution at {}, {} steps", pos, steps);
                solutions.push(steps);
            }
        }
    }
    eprintln!("list_solutions({}) end", start_pos);
    Ok(solutions)
}

fn main() -> Result<(), Error> {
    let mut instructions = Vec::<u8>::new();
    let mut nodes = HashMap::<String, (String, String)>::new();

    for line in stdin().lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        if instructions.is_empty() {
            instructions.extend(line.as_bytes());
            continue;
        }

        let mut parts = line
            .split(|c| " =(,)".contains(c))
            .filter(|s| !s.is_empty());
        let name = parts.next().context("missing name")?.to_owned();
        let left = parts.next().context("missing left")?.to_owned();
        let right = parts.next().context("missing right")?.to_owned();

        nodes.insert(name, (left, right));
    }

    let poses = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.as_str());

    let solutionss = poses
        .map(|pos| list_solutions(&nodes, &instructions, pos))
        .collect::<Result<Vec<_>, _>>()?;

    eprintln!("solutionss len {}", solutionss.len());
    for solutions in solutionss.iter() {
        eprintln!("solutions len {}", solutions.len());
    }

    let mut best = usize::max_value();

    for solutions in solutionss
        .iter()
        .map(|solutions| solutions.iter())
        .multi_cartesian_product()
    {
        eprintln!("loop");
        let mut lcm_value = 1;
        for solution in solutions {
            lcm_value = lcm(lcm_value, *solution);
        }
        best = min(best, lcm_value);
    }

    println!("{}", best);

    Ok(())
}
