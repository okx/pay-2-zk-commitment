use anyhow::Result;
use log::Level;
use plonky2::{
    field::extension::Extendable,
    gates::noop::NoopGate,
    hash::hash_types::RichField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{
            CircuitConfig, CircuitData, CommonCircuitData, VerifierCircuitTarget,
            VerifierOnlyCircuitData,
        },
        config::{AlgebraicHasher, GenericConfig, Hasher},
        proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget},
        prover::prove,
    },
    util::timing::TimingTree,
};
use zk_commit_core::circuits::claim_circuit::generate_claim_circuit;

use crate::types::{C, D, F};

use super::circuit_config::STANDARD_CONFIG;

/// Gets the circuit data of the mobile proving circuit for the claimant
pub fn get_circuit_data(merkle_tree_depth: usize) -> CircuitData<F, C, D> {
    let mut builder: CircuitBuilder<F, D> = CircuitBuilder::new(STANDARD_CONFIG);
    let _claim_targets = generate_claim_circuit(&mut builder, merkle_tree_depth);
    let data = builder.build::<C>();
    data
}

/// A plonky2 proof in the bn128 field.
pub fn bn128_proof<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    InnerC: GenericConfig<D, F = F>,
    const D: usize,
>(
    inner_proof: &ProofWithPublicInputs<F, InnerC, D>,
    inner_vd: &VerifierOnlyCircuitData<InnerC, D>,
    inner_cd: &CommonCircuitData<F, D>,
    config: &CircuitConfig,
    min_degree_bits: Option<usize>,
    print_gate_counts: bool,
) -> Result<(ProofWithPublicInputs<F, C, D>, VerifierOnlyCircuitData<C, D>, CommonCircuitData<F, D>)>
where
    InnerC::Hasher: AlgebraicHasher<F>,
    [(); C::Hasher::HASH_SIZE]:,
{
    let now = std::time::Instant::now();

    let mut builder = CircuitBuilder::<F, D>::new(config.clone());
    let mut pw = PartialWitness::new();
    let pt: ProofWithPublicInputsTarget<D> =
        builder.add_virtual_proof_with_pis::<InnerC>(&inner_cd);

    pw.set_proof_with_pis_target(&pt, &inner_proof);

    let inner_data = VerifierCircuitTarget {
        constants_sigmas_cap: builder.add_virtual_cap(inner_cd.config.fri_config.cap_height),
        circuit_digest: builder.add_virtual_hash(),
    };

    pw.set_cap_target(&inner_data.constants_sigmas_cap, &inner_vd.constants_sigmas_cap);
    pw.set_hash_target(inner_data.circuit_digest, inner_vd.circuit_digest);

    // Register all public inputs
    builder.register_public_inputs(pt.public_inputs.as_slice());
    builder.register_public_inputs(inner_data.circuit_digest.elements.as_slice());

    builder.verify_proof::<InnerC>(&pt, &inner_data, &inner_cd);

    if print_gate_counts {
        builder.print_gate_counts(0);
    }

    if let Some(min_degree_bits) = min_degree_bits {
        // We don't want to pad all the way up to 2^min_degree_bits, as the builder will add a
        // few special gates afterward. So just pad to 2^(min_degree_bits - 1) + 1. Then the
        // builder will pad to the next power of two, 2^min_degree_bits.
        let min_gates = (1 << (min_degree_bits - 1)) + 1;
        for _ in builder.num_gates()..min_gates {
            builder.add_gate(NoopGate, vec![]);
        }
    }

    let data = builder.build::<C>();

    println!("######################### bn254 recursive prove");

    let mut timing = TimingTree::new("recursive_single_proof_bn254", Level::Debug);
    let proof = prove(&data.prover_only, &data.common, pw, &mut timing)?;

    println!("######################### recursive verify #########################");
    data.verify(proof.clone())?;
    println!("recursive signle proof time: {:?}", now.elapsed());
    Ok((proof, data.verifier_only, data.common))
}

#[cfg(test)]
mod test {

    use std::{fs::File, io::Read};

    use plonky2::{field::goldilocks_field::GoldilocksField, plonk::circuit_data::CircuitData};
    use zk_commit_core::{
        prover::{generate_proof_of_claim, setup_commitment, MobileProofData},
        utils::AmountSecretPairing,
    };

    use crate::{
        integration::circuits::{circuit_config::HIGH_RATE_CONFIG, recursive::get_circuit_data},
        types::{Cbn128, C, D, F},
    };
    use plonky2::field::types::Field;

    use super::bn128_proof;

    #[test]
    fn test_bn128_proof() {
        let mut distribution = Vec::new();

        for i in 0..32 {
            let pair = AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(i) };

            distribution.push(pair);
        }

        let commitment_tree = setup_commitment(distribution.clone());

        let _ = generate_proof_of_claim(
            distribution.get(0).unwrap().amount,
            distribution.get(0).unwrap().secret,
            0,
            commitment_tree,
            "test.bin",
        );

        let mut file = File::open("test.bin").expect("Cannot read file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Cannot read file");

        // Deserialize the binary data to a struct
        let decoded: MobileProofData = bincode::deserialize(&buffer).unwrap();

        let data = get_circuit_data(decoded.merkle_depth);
        let CircuitData { prover_only: _, common, verifier_only } = &data;

        println!("decoded proof: {:?}", decoded.proof_with_pis);

        let _ = bn128_proof::<GoldilocksField, Cbn128, C, D>(
            &decoded.proof_with_pis,
            verifier_only,
            common,
            &HIGH_RATE_CONFIG,
            None,
            true,
        );
    }
}
