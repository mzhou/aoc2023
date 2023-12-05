use std::cmp::min;
use std::io::stdin;

use anyhow::{Context, Error};

struct RangeMap {
    entries: Vec<RangeMapEntry>,
}

struct RangeMapEntry {
    destination: u64,
    length: u64,
    source: u64,
}

impl RangeMap {
    fn convert(&self, source: u64) -> u64 {
        let mut destination = source;
        for entry in self.entries.iter() {
            destination = entry.convert(source);
            if destination != source {
                break;
            }
        }
        destination
    }

    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl RangeMapEntry {
    fn convert(&self, source: u64) -> u64 {
        if source >= self.source && source < self.source + self.length {
            self.destination + (source - self.source)
        } else {
            source
        }
    }
}

fn main() -> Result<(), Error> {
    let mut seeds = Vec::<u64>::new();
    let mut maps = Vec::<RangeMap>::new();

    for line in stdin().lines() {
        let line = line?;
        if line.is_empty() {
        } else if line.starts_with("seeds: ") {
            seeds = line["seeds: ".len()..]
                .split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?;
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

    let mut smallest_location = u64::max_value();

    for seed in seeds.iter() {
        let mut destination = *seed;
        for map in maps.iter() {
            destination = map.convert(destination);
        }
        smallest_location = min(smallest_location, destination);
    }

    println!("{}", smallest_location);

    Ok(())
}
