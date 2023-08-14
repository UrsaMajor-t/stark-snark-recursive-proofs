use std::marker::PhantomData;
use winter_circom_prover::{circom_compile, circom_prove, SNP, WinterCircomProofOptions, WinterPublicInputs};
use winter_circom_prover::utils::{LoggingLevel, WinterCircomError};
use winter_circom_prover::winterfell::crypto::ElementHasher;
use winter_circom_prover::winterfell::math::fields::f256::BaseElement;
use winter_circom_prover::winterfell::{Air, ProofOptions, Prover, Trace};
use winter_circom_prover::winterfell::math::FieldElement;

mod air;
pub(crate) use air::PROOF_OPTIONS;

mod prover;
pub use prover::WorkProver;
use winter_circom_prover::hash::Poseidon;

fn main(){
    let  i =get_instance().unwrap();
    let  i = i.as_ref();
    i.circom_compile().unwrap();
    i.circom_prove().unwrap();
}

pub struct SUM<H: ElementHasher,const N:usize> {
    options: WinterCircomProofOptions<N>,
    circuit_name: String,
    logging_level: LoggingLevel,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher,const N: usize> SUM<H,N> {
    pub fn new(circuit_name: String, logging_level:LoggingLevel, options: WinterCircomProofOptions<N>) -> Self {

        SUM {
            options,
            circuit_name,
            logging_level,
            _hasher: PhantomData,
        }
    }
}

impl<H: ElementHasher,const N: usize> SNP for SUM<H,N>
    where
        H: ElementHasher<BaseField = BaseElement>,
{
    fn circom_verify(&self) -> Result<(), WinterCircomError> {
        todo!()
    }

    fn circom_prove(&self) -> Result<(), WinterCircomError>{
        let start = BaseElement::ONE;

        // build proof
        let options = self.options.clone().get_proof_options();
        let prover = WorkProver::<H>::new(options);
        let trace = prover.build_trace(start, self.options.clone().trace_length);

        circom_prove(prover, trace,self.circuit_name.clone().as_str(), self.logging_level.clone())
    }

    fn circom_compile(&self) -> Result<(), WinterCircomError>{
        circom_compile::<WorkProver<H>, N>(self.options.clone(), self.circuit_name.clone().as_str(), self.logging_level.clone())
    }
}

pub fn get_instance()->Result<Box<dyn SNP>, String>{
    Ok(Box::new(SUM::<Poseidon, 2>::new(
        String::from("sum"),
        LoggingLevel::Default,
        PROOF_OPTIONS
    )))
}