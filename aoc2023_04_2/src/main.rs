use std::collections::{HashSet, VecDeque};
use std::io::stdin;

use anyhow::{Context, Error};

fn main() -> Result<(), Error> {
    let mut total_cards = 0;

    let mut copies = VecDeque::<i32>::new();

    for line in stdin().lines() {
        total_cards += 1;
        if let Some(this_copies) = copies.pop_front() {
            total_cards += this_copies;
        }

        let line = line?;
        let start_i = line.find(":").context("colon not found")? + 1;
        let Some((winning, have)) = &line[start_i..].split_once('|') else {
            return Err(Error::msg(" | not found in line"));
        };
        let winning_set = winning
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect::<HashSet<_>>();
        let mut matches = 0;
        for number in have.split(' ') {
            if number.is_empty() {
                continue;
            }
            if winning_set.contains(number) {
                matches += 1;
            }
        }
        for i in 0..matches {
            if i < copies.len() {
                copies[i] += 1;
            } else {
                copies.push_back(1);
            }
        }
    }

    println!("{}", total_cards);

    Ok(())
}
