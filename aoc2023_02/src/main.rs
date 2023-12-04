use std::io::stdin;

use anyhow::{Context, Error};
use regex::Regex;

fn main() -> Result<(), Error> {
    let mut game_id = 1;
    let mut possible_id_sum = 0;

    let re = Regex::new("([0-9]+) (blue|green|red)")?;

    for line in stdin().lines().into_iter() {
        let line = line?;

        let reveals = line.split(";");

        let mut game_possible = true;

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
            }
            let possible = blue <= 14 && green <= 13 && red <= 12;
            game_possible = game_possible && possible;
        }

        if game_possible {
            possible_id_sum += game_id;
        }

        game_id += 1;
    }

    println!("{}", possible_id_sum);

    Ok(())
}
