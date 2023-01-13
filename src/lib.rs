#![no_std]

// We'll need an allocator for the threshold DPRF
extern crate alloc;
use alloc::{string::String, vec::Vec};

use curve25519_dalek::{
    self,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};

use rand_core::{CryptoRng, RngCore};

extern crate sha2;
use sha2::Sha512;

#[derive(Clone, Debug, PartialEq)]
pub struct Dprf {
    key: Scalar,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialEval(RistrettoPoint);

impl PartialEval {
    pub fn from_bytes(bytes: [u8; 32]) -> Option<PartialEval> {
        CompressedRistretto(bytes).decompress().map(PartialEval)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.compress().to_bytes()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output(RistrettoPoint);

impl Output {
    pub fn from_bytes(bytes: [u8; 32]) -> Option<Output> {
        CompressedRistretto(bytes).decompress().map(Output)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.compress().to_bytes()
    }
}

impl Dprf {
    pub fn new(key: Key) -> Dprf {
        Dprf { key: key.0 }
    }

    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self {
            key: Scalar::random(rng),
        }
    }

    pub fn partial_eval(&self, input: &[u8]) -> PartialEval {
        // Hash the input into the group, and multiply it by the DprfOutput
        PartialEval(self.key * RistrettoPoint::hash_from_bytes::<Sha512>(input))
    }

    fn combine_partials(partial_outputs: &[PartialEval]) -> Option<RistrettoPoint> {
        partial_outputs
            .iter()
            .map(|d| d.0)
            .reduce(|accum, o| accum + o)
    }

    pub fn combine(&self, partial_outputs: &[PartialEval], input: &[u8]) -> Option<Output> {
        Dprf::combine_partials(partial_outputs)
            .map(|p| self.partial_eval(input).0 + p)
            .map(|p| Output(p))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tdprf {
    key: Scalar,
}

struct Poly(Vec<Scalar>);

impl Poly {
    // Generate a random polynomial of a given degree, fixing f(0) = key
    fn from_random<R: RngCore + CryptoRng>(rng: &mut R, key: Scalar, degree: u32) -> Self {
        let mut f = Vec::with_capacity((degree + 1) as usize);
        f.push(key);
        for _ in 0..degree {
            f.push(Scalar::random(rng));
        }
        Poly(f)
    }

    // Evaluate a polynomial at x using Horner's method
    fn eval(&self, x: Scalar) -> Scalar {
        let mut y = Scalar::zero();
        for c in self.0.iter().rev() {
            y = y * x + c;
        }
        y
    }

    // Get the Lagrange coefficients for f
    fn lagrange(xs: &[Scalar]) -> Vec<Scalar> {
        let mut lagrange = Vec::with_capacity(xs.len() * xs.len());
        for (i, x0) in xs.iter().enumerate() {
            let mut li = Scalar::one();
            for (j, x1) in xs.iter().enumerate() {
                if i != j {
                    // Need to be careful about signs here, since the field isn't characteristic 2
                    li *= -x1 * ((x0 - x1).invert())
                }
            }

            lagrange.push(li)
        }
        lagrange
    }
}

#[derive(Debug, Clone)]
pub struct Key(Scalar);

impl Key {
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self(Scalar::random(rng))
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Option<Key> {
        Scalar::from_canonical_bytes(bytes).map(Key)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl Tdprf {
    pub fn new(key: Key) -> Self {
        Self { key: key.0 }
    }

    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self {
            key: Scalar::random(rng),
        }
    }

    // TODO: Remove the original key from the API? Need to adjust the test
    pub fn generate<R: RngCore + CryptoRng>(
        rng: &mut R,
        clients: &[String],
        k: u32,
    ) -> (Key, Vec<Key>) {
        // Generate the threshold key
        let original_key = Scalar::random(rng);

        // TODO: Result type (thiserror)
        assert!(clients.len() > 0);
        assert!(clients.len() >= k as usize);

        // Generate a random polynomial f of degree k-1 centered with f(0) = key
        let f = Poly::from_random(rng, original_key, k - 1);

        // Generate the keys for each client
        (
            Key(original_key),
            clients
                .iter()
                .map(|c| {
                    let x = Scalar::hash_from_bytes::<Sha512>(c.as_bytes());
                    let y = f.eval(x);
                    Key(y)
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn partial_eval(&self, input: &[u8]) -> PartialEval {
        // Hash the input into the group, and multiply it by the key
        PartialEval(self.key * RistrettoPoint::hash_from_bytes::<Sha512>(input))
    }

    fn reconstruct_partials(clients: &[String], evals: &[PartialEval]) -> Output {
        assert_eq!(clients.len(), evals.len());
        let xs = clients
            .iter()
            .map(|c| Scalar::hash_from_bytes::<Sha512>(c.as_bytes()))
            .collect::<Vec<_>>();
        let lagrange = Poly::lagrange(&xs);

        let reconstructed = evals.iter().zip(lagrange).map(|(p, li)| li * p.0).sum();

        Output(reconstructed)
    }

    pub fn reconstruct(&self, input: &[u8], clients: &[String], evals: &[PartialEval]) -> Output {
        let from_clients = Tdprf::reconstruct_partials(clients, evals).0;
        Output((self.key * RistrettoPoint::hash_from_bytes::<Sha512>(input)) + from_clients)
    }
}

#[cfg(test)]
mod tests {
    extern crate std; // use standard library only for tests
    use std::time::{SystemTime, UNIX_EPOCH};

    use alloc::format;
    use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
    use rand::rngs::OsRng;
    use sha2::Sha512;

    use super::*;

    // Find f(0) using Lagrange interpolation
    fn interpolate(xs: &[Scalar], ys: &[Scalar]) -> Scalar {
        assert_eq!(xs.len(), ys.len());
        Poly::lagrange(&xs)
            .iter()
            .zip(ys)
            .map(|(li, y)| li * y)
            .sum()
    }

    #[test]
    fn test_poly() {
        let mut rng = OsRng;

        let key = Scalar::random(&mut rng);
        let f = Poly::from_random(&mut rng, key, 3);

        let xs: Vec<Scalar> = (0..5).map(|_| Scalar::random(&mut rng)).collect();

        let ys: Vec<Scalar> = xs.iter().map(|x| f.eval(*x)).collect();
        assert_eq!(
            ys[1],
            f.0[0]
                + (f.0[1] * xs[1])
                + (f.0[2] * (xs[1] * xs[1]) + (f.0[3] * (xs[1] * xs[1] * xs[1])))
        );

        assert_eq!(interpolate(&xs, &ys), key);
        assert_eq!(interpolate(&xs[..4], &ys[..4]), key);
        assert_ne!(interpolate(&xs[..3], &ys[..3]), key);
    }

    #[test]
    fn test_threshold_dprf() {
        let mut rng = OsRng;
        let clients = (1..6)
            .into_iter()
            .map(|i| format!("192.168.0.{}", i))
            .collect::<Vec<_>>();

        // Simulate TOTP
        let input: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            / 30;
        let input = &input.to_ne_bytes();

        let (key, subkeys) = Tdprf::generate(&mut rng, &clients, 4);
        let evals = subkeys
            .into_iter()
            .map(|key| Tdprf::new(key).partial_eval(input))
            .collect::<Vec<_>>();

        let reconstructed = Tdprf::reconstruct_partials(&clients, &evals);
        assert_eq!(
            reconstructed.0,
            key.0 * RistrettoPoint::hash_from_bytes::<Sha512>(input)
        );

        let reconstructed = Tdprf::reconstruct_partials(&clients[..4], &evals[..4]);
        assert_eq!(
            reconstructed.0,
            key.0 * RistrettoPoint::hash_from_bytes::<Sha512>(input)
        );

        // Below threshold
        let reconstructed = Tdprf::reconstruct_partials(&clients[..3], &evals[..3]);
        assert_ne!(
            reconstructed.0,
            key.0 * RistrettoPoint::hash_from_bytes::<Sha512>(input)
        );

        // Check reconstruct with
        let main = Tdprf::random(&mut rng);
        assert_eq!(
            main.reconstruct(input, &clients, &evals),
            main.reconstruct(input, &clients[..4], &evals[..4])
        );
        assert_ne!(
            main.reconstruct(input, &clients, &evals),
            main.reconstruct(input, &clients[..3], &evals[..3])
        );
    }

    #[test]
    fn test_dprf_all() {
        let f1 = Dprf::random(&mut OsRng);
        let f2 = Dprf::random(&mut OsRng);

        // Simulate TOTP
        let input: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            / 30;
        let input = input.to_ne_bytes();

        let eval1 = f1.partial_eval(&input);
        let eval2 = f2.partial_eval(&input);
        let evals = [eval1, eval2];

        assert_eq!(
            evals[0],
            PartialEval::from_bytes(evals[0].to_bytes()).unwrap()
        );

        assert_ne!(
            evals[0],
            PartialEval::from_bytes(evals[1].to_bytes()).unwrap()
        );

        let combined = Output(Dprf::combine_partials(&evals).unwrap());
        let combined_with = f1.combine(&evals[1..], &input).unwrap();

        assert_eq!(
            Output::from_bytes(combined.to_bytes()).unwrap(),
            Output::from_bytes(combined_with.to_bytes()).unwrap()
        );
        assert_eq!(combined, combined_with);

        let scalar_k1 = f1.key;
        let scalar_k2 = f2.key;
        let x_point = RistrettoPoint::hash_from_bytes::<Sha512>(&input);
        assert_eq!(combined.0, (scalar_k1 + scalar_k2) * x_point);
    }
}
