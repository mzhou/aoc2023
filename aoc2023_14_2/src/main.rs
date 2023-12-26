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

    for _ in 0..1000000000 {
        cycle(&mut grid, width);
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
