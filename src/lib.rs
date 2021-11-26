pub mod error;
pub mod mlsag;
pub mod ringct;

use blstrs::{group::ff::Field, G1Projective, Scalar};

pub use blstrs;
pub use error::Error;
pub use mlsag::{DecoyInput, MlsagMaterial, MlsagSignature, TrueInput};
pub use ringct::{Output, RingCtMaterial};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub struct RevealedCommitment {
    pub value: u64,
    pub blinding: Scalar,
}

impl RevealedCommitment {
    /// Construct a revealed commitment from a value, generating a blinding randomly
    pub fn from_value(value: u64, mut rng: impl rand_core::RngCore) -> Self {
        Self {
            value,
            blinding: Scalar::random(&mut rng),
        }
    }

    pub fn commit(&self, pc_gens: &bulletproofs::PedersenGens) -> G1Projective {
        pc_gens.commit(Scalar::from(self.value), self.blinding)
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn blinding(&self) -> Scalar {
        self.blinding
    }
}

/// Hashes a point to another point on the G1 curve
pub fn hash_to_curve(p: G1Projective) -> G1Projective {
    const DOMAIN: &[u8; 25] = b"blst-ringct-hash-to-curve";
    G1Projective::hash_to_curve(&p.to_compressed(), DOMAIN, &[])
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
