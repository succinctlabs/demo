use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::Plonky2xFunction;
use plonky2x::prelude::{Bytes32Variable, CircuitBuilder};

#[derive(Debug, Clone)]
struct Sha256;

impl Circuit for Sha256 {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let input = builder.evm_read::<Bytes32Variable>();
        let hash = builder.sha256(&input.as_bytes());
        builder.evm_write::<Bytes32Variable>(hash);
    }
}

fn main() {
    Sha256::entrypoint();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::DefaultParameters;
    use plonky2x::utils::bytes32;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    const EXAMPLE_INPUT: &str =
        "0x49869d23ba93a746cc8ea649a48bb6c4b2159cf3a71aef492af63dac27522c9f";

    const EXAMPLE_OUTPUT: &str =
        "0x34327c0f2222f180f511e1375af48b83145b50b60d558c9951defb99f2021326";

    #[test]
    fn test_circuit() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        // Build the circuit.
        let mut builder = CircuitBuilder::<L, D>::new();
        Sha256::define(&mut builder);
        let circuit = builder.build();

        // Generate input.
        let mut input = circuit.input();
        input.evm_write::<Bytes32Variable>(bytes32!(EXAMPLE_INPUT));

        // Generate the proof and verify.
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        // Read output.
        let output = output.evm_read::<Bytes32Variable>();
        let expected_output = bytes32!(EXAMPLE_OUTPUT);
        assert_eq!(output, expected_output);

        // Test circuit serialization.
        Sha256::test_serialization::<L, D>();
    }
}
