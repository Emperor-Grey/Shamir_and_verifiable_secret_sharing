use std::vec;

use num_bigint::BigInt;
use rand::prelude::*;

use crate::vss::{VSSCommitments, VSSParams};

#[derive(Debug, Clone)]
pub struct SharmirModel {
    secret: i64,
    shares: usize,
    threshold: usize,
    generated_shares: Vec<(i64, i64)>,
    coefficients: Vec<i64>,
    vss_commitments: Option<VSSCommitments>,
    vss_params: VSSParams,
}

impl SharmirModel {
    pub fn new(secret: i64, shares: usize, threshold: usize) -> Self {
        Self {
            secret,
            shares,
            threshold,
            generated_shares: vec![],
            coefficients: vec![],
            vss_commitments: None,
            vss_params: VSSParams::new(),
        }
    }

    pub fn construct_polynomial(&mut self, x: i64) -> i64 {
        let mut rng = rand::thread_rng();
        let mut sum = self.secret;

        // Store coefficients for VSS if not already generated
        if self.coefficients.is_empty() {
            self.coefficients = vec![self.secret];
            for _ in 1..self.threshold {
                let coefficient = rng.gen_range(1..=self.secret / 2);
                self.coefficients.push(coefficient);
            }
            // Generate VSS commitments
            self.vss_commitments = Some(VSSCommitments::new(&self.coefficients, &self.vss_params));
        }

        for (power, &coeff) in self.coefficients[1..].iter().enumerate() {
            sum += coeff * x.pow((power + 1) as u32);
        }

        sum
    }

    pub fn verify_share(&self, x: i64, share: i64) -> bool {
        if let Some(commitments) = &self.vss_commitments {
            commitments.verify_share(x, share, &self.vss_params)
        } else {
            false
        }
    }

    // Simply return a reference to generated_shares
    // Use &self as parameter to borrow immutably
    pub fn get_shares(&mut self) -> &Vec<(i64, i64)> {
        &self.generated_shares
    }

    // 1. Create empty vector for shares
    // 2. Loop from 0 to self.shares
    // 3. For each iteration:
    //    - Convert loop index to i64 for x value
    //    - Call construct_polynomial(x) to get y value
    //    - Push tuple (x,y) to shares vector
    // 4. Finally assign shares vector to self.generated_shares
    // Note: Need &mut self since we're modifying state
    pub fn generate_shares(&mut self) {
        let mut new_shares: Vec<(i64, i64)> = vec![];

        for i in 0..self.shares {
            let x = i as i64;
            let y = self.construct_polynomial(x);
            new_shares.push((x, y));
        }
        self.generated_shares = new_shares;
    }

    // - Steps:
    //   1. Split shares into x and y vectors
    //   2. Calculate Lagrange basis polynomials
    //   3. Sum up the interpolation
    //   4. Convert result back to u64
    pub fn reconstruct_secret(&mut self, shares: &[(i64, i64)]) -> i64 {
        let (x_values, y_values) = self.split_shares(shares);
        let mut result = 0.0;

        for i in 0..shares.len() {
            let (numerator, denominator) = self.lagrange_basis(i, &x_values);
            result += y_values[i] as f64 * numerator / denominator;
        }

        result.round() as i64
    }

    fn split_shares(&self, shares: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
        let x_values: Vec<i64> = shares.iter().map(|&(x, _)| x).collect();
        let y_values: Vec<i64> = shares.iter().map(|&(_, y)| y).collect();
        (x_values, y_values)
    }

    fn lagrange_basis(&self, share_index: usize, x_values: &[i64]) -> (f64, f64) {
        let mut numerator = 1.0;
        let mut denominator = 1.0;

        for (index, &current_x) in x_values.iter().enumerate() {
            if index != share_index {
                numerator *= current_x as f64;
                denominator *= (current_x - x_values[share_index]) as f64;
            }
        }

        (numerator, denominator)
    }
}
