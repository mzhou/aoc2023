use std::collections::HashMap;
use std::io::stdin;

use anyhow::{Context, Error};

fn is_all_done(poses: &[&str]) -> bool {
    poses
        .iter()
        .fold(true, |ret, value| ret && value.ends_with('Z'))
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

    let mut steps = 0;

    let mut poses = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.as_str())
        .collect::<Vec<_>>();

    while !is_all_done(&poses) {
        for instruction in instructions.iter() {
            let instruction = *instruction;
            if is_all_done(&poses) {
                break;
            }
            for pos in poses.iter_mut() {
                let node = nodes.get(*pos).context("invalid node")?;
                *pos = match instruction {
                    b'L' => node.0.as_str(),
                    b'R' => node.1.as_str(),
                    _ => panic!("invalid instruction"),
                };
            }
            steps += 1;
        }
    }

    println!("{}", steps);

    Ok(())
}
