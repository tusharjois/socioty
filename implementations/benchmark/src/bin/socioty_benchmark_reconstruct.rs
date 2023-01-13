use rand::{rngs::OsRng, Rng};
use socioty::{Key, Tdprf};

use std::time::{Duration, Instant};

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
        let (_, subkeys) = Tdprf::generate(&mut OsRng, &clients, k);

        let timestamp: i32 = rand::thread_rng().gen();
        let input = ((timestamp as i64) / 30).to_ne_bytes();

        let mut partial_outputs = Vec::with_capacity(clients.len());
        for subkey in subkeys {
            let f = Tdprf::new(subkey);
            partial_outputs.push(f.partial_eval(&input));
        }

        let f = Tdprf::new(Key::random(&mut OsRng));

        let now = Instant::now();
        {
            f.reconstruct(&input, &clients, &partial_outputs);
        }
        let elapsed = now.elapsed();
        durations.push(elapsed);
    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("{:?}", avg);
}
