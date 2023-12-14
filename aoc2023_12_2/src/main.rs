use std::io::stdin;

use anyhow::{Context, Error};

fn is_combo_valid(current: &[u8], groups: &[usize]) -> bool {
    /*
    eprintln!(
        "is_combo_valid({}, {:?})",
        String::from_utf8_lossy(current),
        groups
    );
    */

    let mut run_length = 0;
    let mut group_i = 0;
    for (i, b) in current.iter().enumerate() {
        if *b == b'#' {
            run_length += 1;
            continue;
        }
        if run_length != 0 {
            if group_i >= groups.len() || run_length != groups[group_i] {
                //eprintln!("{} {}", i, false);
                return false;
            }
            run_length = 0;
            group_i += 1;
        }
    }
    if run_length != 0 {
        if group_i >= groups.len() || run_length != groups[group_i] {
            //eprintln!("end {}", false);
            return false;
        }
        run_length = 0;
        group_i += 1;
    }
    let ret = group_i == groups.len();
    //eprintln!("length {}", ret);
    ret
}

fn main() -> Result<(), Error> {
    let mut total_combo_count = 0;

    for line in stdin().lines() {
        let line = line?;

        let (status, groups_str) = line.split_once(' ').context("missing space")?;
        let groups = groups_str
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let status = status.as_bytes();

        let unfolded_status = unfold_status(status);
        let unfolded_groups = unfold_groups(&groups);

        let mut unknown_poses = Vec::<usize>::new();
        for (i, c) in unfolded_status.iter().enumerate() {
            if *c == b'?' {
                unknown_poses.push(i);
            }
        }

        let mut valid_combo_count = 0;
        let mut current = unfolded_status
            .iter()
            .map(|b| match b {
                b'?' => b'.',
                x => *x,
            })
            .collect::<Vec<_>>();

        loop {
            if is_combo_valid(&current, &unfolded_groups) {
                valid_combo_count += 1;
            }

            // increment
            let mut incremented = false;
            for unknown_pos in unknown_poses.iter() {
                let unknown_pos = *unknown_pos;
                if current[unknown_pos] == b'.' {
                    current[unknown_pos] = b'#';
                    incremented = true;
                    break;
                }
                current[unknown_pos] = b'.';
            }
            if !incremented {
                break;
            }
        }

        total_combo_count += valid_combo_count;
    }

    println!("{}", total_combo_count);

    Ok(())
}

fn unfold_groups(groups: &[usize]) -> Vec<usize> {
    groups.repeat(5).into_iter().collect()
}

fn unfold_status(status: &[u8]) -> Vec<u8> {
    [
        status,
        &[b'?'],
        status,
        &[b'?'],
        status,
        &[b'?'],
        status,
        &[b'?'],
        status,
    ]
    .into_iter()
    .flatten()
    .map(|b| *b)
    .collect()
}
