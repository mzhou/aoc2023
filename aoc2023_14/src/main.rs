use std::error::Error;
use std::io::stdin;

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

    while roll_north(&mut grid, width) {}

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
