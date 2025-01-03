#![allow(unused, dead_code)]
mod shamir;
mod vss;

use shamir::SharmirModel;
use std::env;

// How to run -> cargo run args
// -q for silent mode 143 - secret_number 5 - num_of_shares 2 - threshold

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Please give all the args... (secret, shares, threshold)");
        std::process::exit(1);
    }

    let secret: i64 = args[1].parse().expect("Secret must be an integer");
    let shares: usize = args[2].parse().expect("Shares must be an integer");

    let mut s = SharmirModel::new(secret, shares, 3);
    let mut m = s.clone();

    s.generate_shares();
    let generated_shares = s.get_shares().clone();

    let sum = m.construct_polynomial(1);
    println!("Polynomial value sum at x=1: {}", sum);

    println!("Generated shares: {:?}", generated_shares);

    // Verify each share
    for &(x, share) in &generated_shares {
        let is_valid = s.verify_share(x, share);
        println!("Share ({}, {}) is valid: {}", x, share, is_valid);
    }

    let reconstructed_secret = m.reconstruct_secret(&generated_shares);
    println!("Reconstructed secret: {}", reconstructed_secret);
}
