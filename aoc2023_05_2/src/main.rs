use std::cmp::min;
use std::io::stdin;

use anyhow::{Context, Error};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Range {
    length: u64,
    start: u64,
}

struct RangeMap {
    entries: Vec<RangeMapEntry>,
}

struct RangeMapEntry {
    destination: u64,
    length: u64,
    source: u64,
}

impl Range {
    fn until_max(start: u64) -> Self {
        Self {
            length: u64::max_value() - start,
            start,
        }
    }
}

impl RangeMap {
    fn convert(&self, source: u64) -> Range {
        let initial_destination = Range::until_max(source);
        let mut destination = initial_destination;
        for entry in self.entries.iter() {
            destination = entry.convert(source);
            if destination != initial_destination {
                break;
            }
            if entry.source > source {
                destination.length = entry.source - source;
            }
        }
        destination
    }

    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn prepare(&mut self) {
        self.entries.sort_by_key(|entry| entry.source);
    }
}

impl RangeMapEntry {
    fn convert(&self, source: u64) -> Range {
        if source >= self.source && source < self.source + self.length {
            Range {
                length: self.length - (source - self.source),
                start: self.destination + (source - self.source),
            }
        } else {
            Range::until_max(source)
        }
    }
}

fn main() -> Result<(), Error> {
    let mut seed_ranges = Vec::<Range>::new();
    let mut maps = Vec::<RangeMap>::new();

    for line in stdin().lines() {
        let line = line?;
        if line.is_empty() {
        } else if line.starts_with("seeds: ") {
            let mut numbers = line["seeds: ".len()..].split(" ").map(|s| s.parse::<u64>());
            while let Some(start) = numbers.next() {
                let start = start?;
                let length = numbers.next().context("missing length")??;
                let range = Range { length, start };
                seed_ranges.push(range);
            }
        } else if line.ends_with(" map:") {
            maps.push(RangeMap::new());
        } else {
            let mut numbers = line.split(" ");
            let destination = numbers
                .next()
                .context("missing destination from entry")?
                .parse::<u64>()?;
            let source = numbers
                .next()
                .context("missing source from entry")?
                .parse::<u64>()?;
            let length = numbers
                .next()
                .context("missing length from entry")?
                .parse::<u64>()?;
            let entry = RangeMapEntry {
                destination,
                length,
                source,
            };
            maps.last_mut().context("map missing")?.entries.push(entry);
        }
    }

    for map in maps.iter_mut() {
        map.prepare();
    }

    let mut smallest_location = u64::max_value();

    for seed_range in seed_ranges.iter() {
        let mut seed = seed_range.start;
        while seed < seed_range.start + seed_range.length {
            let mut destination = Range::until_max(seed);
            for map in maps.iter() {
                let this_destination = map.convert(destination.start);
                destination = Range {
                    length: min(this_destination.length, destination.length),
                    start: this_destination.start,
                };
            }
            smallest_location = min(smallest_location, destination.start);
            seed += destination.length;
        }
    }

    println!("{}", smallest_location);

    Ok(())
}
