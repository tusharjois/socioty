use std::time::{Duration, Instant};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
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
        let now = Instant::now();
        {
            let _ = Tdprf::generate(&mut EspRng, &clients, k);
        }
        let elapsed = now.elapsed();
        println!("Run {} of {}: {:?}", r + 1, runs, elapsed);
        durations.push(elapsed);

    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("Average time {:?}", avg);
    println!("Fin.");
    loop { }
}
