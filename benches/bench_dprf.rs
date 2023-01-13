#[macro_use]
extern crate bencher;

use bencher::Bencher;
use curve25519_dalek::scalar::Scalar;
use rand::{thread_rng, Rng, RngCore};
use rand_core::OsRng;
use socioty::Dprf;

fn bench_evaluate(bench: &mut Bencher) {
    let f = Dprf::random(&mut OsRng);

    bench.iter(|| {
        let timestamp: i32 = rand::thread_rng().gen();
        let input = ((timestamp as i64) / 30).to_ne_bytes();
        f.partial_eval(&input);
    })
}

fn bench_multiply_scalar(bench: &mut Bencher) {
    let mut s1_bytes = [0u8; 32];

    thread_rng().fill_bytes(&mut s1_bytes);

    let mut s1 = Scalar::from_bytes_mod_order(s1_bytes);

    bench.iter(|| {
        for _ in 0..252 {
            let mut s2_bytes = [0u8; 32];
            let s2 = Scalar::from_bytes_mod_order(s2_bytes);
            thread_rng().fill_bytes(&mut s2_bytes);
            s1 *= s2
        }
    })
}

benchmark_group!(benches, bench_evaluate, bench_multiply_scalar);
benchmark_main!(benches);
