use std::io::stdin;

use anyhow::{Context, Error};

fn main() -> Result<(), Error> {
    let mut tiles = Vec::<u8>::new();
    let mut width = 0;

    for line in stdin().lines() {
        let line = line?;
        if width == 0 {
            width = line.as_bytes().len();
        } else {
            if line.len() != width {
                return Err(Error::msg("inconsistent width"));
            }
        }
        tiles.extend(line.as_bytes());
    }

    let height = tiles.len() / width;

    let start_pos = tiles
        .iter()
        .position(|t| *t == b'S')
        .context("start tile missing")?;

    eprintln!("start_pos {}", start_pos);

    // https://stackoverflow.com/questions/27393166/how-do-i-initialize-an-array-of-vectors
    //let mut distances = [
    //    vec![0u64; tiles.len()],
    //    vec![0u64; tiles.len()],
    //    vec![0u64; tiles.len()],
    //    vec![0u64; tiles.len()],
    //];
    //let mut proper_loop = [false; 4];

    // 0 -> left, 1 -> up, 2 -> right, 3 -> down
    let calc_pos = |pos: usize, direction: usize| -> Option<usize> {
        match direction {
            0 => {
                if pos % width == 0 {
                    None
                } else {
                    Some(pos - 1)
                }
            }
            1 => {
                if pos < width {
                    None
                } else {
                    Some(pos - width)
                }
            }
            2 => {
                if pos % width == width - 1 {
                    None
                } else {
                    Some(pos + 1)
                }
            }
            3 => {
                if pos % width == height {
                    None
                } else {
                    Some(pos + width)
                }
            }
            _ => panic!("invalid case"),
        }
    };

    for first_direction in 0usize..4 {
        eprintln!("first_direction {}", first_direction);

        let mut pos = calc_pos(start_pos, first_direction);
        let mut distance = 1;
        let mut last_direction = first_direction;

        while let Some(some_pos) = pos {
            //distances[first_direction][some_pos] = distance;
            distance += 1;

            let tile = tiles[some_pos];
            eprintln!("tile {} distance {}", tile as char, distance);
            let direction = match tile {
                b'S' => {
                    //proper_loop[first_direction] = true;
                    println!("{}", distance / 2);
                    return Ok(());
                }
                b'|' => match last_direction {
                    1 => 1,
                    3 => 3,
                    _ => panic!("inconsistent direction"),
                },
                b'-' => match last_direction {
                    0 => 0,
                    2 => 2,
                    _ => panic!("inconsistent direction"),
                },
                b'L' => match last_direction {
                    0 => 1,
                    3 => 2,
                    _ => panic!("inconsistent direction"),
                },
                b'J' => match last_direction {
                    2 => 1,
                    3 => 0,
                    _ => panic!("inconsistent direction"),
                },
                b'7' => match last_direction {
                    2 => 3,
                    1 => 0,
                    _ => panic!("inconsistent direction"),
                },
                b'F' => match last_direction {
                    1 => 2,
                    0 => 3,
                    _ => panic!("inconsistent direction"),
                },
                b'.' => {
                    break;
                }
                _ => panic!("invalid tile"),
            };
            last_direction = direction;

            pos = calc_pos(some_pos, direction);
        }
    }

    Ok(())
}
