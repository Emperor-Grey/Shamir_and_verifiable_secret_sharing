use rand::prelude::*;

// Represents the Verifiable Secret Sharing (VSS) scheme.
// This uses modular arithmetic to securely share and verify secrets.
//
// - `prime` is the large prime number \( p \), defining the finite field \( \mathbb{Z}_p \).
// - `generator` is the generator \( g \) of the field \( \mathbb{Z}_p^* \).
// - `commitments` contains \( g^{a_i} \mod p \) for each coefficient \( a_i \) of the secret polynomial.
pub struct VerifiableSecretSharing {
    pub prime: i64,
    pub generator: i64,
    pub commitments: Vec<i64>, // Commitments for verification (g^(a_i) mod p)
}

impl VerifiableSecretSharing {
    pub fn new(prime: i64, generator: i64) -> Self {
        Self {
            prime,
            generator,
            commitments: vec![],
        }
    }

    // **Formula**: For each coefficient \( a_i \), compute \( g^{a_i} \mod p \).
    // These commitments are stored in the `commitments` vector for later verification.
    //
    // # Arguments
    // - `coefficients`: The coefficients \( a_0, a_1,...a_k \) of the secret polynomial.
    pub fn generate_commitments(&mut self, coefficients: &[i64]) {
        self.commitments = coefficients
            .iter()
            // Compute g^(a_i) mod p
            .map(|&coeff| self.mod_exp(self.generator, coeff, self.prime))
            .collect();
    }

    // **Formula**:
    // - Left-hand side (LHS): \( g^y \mod p \), where \( y = f(x) \) is the share value.
    // - Right-hand side (RHS): \( g^{a_0} \cdot g^{a_1 \cdot x} \cdot g^{a_2 \cdot x^2} \cdots \mod p \).
    // Verification passes if LHS == RHS.
    //
    // # Arguments
    // - `x`: The share's \( x \)-coordinate.
    // - `y`: The share's \( y \)-value (i.e., the result of \( f(x) \)).
    //
    // # Returns
    // - `true` if the share is valid, `false` otherwise.
    pub fn verify_share(&self, x: i64, y: i64) -> bool {
        let lhs = self.mod_exp(self.generator, y, self.prime); // Compute LHS: g^y mod p
        let rhs = self.calculate_rhs(x); // Compute RHS: g^(a_0) * g^(a_1 * x) * ... mod p
        lhs == rhs // Check if LHS equals RHS
    }

    // !Copied from gpt
    // **Formula**: \( \text{base}^{\text{exp}} \mod \text{modulus} \).
    // Efficiently calculates \( b^e \mod m \).
    //
    // # Arguments
    // - `base`: The base \( b \).
    // - `exp`: The exponent \( e \).
    // - `modulus`: The modulus \( m \).
    //
    // # Returns
    // - The result of \( b^e \mod m \).
    fn mod_exp(&self, base: i64, exp: i64, modulus: i64) -> i64 {
        let mut result = 1; // Start with the multiplicative identity
        let mut base = base % modulus; // Ensure base is within modulus
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                // If the current bit of the exponent is 1, multiply result by base
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus; // Square the base
            exp /= 2; // Shift to the next bit
        }

        result
    }

    // **Formula**:
    // \( \text{RHS} = g^{a_0} \cdot g^{a_1 \cdot x} \cdot g^{a_2 \cdot x^2} \cdots \mod p \).
    // This uses the commitments \( g^{a_i} \) and evaluates them at \( x \).
    //
    // # Arguments
    // - `x`: The share's \( x \)-coordinate.
    //
    // # Returns
    // - The calculated RHS.
    fn calculate_rhs(&self, x: i64) -> i64 {
        let mut result = 1; // Start with the multiplicative identity
        let mut power_of_x = 1; // Represents \( x^0, x^1, x^2, \dots \)

        for &commitment in &self.commitments {
            // For each commitment \( g^{a_i} \), compute \( (g^{a_i})^{x^i} \mod p \)
            result = (result * self.mod_exp(commitment, power_of_x, self.prime)) % self.prime;
            power_of_x *= x; // Increment the power of x
        }

        result
    }
}
