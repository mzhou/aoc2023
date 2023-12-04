use std::error::Error;
use std::io::stdin;
use std::num::TryFromIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Vec::<char>::new();
    let mut width: isize = 0;
    for line in stdin().lines() {
        let line = line?;
        grid.extend(line.chars());
        if width == 0 {
            width = grid.len().try_into()?;
        }
    }
    let height = isize::try_from(grid.len())? / width;

    let mut part_number_total = 0;

    let mut in_part_number = false;
    let mut is_part = false;
    let mut part_number = 0;

    macro_rules! end_part_number {
        () => {
            if in_part_number && is_part {
                part_number_total += part_number;
            }
            in_part_number = false;
            is_part = false;
            part_number = 0;
        };
    }

    for i in 0isize..grid.len().try_into()? {
        let c = grid[usize::try_from(i)?];
        let x = i % width;
        let y = i / width;

        if x == 0 {
            end_part_number!();
        }

        if !c.is_ascii_digit() {
            end_part_number!();
            continue;
        }

        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => {
                continue;
            }
        };

        in_part_number = true;
        part_number = part_number * 10 + digit;

        let check = |x_offset: isize, y_offset: isize| -> Result<bool, TryFromIntError> {
            let target_x = x + x_offset;
            let target_y = y + y_offset;
            if target_x < 0 || target_x >= width {
                return Ok(false);
            }
            if target_y < 0 || target_y >= height {
                return Ok(false);
            }
            let target_i: usize = (target_x + target_y * width).try_into()?;
            Ok(is_symbol(grid[target_i]))
        };

        let near_symbol = check(-1, -1)?
            || check(-1, 0)?
            || check(-1, 1)?
            || check(0, -1)?
            || check(0, 1)?
            || check(1, -1)?
            || check(1, 0)?
            || check(1, 1)?;

        is_part = is_part || near_symbol;
    }

    println!("{}", part_number_total);

    Ok(())
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}
