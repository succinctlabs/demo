# Succinct Alpha Demo

<img src="./assets/bg.png"></img>


**Welcome!** You've succesfully created the demo project. This project will demonstrate end-to-end 
usage of the platform, including:

1. Creating a release of your project or a versioned build of your circuit artifacts.
2. Deploying your verifier onchain so that your application can verify proofs.
3. Requesting a proof onchain and having the result delivered to your smart contract.

**This demo will deploy an oracle that returns the sum of the balances of the first four Ethereum 
validators.**  The entire result will be computed using zero-knowledge proofs, meaning that there
are absolutely no trust assumptions (beyond implementation bugs).

The important parts of the core logic are described below using our proving framework [plonky2x](). The platform can 
support any proving system, ranging from zkVMs to circuit libraries like Halo2, Gnark, and more. The source code attached with this project is available on [GitHub]().

## Source Code

```rust
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
```