use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut times = Vec::<u64>::new();
    let mut distances = Vec::<u64>::new();
    let mut win_combos = 1;

    for line in stdin().lines() {
        let line = line?;
        let numbers = line[9..]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        if times.is_empty() {
            times = numbers;
        } else if distances.is_empty() {
            distances = numbers;
        }
    }

    for i in 0..times.len() {
        let time = times[i] as f64;
        let distance = distances[i] as f64;
        // time_held * (time_race - time_held) > distance
        // time_held * time_race - time_held**2 > distance
        // -time_held**2 + time_race*time_held - distance > 0
        // a = -1, b = time_race, c = -distance
        let mut x = [
            ((-time - (time * time - 4. * (-1.) * (-distance)).sqrt()) / (2. * (-1.))),
            ((-time + (time * time - 4. * (-1.) * (-distance)).sqrt()) / (2. * (-1.))),
        ];
        x.sort_by(|a, b| a.partial_cmp(b).unwrap()); // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
        let combos = ((x[1] - 0.0000001).floor() - (x[0] + 0.000001).ceil() + 1.) as u64;
        eprintln!("{} {} => {} {} {}", time, distance, x[0], x[1], combos);
        win_combos *= combos as u64;
    }

    println!("{}", win_combos);

    Ok(())
}
