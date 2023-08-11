use winterfell::math::fields::f256::BaseElement;

// TYPES AND INTERFACES
// ================================================================================================

type Blake3_192 = winterfell::crypto::hashers::Blake3_192<BaseElement>;
type Blake3_256 = winterfell::crypto::hashers::Blake3_256<BaseElement>;
type Sha3_256 = winterfell::crypto::hashers::Sha3_256<BaseElement>;
type Rp64_256 = winterfell::crypto::hashers::Rp64_256;
type RpJive64_256 = winterfell::crypto::hashers::RpJive64_256;
type GriffinJive64_256 = winterfell::crypto::hashers::GriffinJive64_256;
pub type Poseidon = winterfell::crypto::hashers::Poseidon<BaseElement>;

/// Defines a set of hash functions available for the provided examples. Some examples may not
/// support all listed hash functions.
///
/// Choice of a hash function has a direct impact on proof generation time, proof size, and proof
/// soundness. In general, sounds of the proof is bounded by the collision resistance of the hash
/// function used by the protocol.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashFunction {
    /// BLAKE3 hash function with 192 bit output.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 96 bits.
    Blake3_192,

    /// BLAKE3 hash function with 256 bit output.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 128 bits.
    Blake3_256,

    /// SHA3 hash function with 256 bit output.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 128 bits.
    Sha3_256,

    /// Rescue Prime hash function with 256 bit output. It only works in `f64` field.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 128 bits.
    Rp64_256,

    /// Rescue Prime hash function with 256 bit output. It only works in `f64` field.
    /// This instance uses the Jive compression mode in Merkle trees.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 128 bits.
    RpJive64_256,

    /// Griffin hash function with 256 bit output. It only works in `f64` field.
    /// This instance uses the Jive compression mode in Merkle trees.
    ///
    /// When this function is used in the STARK protocol, proof security cannot exceed 128 bits.
    GriffinJive64_256,

    Poseidon,
}