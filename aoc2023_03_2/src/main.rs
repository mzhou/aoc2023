use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::stdin;
use std::num::TryFromIntError;

struct PotentialGear<const MAX_PARTS: usize = 2> {
    num_parts: usize,
    part_numbers: [u32; MAX_PARTS],
}

impl<const MAX_PARTS: usize> PotentialGear<MAX_PARTS> {
    fn default() -> Self {
        Self {
            num_parts: 0,
            part_numbers: [0; MAX_PARTS],
        }
    }

    fn process_part(&mut self, part_number: u32) {
        if self.num_parts < MAX_PARTS {
            self.part_numbers[self.num_parts] = part_number;
        }
        self.num_parts += 1;
    }
}

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

    let mut potential_gears = HashMap::<usize, PotentialGear>::new();

    let mut in_part_number = false;
    let mut part_gears = HashSet::<usize>::new();
    let mut part_number = 0;

    macro_rules! end_part_number {
        () => {
            if in_part_number {
                for gear_i in part_gears.iter() {
                    let potential_gear = potential_gears
                        .entry(*gear_i)
                        .or_insert_with(PotentialGear::default);
                    potential_gear.process_part(part_number);
                }
            }
            in_part_number = false;
            part_gears.clear();
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

        let mut check = |x_offset: isize, y_offset: isize| -> Result<(), TryFromIntError> {
            let target_x = x + x_offset;
            let target_y = y + y_offset;
            if target_x < 0 || target_x >= width {
                return Ok(());
            }
            if target_y < 0 || target_y >= height {
                return Ok(());
            }
            let target_i: usize = (target_x + target_y * width).try_into()?;
            if grid[target_i] == '*' {
                part_gears.insert(target_i);
            }
            Ok(())
        };

        check(-1, -1)?;
        check(-1, 0)?;
        check(-1, 1)?;
        check(0, -1)?;
        check(0, 1)?;
        check(1, -1)?;
        check(1, 0)?;
        check(1, 1)?;
    }

    let mut gear_ratio_total = 0;

    for potential_gear in potential_gears.values() {
        if potential_gear.num_parts == 2 {
            gear_ratio_total += potential_gear.part_numbers[0] * potential_gear.part_numbers[1];
        }
    }

    println!("{}", gear_ratio_total);

    Ok(())
}
