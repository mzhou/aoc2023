use std::cmp::max;
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

    let mut best_energized = 0usize;

    for pos in 0..grid.len() {
        // top
        if pos / width == 0 {
            let mut direction_info = vec![0u8; grid.len()];
            shoot_beam(&grid, width, &mut direction_info, pos, 2);
            let energized = direction_info.iter().cloned().filter(|d| *d != 0u8).count();
            best_energized = max(best_energized, energized);
        }
        // left
        if pos % width == 0 {
            let mut direction_info = vec![0u8; grid.len()];
            shoot_beam(&grid, width, &mut direction_info, pos, 1);
            let energized = direction_info.iter().cloned().filter(|d| *d != 0u8).count();
            best_energized = max(best_energized, energized);
        }
        // right
        if pos % width == width - 1 {
            let mut direction_info = vec![0u8; grid.len()];
            shoot_beam(&grid, width, &mut direction_info, pos, 3);
            let energized = direction_info.iter().cloned().filter(|d| *d != 0u8).count();
            best_energized = max(best_energized, energized);
        }
        // bottom
        if pos / width == grid.len() / width - 1 {
            let mut direction_info = vec![0u8; grid.len()];
            shoot_beam(&grid, width, &mut direction_info, pos, 3);
            let energized = direction_info.iter().cloned().filter(|d| *d != 0u8).count();
            best_energized = max(best_energized, energized);
        }
    }

    println!("{}", best_energized);

    Ok(())
}

// 1 = right, 2 = down, 3 = left, 3 = up
fn shoot_beam(grid: &[u8], width: usize, direction_info: &mut [u8], pos: usize, direction: u8) {
    let direction_mask = 1 << direction;

    if direction_info[pos] & direction_mask != 0 {
        return;
    }

    direction_info[pos] |= direction_mask;

    let t = grid[pos];

    let mut proceed = |new_direction| {
        if new_direction == 1 && pos % width < width - 1 {
            let new_pos = pos + 1;
            shoot_beam(grid, width, direction_info, new_pos, new_direction);
        }
        if new_direction == 2 && pos < grid.len() - width {
            let new_pos = pos + width;
            shoot_beam(grid, width, direction_info, new_pos, new_direction);
        }
        if new_direction == 3 && pos % width >= 1 {
            let new_pos = pos - 1;
            shoot_beam(grid, width, direction_info, new_pos, new_direction);
        }
        if new_direction == 4 && pos >= width {
            let new_pos = pos - width;
            shoot_beam(grid, width, direction_info, new_pos, new_direction);
        }
    };

    // straight
    if t == b'.'
        || (t == b'-' && (direction_mask & 0b01010) != 0)
        || (t == b'|' && (direction_mask & 0b10100) != 0)
    {
        proceed(direction);
    }

    // mirrored
    if t == b'/' {
        let new_direction = match direction {
            1 => 4,
            2 => 3,
            3 => 2,
            4 => 1,
            _ => panic!("invalid direction"),
        };
        proceed(new_direction);
    }

    if t == b'\\' {
        let new_direction = match direction {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("invalid direction"),
        };
        proceed(new_direction);
    }

    // split
    if t == b'|' && (direction_mask & 0b01010) != 0 {
        proceed(2);
        proceed(4);
    }

    if t == b'-' && (direction_mask & 0b10100) != 0 {
        proceed(1);
        proceed(3);
    }
}
