use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct VSSParams {
    pub p: BigInt, // Large prime
    pub q: BigInt, // Prime divisor of p-1
    pub g: BigInt, // Generator of order q
}

#[derive(Debug, Clone)]
pub struct VSSCommitments {
    commitments: Vec<BigInt>,
}

impl VSSParams {
    pub fn new() -> Self {
        // TODO Change to larger primes
        let p = BigInt::parse_bytes(b"2039", 10).unwrap(); // Example prime
        let q = BigInt::parse_bytes(b"1019", 10).unwrap(); // (p-1)/2
        let g = BigInt::from(2); // Generator

        Self { p, q, g }
    }
}

impl VSSCommitments {
    pub fn new(coefficients: &[i64], params: &VSSParams) -> Self {
        let mut commitments = Vec::new();

        for &coeff in coefficients {
            let commitment = params.g.modpow(&BigInt::from(coeff), &params.p);
            commitments.push(commitment);
        }

        Self { commitments }
    }

    pub fn verify_share(&self, x: i64, share: i64, params: &VSSParams) -> bool {
        let mut expected = BigInt::one();
        let x_big = BigInt::from(x);

        for (i, commitment) in self.commitments.iter().enumerate() {
            let power = x_big.modpow(&BigInt::from(i), &params.p);
            let term = commitment.modpow(&power, &params.p);
            expected = (expected * term) % &params.p;
        }

        let actual = params.g.modpow(&BigInt::from(share), &params.p);
        expected == actual
    }
}
