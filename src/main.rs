#![allow(unused, dead_code)]
mod shamir;
mod verifiable_secret;

use shamir::SharmirModel;
use std::env;
use verifiable_secret::VerifiableSecretSharing;

// How to run -> cargo run args
// -q for silent mode 143 - secret_number 5 - num_of_shares 2 - threshold

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Please give all the args... (secret, shares, threshold)");
        std::process::exit(1);
    }

    let secret: i64 = args[1].parse().expect("Secret must be an integer");
    let shares: usize = args[2].parse().expect("Shares must be an integer");
    let threshold: usize = args[3].parse().expect("Threshold must be an integer");

    let mut s = SharmirModel::new(secret, shares, threshold);
    let mut m = s.clone();

    s.generate_shares();
    let shares = s.get_shares();

    let sum = m.construct_polynomial(1);
    println!("Polynomial value sum at x=1: {}", sum);

    println!("Generated shares: {:?}", shares);

    let reconstructed_secret = m.reconstruct_secret(shares);
    println!("Reconstructed secret: {}", reconstructed_secret);

    // VSS CHECKING
    let prime = 101;
    let generator = 2;
    let mut vss = VerifiableSecretSharing::new(prime, generator);

    let coefficients = (0..threshold)
        .map(|x| m.construct_polynomial(x as i64))
        .collect::<Vec<_>>();
    vss.generate_commitments(&coefficients);
    println!("Commitments: {:?}", vss.commitments);

    for &(x, y) in shares {
        let is_valid = vss.verify_share(x, y);
        println!(
            "Share ({}, {}): {}",
            x,
            y,
            if is_valid { "Valid" } else { "Invalid" }
        );
    }
}
