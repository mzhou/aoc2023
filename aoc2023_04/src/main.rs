use std::collections::HashSet;
use std::io::stdin;

use anyhow::{Context, Error};

fn main() -> Result<(), Error> {
    let mut total_points = 0;

    for line in stdin().lines() {
        let line = line?;
        let start_i = line.find(":").context("colon not found")? + 1;
        let Some((winning, have)) = &line[start_i..].split_once('|') else {
            return Err(Error::msg(" | not found in line"));
        };
        let winning_set = winning
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect::<HashSet<_>>();
        let mut points = 0;
        for number in have.split(' ') {
            if number.is_empty() {
                continue;
            }
            if winning_set.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total_points += points;
    }

    println!("{}", total_points);

    Ok(())
}
