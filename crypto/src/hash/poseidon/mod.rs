mod param;
use core::marker::PhantomData;


mod poseidon;

#[cfg(test)]
mod tests;

use super::{Digest,ElementHasher, Hasher, ByteDigest};
//FIXME: f64 -> f256
use math::{FieldElement, StarkField};

mod digest;
pub use digest::ElementDigest;

// POSEIDON WITH 256-BIT OUTPUT
// ===============================================================================================
/// Implementation of the [Hasher](super::Hasher) trait for POSEIDON hash function with 256-bit
/// output.
const DIGEST_SIZE : usize = 1;



pub struct Poseidon<B: StarkField>(PhantomData<B>);

impl<B:StarkField> Hasher for Poseidon<B> {
    // TODO: ByteDigest<32>; ?  See SHA3 / RESCUE
    type Digest = ByteDigest<32>;

    fn hash(bytes: &[u8]) -> Self::Digest {
        // return the first [RATE] elements of the state as hash result
        poseidon::digest(bytes)
    }

    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        let mut data = [0; 64];
        data[..32].copy_from_slice(values[0].0.as_slice());
        data[32..].copy_from_slice(values[1].0.as_slice());
        poseidon::digest(&data)
    }

    fn merge_with_int(seed: Self::Digest, value: u64) -> Self::Digest {
        let mut data = [0; 40];
        data[..32].copy_from_slice(&seed.0);
        data[32..].copy_from_slice(&value.to_le_bytes());
        poseidon::digest(&data)
    }
}

impl<B: StarkField> ElementHasher for Poseidon<B> {
    type BaseField = B;

    fn hash_elements<E: FieldElement<BaseField = Self::BaseField>>(elements: &[E]) -> Self::Digest {

        assert!(B::IS_CANONICAL);

        let bytes = E::elements_as_bytes(elements);
        poseidon::digest(bytes)

    }
}
