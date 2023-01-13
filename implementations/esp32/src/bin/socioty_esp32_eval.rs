use std::time::{Duration, Instant};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use rand_core::RngCore;
use socioty::Tdprf;
use socioty_esp32::EspRng;

#[cfg(feature = "aead")]
use ascon_aead::aead::{Aead, KeyInit};
#[cfg(feature = "aead")]
use ascon_aead::{Ascon128, Key, Nonce};

const RUNS_RAW: &'static str = env!(
    "RUNS",
    "number of runs is missing, define RUNS=<X> at compile time"
);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    unsafe {
        esp_idf_sys::esp_task_wdt_init(3600, false);
    }

    let runs = RUNS_RAW.parse().unwrap();

    #[cfg(feature = "aead")]
    let key = Key::<Ascon128>::from_slice(b"very secret key.");
    #[cfg(feature = "aead")]
    let cipher = Ascon128::new(key);

    let mut durations = Vec::with_capacity(runs as usize);
    for r in 0..runs {
        let f = Tdprf::random(&mut EspRng);
        let timestamp = EspRng.next_u32() as i64;
        let input = ((timestamp) / 30).to_ne_bytes();

        #[cfg(not(feature = "aead"))]
        let elapsed = {
            let now = Instant::now();
            {
                let _ = f.partial_eval(&input);
            }
            now.elapsed()
        };

        #[cfg(feature = "aead")]
        let elapsed = {
            let now = Instant::now();
            {
                let mut p = f.partial_eval(&input).to_bytes();
                let mut random_nonce = [0u8; 16];
                EspRng.fill_bytes(&mut random_nonce);
                let nonce = Nonce::<Ascon128>::from_slice(&random_nonce);
                let _ = cipher.encrypt(&nonce, p.as_ref()).unwrap();
            }
            now.elapsed()
        };

        println!("Run {} of {}: {:?}", r + 1, runs, elapsed);
        durations.push(elapsed);
    }

    let sum: Duration = durations.iter().sum();
    let avg = sum / runs;
    println!("Average time {:?}", avg);
    println!("Fin.");
    loop {}
}
