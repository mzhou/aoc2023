use std::cmp::max;
use std::io::stdin;

use anyhow::{Context, Error};
use regex::Regex;

fn main() -> Result<(), Error> {
    let mut power_total = 0;

    let re = Regex::new("([0-9]+) (blue|green|red)")?;

    for line in stdin().lines().into_iter() {
        let line = line?;

        let reveals = line.split(";");

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for reveal in reveals {
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            let captures = re.captures_iter(reveal);
            for capture in captures {
                let color = capture.get(2).context("color group missing")?.as_str();
                let quantity: i32 = capture
                    .get(1)
                    .context("quantity group missing")?
                    .as_str()
                    .parse()
                    .context("failed conversion")?;
                match color {
                    "blue" => {
                        blue = quantity;
                    }
                    "green" => {
                        green = quantity;
                    }
                    "red" => {
                        red = quantity;
                    }
                    _ => {
                        return Err(Error::msg("unknown color"));
                    }
                }
                max_blue = max(max_blue, blue);
                max_green = max(max_green, green);
                max_red = max(max_red, red);
            }
        }

        let power = max_blue * max_green * max_red;

        power_total += power;
    }

    println!("{}", power_total);

    Ok(())
}
