use std::collections::HashMap;
use std::error::Error;
use std::io::stdin;

fn cycle(grid: &mut [u8], width: usize) {
    while roll_north(grid, width) {}
    while roll_west(grid, width) {}
    while roll_south(grid, width) {}
    while roll_east(grid, width) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Vec::<u8>::new();
    let mut width = 0;

    for line in stdin().lines() {
        let line = line?;
        let line_bytes = line.as_bytes();

        if width == 0 {
            width = line_bytes.len();
        } else {
            if width != line_bytes.len() {
                panic!("width mismatch");
            }
        }

        grid.extend(line_bytes.iter());
    }

    let mut seen = HashMap::<Vec<u8>, usize>::new();

    let mut i = 0;
    while i != 1000000000 {
        if let Some(when_seen) = seen.get(&grid) {
            let cycle_length = i - when_seen;
            eprintln!("{} seen at {}", i, when_seen);
            let addable_times = (1000000000 - i - 1) / cycle_length;
            i += addable_times * cycle_length;
        }

        seen.insert(grid.clone(), i);
        cycle(&mut grid, width);
        i += 1;
    }

    let mut total_load = 0;

    let height = grid.len() / width;

    for i in 0..grid.len() {
        if grid[i] != b'O' {
            continue;
        }
        let load = height - i / width;
        total_load += load;
    }

    println!("{}", total_load);

    Ok(())
}

fn roll_east(grid: &mut [u8], width: usize) -> bool {
    let mut any_moved = false;

    for i in 0..grid.len() {
        if grid[i] != b'.' {
            continue;
        }
        if i % width == 0 {
            continue;
        }
        if grid[i - 1] != b'O' {
            continue;
        }
        grid[i] = b'O';
        grid[i - 1] = b'.';
        any_moved = true;
    }

    any_moved
}

fn roll_north(grid: &mut [u8], width: usize) -> bool {
    let mut any_moved = false;

    for i in 0..grid.len() {
        if grid[i] != b'.' {
            continue;
        }
        if i >= grid.len() - width {
            continue;
        }
        if grid[i + width] != b'O' {
            continue;
        }
        grid[i] = b'O';
        grid[i + width] = b'.';
        any_moved = true;
    }

    any_moved
}

fn roll_south(grid: &mut [u8], width: usize) -> bool {
    let mut any_moved = false;

    for i in 0..grid.len() {
        if grid[i] != b'.' {
            continue;
        }
        if i < width {
            continue;
        }
        if grid[i - width] != b'O' {
            continue;
        }
        grid[i] = b'O';
        grid[i - width] = b'.';
        any_moved = true;
    }

    any_moved
}

fn roll_west(grid: &mut [u8], width: usize) -> bool {
    let mut any_moved = false;

    for i in 0..grid.len() {
        if grid[i] != b'.' {
            continue;
        }
        if i % width == 0 {
            continue;
        }
        if grid[i - 1] != b'O' {
            continue;
        }
        grid[i] = b'O';
        grid[i - 1] = b'.';
        any_moved = true;
    }

    any_moved
}
