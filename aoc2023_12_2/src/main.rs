use std::collections::HashMap;
use std::io::stdin;

use anyhow::{Context, Error};

static DEBUG: bool = false;

type Cache = HashMap<State, usize>;

#[derive(Eq, Hash, PartialEq)]
struct State {
    first: Option<u8>,
    current: Vec<u8>,
    active_group: Option<usize>,
    groups: Vec<usize>,
}

fn count_combos(current: &[u8], groups: &[usize]) -> usize {
    let mut cache = Cache::new();
    count_combos_recursive(
        &mut cache,
        0,
        current.first().cloned(),
        exclude_first(current),
        None,
        groups,
    )
}

fn count_combos_recursive(
    cache: &mut Cache,
    depth: usize,
    first: Option<u8>,
    current: &[u8],
    active_group: Option<usize>,
    groups: &[usize],
) -> usize {
    if DEBUG {
        eprintln!(
            "count_combos_recursive({}, {}, {} {}, {:?}, {} {:?})",
            depth,
            first.map(|f| f as char).unwrap_or('N'),
            current.len(),
            String::from_utf8_lossy(current),
            active_group,
            groups.len(),
            groups
        );
    }

    let state = State {
        first,
        current: current.iter().cloned().collect(),
        active_group,
        groups: groups.iter().cloned().collect(),
    };

    if let Some(ret) = cache.get(&state) {
        return *ret;
    }

    let remaining_damaged =
        active_group.unwrap_or_default() + groups.iter().map(|g| *g as usize).sum::<usize>();

    let first = match first {
        None => {
            //eprintln!(
            //    "first None active_group {:?} groups {:?}",
            //    active_group, remaining_damaged
            //);
            let ret = if remaining_damaged == 0 { 1 } else { 0 };
            if DEBUG {
                eprintln!("{} return {}", depth, ret);
            }
            cache.insert(state, ret);
            return ret;
        }
        Some(f) => f,
    };

    let max_remaining_damaged = max_damaged(current);

    // early exit heuristics

    if max_remaining_damaged + 2 < remaining_damaged {
        return 0;
    }

    if remaining_damaged + groups.len() > current.len() + 2 {
        return 0;
    }

    match first {
        b'.' => match active_group {
            None => {
                // just advance current
                let ret = count_combos_recursive(
                    cache,
                    depth + 1,
                    current.first().cloned(),
                    exclude_first(current),
                    active_group,
                    groups,
                );
                cache.insert(state, ret);
                return ret;
            }
            Some(g) => {
                // ending a group
                if g != 0 {
                    // can't if group isn't exhausted
                    //eprintln!(
                    //    "group ended first {} current {} active_group {} groups {:?}",
                    //    first as char,
                    //    String::from_utf8_lossy(current),
                    //    g,
                    //    groups
                    //);
                    let ret = 0;
                    cache.insert(state, ret);
                    return ret;
                }
                // advance current, end group
                let ret = count_combos_recursive(
                    cache,
                    depth + 1,
                    current.first().cloned(),
                    exclude_first(current),
                    None,
                    groups,
                );
                cache.insert(state, ret);
                return ret;
            }
        },
        b'#' => match active_group {
            None => {
                // start group
                if groups.is_empty() {
                    let ret = 0;
                    cache.insert(state, ret);
                    return ret;
                }
                let ret = count_combos_recursive(
                    cache,
                    depth + 1,
                    current.first().cloned(),
                    exclude_first(current),
                    Some(groups[0] - 1),
                    exclude_first(groups),
                );
                cache.insert(state, ret);
                return ret;
            }
            Some(g) => {
                // continue existing group
                if g == 0 {
                    // can't if current group has run out
                    let ret = 0;
                    cache.insert(state, ret);
                    return ret;
                }
                // reduce current group by 1
                let ret = count_combos_recursive(
                    cache,
                    depth + 1,
                    current.first().cloned(),
                    exclude_first(current),
                    Some(g - 1),
                    groups,
                );
                cache.insert(state, ret);
                return ret;
            }
        },
        b'?' => {
            // sum up both alternatives for first, with some pruning
            let mut count = 0;
            if max_remaining_damaged >= remaining_damaged
                && (remaining_damaged + groups.len().saturating_sub(1) <= current.len())
            {
                // don't bother trying . if we can't satisfy the groups even making
                // all remaining ones #
                count += count_combos_recursive(
                    cache,
                    depth + 1,
                    Some(b'.'),
                    current,
                    active_group,
                    groups,
                );
            }
            if remaining_damaged != 0 {
                count += count_combos_recursive(
                    cache,
                    depth + 1,
                    Some(b'#'),
                    current,
                    active_group,
                    groups,
                );
            }
            cache.insert(state, count);
            return count;
        }
        _ => panic!("invalid tile"),
    }
}

fn exclude_first<T>(s: &[T]) -> &[T] {
    if s.is_empty() {
        &[]
    } else {
        &s[1..]
    }
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

        eprintln!(
            "processing {} {:?}",
            String::from_utf8_lossy(&unfolded_status),
            unfolded_groups
        );

        let valid_combo_count = count_combos(&unfolded_status, &unfolded_groups);

        /*
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
        */

        total_combo_count += valid_combo_count;
    }

    println!("{}", total_combo_count);

    Ok(())
}

fn max_damaged(statuses: &[u8]) -> usize {
    statuses
        .iter()
        .filter(|s| **s == b'#' || **s == b'?')
        .count()
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
