use std::io::stdin;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let mut total = 0;
    for line in stdin().lines().into_iter() {
        let line = line?;
        let mut first_digit = Option::<u32>::default();
        let mut last_digit = Option::<u32>::default();
        let mut current = line.as_str();
        while !current.is_empty() {
            let unicode = current.chars().next().unwrap() as u32;
            let mut digit = Option::<u32>::default();
            if current.starts_with("one") {
                digit = Some(1);
            } else if current.starts_with("two") {
                digit = Some(2);
            } else if current.starts_with("three") {
                digit = Some(3);
            } else if current.starts_with("four") {
                digit = Some(4);
            } else if current.starts_with("five") {
                digit = Some(5);
            } else if current.starts_with("six") {
                digit = Some(6);
            } else if current.starts_with("seven") {
                digit = Some(7);
            } else if current.starts_with("eight") {
                digit = Some(8);
            } else if current.starts_with("nine") {
                digit = Some(9);
            } else if unicode >= '0' as u32 && unicode <= '9' as u32 {
                digit = Some(unicode - '0' as u32);
            }
            if let Some(digit) = digit {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            }
            current = &current[1..];
        }
        let first_digit = first_digit.ok_or(Error::msg("no first digit"))?;
        let last_digit = last_digit.ok_or(Error::msg("no last digit"))?;
        total += first_digit * 10 + last_digit;
    }
    println!("{}", total);
    Ok(())
}
