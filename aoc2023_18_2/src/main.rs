use std::cmp::{max, min};
use std::io::stdin;

use anyhow::{Context, Error};

struct FixedInstruction {
    direction: u8,
    steps: u32,
}

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

fn unrolled_flood_fill(grid: &mut [u8], width: usize, pos: usize) {
    let mut candidates = Vec::<usize>::new();

    candidates.push(pos);

    while let Some(pos) = candidates.pop() {
        if grid[pos] != b'.' {
            continue;
        }

        grid[pos] = b'O';

        if pos < grid.len() - width {
            candidates.push(pos + width);
        }

        if pos % width != 0 {
            candidates.push(pos - 1);
        }

        if pos % width != width - 1 {
            candidates.push(pos + 1);
        }

        if pos >= width {
            candidates.push(pos - width);
        }
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

    let instructions = instructions
        .into_iter()
        .map(|i| {
            let direction = match i.color & 0xf {
                0 => Ok(b'R'),
                1 => Ok(b'D'),
                2 => Ok(b'L'),
                3 => Ok(b'U'),
                _ => Err(Error::msg("invalid direction")),
            }?;
            let steps = i.color >> 4;
            Ok::<_, Error>(FixedInstruction { direction, steps })
        })
        .collect::<Result<Vec<_>, _>>()?;

    for instruction in instructions.iter() {
        eprintln!("{} {}", instruction.direction as char, instruction.steps);
    }

    let mut current_x = 0u64;
    let mut current_y = 0u64;
    let mut height = 0u64;
    let mut initial_x = 0u64;
    let mut initial_y = 0u64;
    let mut width = 0u64;

    for instruction in instructions.iter() {
        let steps = instruction.steps as u64;
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

    let mut current_x = initial_x;
    let mut current_y = initial_y;

    let mut boundary_xs = Vec::<u64>::new();
    let mut boundary_ys = Vec::<u64>::new();

    vec_set_insert(&mut boundary_xs, current_x);
    vec_set_insert(&mut boundary_ys, current_y);

    for instruction in instructions.iter() {
        let steps = instruction.steps as u64;
        match instruction.direction {
            b'D' => {
                current_y += steps;
                vec_set_insert(&mut boundary_ys, current_y);
            }
            b'L' => {
                current_x -= steps;
                vec_set_insert(&mut boundary_xs, current_x);
            }
            b'R' => {
                current_x += steps;
                vec_set_insert(&mut boundary_xs, current_x);
            }
            b'U' => {
                current_y -= steps;
                vec_set_insert(&mut boundary_ys, current_y);
            }
            _ => return Err(Error::msg("invalid direction")),
        }
    }

    eprintln!(
        "boundary_xs {:?} boundary_ys {:?}",
        boundary_xs, boundary_ys
    );

    let slots_x = boundary_xs.len() * 2 + 1;
    let slots_y = boundary_ys.len() * 2 + 1;

    let mut grid = vec![b'.'; slots_y * slots_x];

    let mut current_x = initial_x;
    let mut current_y = initial_y;

    let Ok(i_x) = boundary_xs.binary_search(&current_x) else {
        return Err(Error::msg("slot x not found"));
    };
    let Ok(i_y) = boundary_ys.binary_search(&current_y) else {
        return Err(Error::msg("slot y not found"));
    };

    grid[(2 * i_y + 1) * slots_x + (2 * i_x + 1)] = b'#';

    for instruction in instructions.iter() {
        let steps = instruction.steps as u64;

        let mut next_x = current_x;
        let mut next_y = current_y;
        match instruction.direction {
            b'D' => {
                next_y += steps;
            }
            b'L' => {
                next_x -= steps;
            }
            b'R' => {
                next_x += steps;
            }
            b'U' => {
                next_y -= steps;
            }
            _ => return Err(Error::msg("invalid direction")),
        }

        let Ok(i_start_x) = boundary_xs.binary_search(&current_x) else {
            return Err(Error::msg("start x not found"));
        };
        let Ok(i_start_y) = boundary_ys.binary_search(&current_y) else {
            return Err(Error::msg("start y not found"));
        };

        let Ok(i_last_x) = boundary_xs.binary_search(&next_x) else {
            return Err(Error::msg("start x not found"));
        };
        let Ok(i_last_y) = boundary_ys.binary_search(&next_y) else {
            return Err(Error::msg("start y not found"));
        };

        let start_slot_x = 2 * i_start_x + 1;
        let start_slot_y = 2 * i_start_y + 1;
        let last_slot_x = 2 * i_last_x + 1;
        let last_slot_y = 2 * i_last_y + 1;

        for slot_y in min(start_slot_y, last_slot_y)..max(start_slot_y, last_slot_y) + 1 {
            for slot_x in min(start_slot_x, last_slot_x)..max(start_slot_x, last_slot_x) + 1 {
                grid[slot_y * slots_x + slot_x] = b'#';
            }
        }

        current_x = next_x;
        current_y = next_y;
    }

    eprintln!("before fill");
    for y in 0..slots_y {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&grid[y * slots_x..(y + 1) * slots_x])
        );
    }

    unrolled_flood_fill(&mut grid, slots_x, 0);

    eprintln!("after fill");
    for y in 0..slots_y {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&grid[y * slots_x..(y + 1) * slots_x])
        );
    }

    let mut dug_or_surrounded = 0u64;

    for pos in 0..grid.len() {
        if grid[pos] == b'O' {
            continue;
        }

        let slot_x = pos % slots_x;
        let slot_y = pos / slots_x;

        let height = if slot_y % 2 == 1 {
            1
        } else {
            boundary_ys[slot_y / 2] - boundary_ys[slot_y / 2 - 1] - 1
        };
        let width = if slot_x % 2 == 1 {
            1
        } else {
            boundary_xs[slot_x / 2] - boundary_xs[slot_x / 2 - 1] - 1
        };

        let area = height * width;

        eprintln!(
            "slot_x {} slot_y {} height {} width {} area {} type {}",
            slot_x, slot_y, height, width, area, grid[pos] as char
        );

        dug_or_surrounded += area;
    }

    println!("{}", dug_or_surrounded);

    Ok(())
}

fn vec_set_insert<T>(vec: &mut Vec<T>, value: T)
where
    T: Ord,
{
    if let Err(pos) = vec.binary_search(&value) {
        vec.insert(pos, value);
    }
}
