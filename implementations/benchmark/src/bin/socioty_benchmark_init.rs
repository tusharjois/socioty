use std::time::{Duration, Instant};

use rand::rngs::OsRng;
use socioty::Tdprf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let runs = args[1].parse().unwrap();
    let clients = (0..args[2].parse().unwrap())
        .into_iter()
        .map(|i| format!("192.168.0.{}", i + 1))
        .collect::<Vec<_>>();
    let k = args[3].parse().unwrap();

    let mut durations = Vec::with_capacity(runs as usize);
    for _ in 0..runs {
        let now = Instant::now();
        {
            Tdprf::generate(&mut OsRng, &clients, k);
        }
        let elapsed = now.elapsed();
        durations.push(elapsed);
    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("{:?}", avg);
}
