use std::io::stdin;

use anyhow::{Context, Error};

struct Instruction {
    color: u32,
    direction: u8,
    steps: u8,
}

fn main() -> Result<(), Error> {
    let initial_x = 0usize;
    let initial_y = 0usize;

    for line in stdin().lines() {
        let line = line?;
        let line_bytes = line.as_bytes();

        let mut parts = line_bytes.split(|b| *b == b' ');
        let direction = parts.next().context("missing direction")?;
    }

    Ok(())
}
