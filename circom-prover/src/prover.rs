use super::air::{PublicInputs, WorkAir};
use winter_air::ProofOptions;
use winter_math::{fields::f256::BaseElement, FieldElement};
use winter_prover::{Prover, Trace, TraceTable};

pub struct WorkProver {
    options: ProofOptions,
}

impl WorkProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    pub fn build_trace(&self, start: BaseElement, n: usize) -> TraceTable<BaseElement> {
        let trace_width = 2;
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

    pub const NUM_PUB_INPUTS : usize = 2;

}

impl Prover for WorkProver {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<Self::BaseField>;

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
