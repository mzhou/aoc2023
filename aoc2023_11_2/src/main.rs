use std::cmp::{max, min};
use std::io::stdin;

use anyhow::{Context, Error};

fn between(lower: usize, upper: usize, operand: usize) -> bool {
    ((min(lower, upper) + 1)..max(lower, upper)).contains(&operand)
}

fn main() -> Result<(), Error> {
    let mut grid = Vec::<u8>::new();
    let mut width = 0;

    for line in stdin().lines() {
        let line = line?;
        let line_bytes = line.as_bytes();
        if width == 0 {
            width = line_bytes.len();
        } else {
            if width != line_bytes.len() {
                return Err(Error::msg("inconsistent width"));
            }
        }
        grid.extend(line_bytes);
    }

    let height = grid.len() / width;

    let mut expand_xs = Vec::<usize>::new();
    let mut expand_ys = Vec::<usize>::new();

    for x in 0..width {
        let mut all_empty = true;
        for y in 0..height {
            if grid[y * width + x] != b'.' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            expand_xs.push(x);
        }
    }

    for y in 0..height {
        let mut all_empty = true;
        for x in 0..width {
            if grid[y * width + x] != b'.' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            expand_ys.push(y);
        }
    }

    let expanded_height = height + expand_ys.len();
    let expanded_width = width + expand_xs.len();

    let mut expanded_grid = Vec::<u8>::new();

    for y in 0..height {
        if expand_ys.contains(&y) {
            expanded_grid.extend([b'+'].repeat(width + expand_xs.len()));
        }
        for x in 0..width {
            if expand_xs.contains(&x) {
                expanded_grid.push(b'+');
            }
            expanded_grid.push(grid[y * width + x]);
        }
    }

    for y in 0..expanded_height {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&expanded_grid[y * expanded_width..(y + 1) * expanded_width])
        );
    }

    let mut galaxy_poses = Vec::<(usize, usize)>::new();

    let mut extra_y = 0;

    for y in 0..expanded_height {
        if expanded_grid[y * expanded_width + 0] == b'+' {
            extra_y += 999998;
        }
        let mut extra_x = 0;
        for x in 0..expanded_width {
            if expanded_grid[x] == b'+' {
                extra_x += 999998;
            }
            if expanded_grid[y * expanded_width + x] == b'#' {
                galaxy_poses.push((x + extra_x, y + extra_y));
            }
        }
    }

    /*
    let expanded_xs = expand_xs
        .iter()
        .enumerate()
        .map(|(i, v)| i + v)
        .collect::<Vec<_>>();
    let expanded_ys = expand_xs
        .iter()
        .enumerate()
        .map(|(i, v)| i + v)
        .collect::<Vec<_>>();
    */

    let mut sum_distances = 0;

    for i in 0..galaxy_poses.len() {
        for j in i..galaxy_poses.len() {
            let (pos_i_x, pos_i_y) = galaxy_poses[i];
            let (pos_j_x, pos_j_y) = galaxy_poses[j];
            let distance = pos_i_x.abs_diff(pos_j_x) + pos_i_y.abs_diff(pos_j_y);
            /*
            for expand_x in expanded_xs.iter() {
                if between(pos_i_x, pos_j_x, *expand_x) {
                    distance += 8;
                }
            }
            for expand_y in expanded_ys.iter() {
                if between(pos_i_y, pos_j_y, *expand_y) {
                    distance += 98;
                }
            }
            */
            eprintln!(
                "{} ({}, {}) {} ({}, {}) {}",
                i, pos_i_x, pos_i_y, j, pos_j_x, pos_j_y, distance
            );
            sum_distances += distance;
        }
    }

    println!("{}", sum_distances);

    Ok(())
}
