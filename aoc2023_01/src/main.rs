use std::io::stdin;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let mut total = 0;
    for line in stdin().lines().into_iter() {
        let line = line?;
        let mut first_digit = Option::<u32>::default();
        let mut last_digit = Option::<u32>::default();
        for c in line.chars() {
            let unicode = c as u32; // https://stackoverflow.com/a/54583437
            if unicode >= '0' as u32 && unicode <= '9' as u32 {
                if first_digit.is_none() {
                    first_digit = Some(unicode - '0' as u32);
                }
                last_digit = Some(unicode - '0' as u32);
            }
        }
        let first_digit = first_digit.ok_or(Error::msg("no first digit"))?;
        let last_digit = last_digit.ok_or(Error::msg("no last digit"))?;
        total += first_digit * 10 + last_digit;
    }
    println!("{}", total);
    Ok(())
}
