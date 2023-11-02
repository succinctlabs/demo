# Succinct Alpha Demo: Consensus Oracle

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

## Motivation

Once [EIP-4788](https://eips.ethereum.org/EIPS/eip-4788) is implemented, all smart contracts on the Ethereum Mainnet will gain access to the beacon block root. This means that smart contracts will now be able to access all the information stored in the consensus layer, including validator balances and participation rates. Presently, numerous liquid staking protocols, such as Lido, rely on centralized oracles to transmit this information to the Ethereum Mainnet.

With access to the block root, protocols can trustlessly retrieve this data using Merkle Proofs. However, calculating many of the statistics that protocols require would still be prohibitively costly to verify on the Ethereum Virtual Machine (EVM). For instance, Lido employs us to compute the sum of up to 1 million validator balances, a task that would necessitate over 4 million calls to SHA-256 for verification on the EVM.

Rather than executing all computations in the EVM, we can leverage zero-knowledge proofs to condense the computation into a proof verification, which only consumes approximately 250,000 gas. Although this approach has been theoretically viable for some time, several steps are necessary to transition such an application to production. These steps include running the prover, transmitting the proof, and monitoring the system. Succinct's platform aims to simplify this process for developers, allowing them to deploy applications without the burden of managing their own infrastructure.

## Source Code

This circuit was built using [plonky2x]() which comes built-in with support for all Ethereum types 
(i.e., bytes32, uint256) and helper methods for accessing Ethereum data.

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