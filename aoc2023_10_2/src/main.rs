use std::io::stdin;

use anyhow::{Context, Error};

fn flood_fill(tiles: &[u8], width: usize, gaps: &mut [u8]) {
    flood_fill_recursive(tiles, width, gaps, 0);
}

fn flood_fill_recursive(tiles: &[u8], width: usize, gaps: &mut [u8], pos: usize) {
    eprintln!("flood_fill_recursive({})", pos);
    if gaps[pos] != b'.' {
        return;
    }
    gaps[pos] = b'O';
    let gaps_width = width + 1;

    // special cases for when we're on the edge, no pipe will ever block us from moving to another
    // edge gap

    // left/right edge
    if pos % gaps_width == 0 || pos % gaps_width == gaps_width - 1 {
        if pos / gaps_width > gaps_width {
            eprintln!("flood_fill_recursive({}) edge up", pos);
            flood_fill_recursive(tiles, width, gaps, pos - gaps_width);
        }
        if pos < gaps.len() - gaps_width {
            eprintln!("flood_fill_recursive({}) edge down", pos);
            flood_fill_recursive(tiles, width, gaps, pos + gaps_width);
        }
    }

    // left, will be blocked if the pipe we're on the top right corner of has a vertical at the top
    if pos % gaps_width != 0
        && pos < gaps.len() - gaps_width
        && !b"|LJ".contains(&tiles[(pos / gaps_width) * width + pos % gaps_width - 1])
    {
        eprintln!("flood_fill_recursive({}) left", pos);
        flood_fill_recursive(tiles, width, gaps, pos - 1);
    }

    // up, anything with horizontal at the left will block (tile we're on the bottom left
    // corner of)
    if pos >= gaps_width
        && pos % gaps_width < gaps_width - 1
        && !b"-J7".contains(&tiles[(pos / gaps_width - 1) * width + pos % gaps_width])
    {
        eprintln!("flood_fill_recursive({}) up", pos);
        flood_fill_recursive(tiles, width, gaps, pos - gaps_width);
    }

    // right, anything with vertical at the top will block (tile we're on the top left
    // corner of)
    if pos % gaps_width < gaps_width - 1
        && pos < gaps.len() - gaps_width
        && !b"|LJ".contains(&tiles[(pos / gaps_width) * width + pos % gaps_width])
    {
        eprintln!("flood_fill_recursive({}) right", pos);
        flood_fill_recursive(tiles, width, gaps, pos + 1);
    }

    // down, anything above with horizontal at the left will block (tile we're on the top left
    // corner of)
    if pos < gaps.len() - gaps_width
        && pos % gaps_width < gaps_width - 1
        && !b"-J7".contains(&tiles[(pos / gaps_width) * width + pos % gaps_width])
    {
        eprintln!("flood_fill_recursive({}) down", pos);
        flood_fill_recursive(tiles, width, gaps, pos + gaps_width);
    }
}

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

    let mut distances = vec![0u64; tiles.len()];

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

    let mut start_tile = b'#';

    'outer: for first_direction in 0usize..4 {
        eprintln!("first_direction {}", first_direction);

        let mut pos = calc_pos(start_pos, first_direction);
        let mut distance = 1;
        let mut last_direction = first_direction;

        while let Some(some_pos) = pos {
            distances[some_pos] = distance;
            distance += 1;

            let tile = tiles[some_pos];
            eprintln!("tile {} distance {}", tile as char, distance);
            let direction = match tile {
                b'S' => {
                    start_tile = match (last_direction, first_direction) {
                        (0, 0) => b'-',
                        (0, 1) => b'L',
                        (0, 2) => panic!("inconsistent direction"),
                        (0, 3) => b'F',
                        (1, 0) => b'7',
                        (1, 1) => b'|',
                        (1, 2) => b'F',
                        (1, 3) => panic!("inconsistent direction"),
                        (2, 0) => panic!("inconsistent direction"),
                        (2, 1) => b'J',
                        (2, 2) => b'-',
                        (2, 3) => b'7',
                        (3, 0) => b'J',
                        (3, 1) => panic!("inconsistent direction"),
                        (3, 2) => b'L',
                        (3, 3) => b'|',
                        _ => panic!("invalid direction combo"),
                    };
                    break 'outer;
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

    // copy a map with only the loop
    let mut loop_tiles = vec![b'.'; tiles.len()];
    for i in 0..tiles.len() {
        if distances[i] != 0 {
            loop_tiles[i] = tiles[i];
        }
    }
    loop_tiles[start_pos] = start_tile;

    for y in 0..height {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&loop_tiles[y * width..(y + 1) * width])
        );
    }

    // make a map of "gaps" between the pipes. gap (0, 0) is on the top left corner of pipe (0, 0)
    // and (width, height) is on the bottom right corner of pipe tile (width - 1, height - 1)
    // notation (x, y) with pos = (width + 1) * y + x

    let gaps_width = width + 1;
    let gaps_height = height + 1;

    let mut gaps = vec![b'.'; gaps_width * gaps_height];

    // flood fill from (0, 0)
    flood_fill(&loop_tiles, width, &mut gaps);

    for y in 0..gaps_height {
        eprintln!(
            "{}",
            String::from_utf8_lossy(&gaps[y * gaps_width..(y + 1) * gaps_width])
        );
    }

    Ok(())
}
