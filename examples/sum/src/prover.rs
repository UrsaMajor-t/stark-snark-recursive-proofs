use std::marker::PhantomData;
use winter_circom_prover::hash::Poseidon;
use super::air::{PublicInputs, WorkAir, PROOF_OPTIONS};
use winter_circom_prover::winterfell::{Air, DefaultTraceLde, math::{fields::f256::BaseElement, FieldElement}, ProofOptions, Prover, StarkDomain, Trace, TraceLde, TraceTable};
use winter_circom_prover::winterfell::crypto::{DefaultRandomCoin, ElementHasher};
pub struct WorkProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> WorkProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    pub fn build_trace(&self, start: BaseElement, n: usize) -> TraceTable<BaseElement> {
        let trace_width = PROOF_OPTIONS.trace_width;
        assert!(
            n.is_power_of_two(),
            "length must be a power of 2"
        );
        let mut trace = TraceTable::new(trace_width, n);

        trace.fill(
            |state| {
                state[0] = start;
                state[1] = start;
            },
            |_, state| {
                state[0] += BaseElement::ONE;
                state[1] += state[0];
            },
        );

        trace
    }
}

impl<H: ElementHasher> Prover for WorkProver<H>
    where
        H: ElementHasher<BaseField=BaseElement>, {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<Self::BaseField>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField=Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let last_step = trace.length() - 1;
        PublicInputs {
            start: trace.get(0, 0),
            result: trace.get(1, last_step),
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }

}


#[test]
fn sum_trace_table(){
    let start = BaseElement::ONE;

    // build the trace and the domain
    let option = PROOF_OPTIONS.get_proof_options();
    let prover = WorkProver::<Poseidon>::new(option.clone());
    let trace = prover.build_trace(start, PROOF_OPTIONS.trace_length);
    let pub_input = prover.get_pub_inputs(&trace);
    let air  = WorkAir::new(trace.get_info(),pub_input,option);
    let domain = StarkDomain::new(&air);
    // build the trace polynomials, extended trace, and commitment using the default TraceLde impl
    let (trace_polys, trace_lde) = DefaultTraceLde::<BaseElement, Poseidon>::new(
        &trace.get_info(),
        trace.main_segment(),
        &domain,
    );
}