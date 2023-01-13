use ascon_aead::aead::{Aead, KeyInit};
use ascon_aead::{Ascon128, Key, Nonce};
use rand::{rngs::OsRng, Rng};
use socioty::Tdprf;

use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let runs = args[1].parse().unwrap();
    let use_aead = args.len() > 2 && args[2] == "aead";

    let key = Key::<Ascon128>::from_slice(b"very secret key.");
    let cipher = Ascon128::new(key);

    let mut durations = Vec::with_capacity(runs as usize);
    for _ in 0..runs {
        let f = Tdprf::random(&mut OsRng);
        let timestamp: i32 = rand::thread_rng().gen();
        let input = ((timestamp as i64) / 30).to_ne_bytes();

        let elapsed = if use_aead {
            let now = Instant::now();
            {
                let p = f.partial_eval(&input).to_bytes();
                let random_nonce: [u8; 16] = OsRng.gen();
                let nonce = Nonce::<Ascon128>::from_slice(&random_nonce);
                let _ = cipher.encrypt(&nonce, p.as_ref()).unwrap();
            }
            now.elapsed()
        } else {
            let now = Instant::now();
            {
                let _ = f.partial_eval(&input);
            }
            now.elapsed()
        };
        durations.push(elapsed);
    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("{:?}", avg);
}
