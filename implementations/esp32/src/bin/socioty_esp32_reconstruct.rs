use std::time::{Duration, Instant};

use esp_idf_sys as _;
use rand_core::RngCore;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use socioty::Tdprf;
use socioty_esp32::EspRng;

const RUNS_RAW: &'static str = env!("RUNS", "number of runs is missing, define RUNS=<X> at compile time");
const CLIENTS_RAW: &'static str = env!("CLIENTS", "number of clients is missing, define CLIENTS=<X> at compile time");
const K_RAW: &'static str = env!("K", "threshold k is missing, define K=<X> at compile time");

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    unsafe { esp_idf_sys:: esp_task_wdt_init(3600, false); }

    let runs = RUNS_RAW.parse().unwrap();
    let clients = (0..CLIENTS_RAW.parse().unwrap())
        .into_iter()
        .map(|i| format!("192.168.0.{}", i + 1))
        .collect::<Vec<_>>();
    let k = K_RAW.parse().unwrap();
    println!("Running {} with {} threshold", CLIENTS_RAW, k);

    let mut durations = Vec::with_capacity(runs as usize);
    for r in 0..runs {
        println!("Run {} of {}", r + 1, runs);
        let (_, subkeys) = Tdprf::generate(&mut EspRng, &clients, k);

        let timestamp = EspRng.next_u32() as i64;
        let input = ((timestamp) / 30).to_ne_bytes();

        let mut partial_outputs = Vec::with_capacity(clients.len());
        for subkey in subkeys {
            let f = Tdprf::new(subkey);
            partial_outputs.push(f.partial_eval(&input));
            println!(
                "  Eval(s) completed: {} of {}",
                partial_outputs.len(),
                CLIENTS_RAW
            );
        }

        let f = Tdprf::random(&mut EspRng);

        let now = Instant::now();
        {
            f.reconstruct(&input, &clients, &partial_outputs);
        }
        let elapsed = now.elapsed();
        println!("  Reconstruct time: {:?}", elapsed);
        durations.push(elapsed);
    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("Average reconstruct time {:?}", avg);
    println!("Fin.");
    loop { }
}
