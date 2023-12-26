use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    for line in stdin().lines() {
        let line = line?;

        let mut hash_total = 0u64;
        for section in line.as_bytes().split(|b| *b == b',') {
            let mut hash = 0u16;
            for b in section.iter().cloned() {
                hash += b as u16;
                hash *= 17;
                hash = hash & 0xff;
            }
            hash_total += hash as u64;
        }

        println!("{}", hash_total);
    }

    Ok(())
}
