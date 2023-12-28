use std::cmp::max;
use std::io::stdin;

use anyhow::{Context, Error};

struct Instruction {
    color: u32,
    direction: u8,
    steps: u8,
}

fn flood_fill(grid: &mut [u8], width: usize, pos: usize) {
    if grid[pos] != b'.' {
        return;
    }

    grid[pos] = b'O';

    if pos < grid.len() - width {
        flood_fill(grid, width, pos + width);
    }

    if pos % width != 0 {
        flood_fill(grid, width, pos - 1);
    }

    if pos % width != width - 1 {
        flood_fill(grid, width, pos + 1);
    }

    if pos >= width {
        flood_fill(grid, width, pos - width);
    }
}

fn main() -> Result<(), Error> {
    let mut instructions = Vec::<Instruction>::new();

    for line in stdin().lines() {
        let line = line?;

        let mut parts = line.split(' ');
        let direction = parts.next().context("missing direction")?.as_bytes()[0];
        let steps = parts.next().context("missing steps")?.parse::<u8>()?;
        let color = u32::from_str_radix(&parts.next().context("missing color")?[2..8], 16)?;

        let instruction = Instruction {
            color,
            direction,
            steps,
        };
        instructions.push(instruction);
    }

    let mut current_x = 0usize;
    let mut current_y = 0usize;
    let mut height = 0usize;
    let mut initial_x = 0usize;
    let mut initial_y = 0usize;
    let mut width = 0usize;

    for instruction in instructions.iter() {
        let steps = instruction.steps as usize;
        match instruction.direction {
            b'D' => {
                current_y += steps;
                height = max(current_y + 1, height);
            }
            b'L' => {
                let expand_left = steps.saturating_sub(current_x);
                width += expand_left;
                current_x += expand_left;
                initial_x += expand_left;
                current_x -= steps;
            }
            b'R' => {
                current_x += steps;
                width = max(current_x + 1, width);
            }
            b'U' => {
                let expand_up = steps.saturating_sub(current_y);
                height += expand_up;
                current_y += expand_up;
                initial_y += expand_up;
                current_y -= steps;
            }
            _ => return Err(Error::msg("invalid direction")),
        }
    }

    eprintln!(
        "width {} height {} initial_x {} initial_y {}",
        width, height, initial_x, initial_y
    );

    // add a border so it's easier to flood fill the un-dug space

    height += 2;
    width += 2;
    initial_x += 1;
    initial_y += 1;

    eprintln!(
        "width {} height {} initial_x {} initial_y {}",
        width, height, initial_x, initial_y
    );

    let mut grid = vec![b'.'; width * height];

    let mut current_x = initial_x;
    let mut current_y = initial_y;

    grid[current_y * width + current_x] = b'#';

    for instruction in instructions.iter() {
        let steps = instruction.steps as usize;
        match instruction.direction {
            b'D' => {
                for _ in 0..steps {
                    current_y += 1;
                    grid[current_y * width + current_x] = b'#';
                }
            }
            b'L' => {
                for _ in 0..steps {
                    current_x -= 1;
                    grid[current_y * width + current_x] = b'#';
                }
            }
            b'R' => {
                for _ in 0..steps {
                    current_x += 1;
                    grid[current_y * width + current_x] = b'#';
                }
            }
            b'U' => {
                for _ in 0..steps {
                    current_y -= 1;
                    grid[current_y * width + current_x] = b'#';
                }
            }
            _ => return Err(Error::msg("invalid direction")),
        }
    }

    flood_fill(&mut grid, width, 0);

    for y in 0..height {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&grid[y * width..(y + 1) * width])
        );
    }

    let dug_or_inside_count = grid
        .iter()
        .cloned()
        .filter(|t| *t == b'#' || *t == b'.')
        .count();

    println!("{}", dug_or_inside_count);

    Ok(())
}
