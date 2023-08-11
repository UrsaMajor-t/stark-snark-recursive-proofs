use winter_circom_prover::{circom_compile, utils::{LoggingLevel, WinterCircomError}};

#[allow(dead_code)]
mod prover;
use prover::WorkProver;

mod air;
use air::PROOF_OPTIONS;
use winter_circom_prover::hash::Poseidon;
use winter_circom_prover::winterfell::crypto::{ElementHasher, Hasher};

fn main() -> Result<(), WinterCircomError> {
    circom_compile::<WorkProver<Poseidon>, 2>(PROOF_OPTIONS, "sum", LoggingLevel::Default)
}
