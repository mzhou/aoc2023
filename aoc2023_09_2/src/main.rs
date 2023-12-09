use std::error::Error;
use std::io::stdin;

fn differentiate(numbers: &[i64]) -> Vec<i64> {
    let mut ret = Vec::<i64>::new();

    for i in 1..numbers.len() {
        ret.push(numbers[i] - numbers[i - 1]);
    }

    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut total = 0;

    for line in stdin().lines() {
        let line = line?;

        let numbers = line
            .split(' ')
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut differentiated = numbers.clone();
        let mut extrapolated = *numbers.first().unwrap();
        let mut direction = -1;

        loop {
            differentiated = differentiate(&differentiated);
            if differentiated.iter().all(|n| *n == 0) {
                break;
            }
            extrapolated += direction * *differentiated.first().unwrap();
            direction = -direction;
        }

        eprintln!("{}", extrapolated);

        total += extrapolated;
    }

    println!("{}", total);

    Ok(())
}
