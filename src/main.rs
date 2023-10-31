use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::Plonky2xFunction;
use plonky2x::prelude::{Bytes32Variable, CircuitBuilder, U64Variable};

#[derive(Debug, Clone)]
struct ValidatorBalanceOracle;

impl Circuit for ValidatorBalanceOracle {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let block_root = builder.evm_read::<Bytes32Variable>();
        let balances = builder.beacon_get_balances(block_root);

        let mut sum = builder.zero::<U64Variable>();
        for i in 0..4 {
            let idx = builder.constant::<U64Variable>(i);
            let balance = builder.beacon_get_balance(balances, idx);
            sum = builder.add(sum, balance);
        }

        builder.evm_write::<U64Variable>(sum);
    }
}

fn main() {
    ValidatorBalanceOracle::entrypoint();
}

#[cfg(test)]
mod tests {
    use std::env;

    use plonky2x::prelude::DefaultParameters;
    use plonky2x::utils::bytes32;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    const EXAMPLE_BLOCK_ROOT: &str =
        "0xc366a826d730e5e8767a3b13f81cbf8ffa10a269272232b9243337637aef6dc7";

    #[test]
    fn test_circuit() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();
        env::set_var("CONSENSUS_RPC_1", "https://beaconapi.succinct.xyz");

        // Build the circuit.
        let mut builder = CircuitBuilder::<L, D>::new();
        ValidatorBalanceOracle::define(&mut builder);
        let circuit = builder.build();

        // Generate input.
        let mut input = circuit.input();
        input.evm_write::<Bytes32Variable>(bytes32!(EXAMPLE_BLOCK_ROOT));

        // Generate the proof and verify.
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        // Read output.
        let output = output.evm_read::<U64Variable>();
        println!("{}", output);

        // Test circuit serialization.
        ValidatorBalanceOracle::test_serialization::<L, D>();
    }
}
