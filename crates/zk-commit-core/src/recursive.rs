use log::Level;
use anyhow::Result;
use plonky2::{field::extension::Extendable, gates::noop::NoopGate, hash::hash_types::RichField, iop::witness::{PartialWitness, WitnessWrite}, plonk::{circuit_builder::CircuitBuilder, circuit_data::{CircuitConfig, CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData}, config::{AlgebraicHasher, GenericConfig, GenericHashOut}, proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget}, prover::prove}};
use plonky2::util::timing::TimingTree;
use plonky2::plonk::config::Hasher;
use anyhow::anyhow;


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

    // if print_gate_counts {
    //     builder.print_gate_counts(0);
    // }

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
    
    use log::Level;
    use plonky2::{field::goldilocks_field::GoldilocksField, iop::witness::PartialWitness, plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitData, prover::prove}, util::timing::TimingTree};
    use plonky2_field::types::{Field, PrimeField64};

    use crate::{circuits::{circuit_config::{D, HIGH_RATE_CONFIG, STANDARD_CONFIG}, claim_circuit::{generate_claim_circuit, set_claim_circuit}}, claim_execution::{get_claim_proving_inputs, Claim}, commitment_tree, prover::{setup_commitment, to_any, write_to_file, MobileProofData}, types::{Cbn128, C, F}, utils::AmountSecretPairing};

    use super::bn128_proof;

    #[test]
    fn test_bn128_proof() {

        let mut distribution = Vec::new();

        for i in 0..8{
            let pair = AmountSecretPairing{
                amount: F::ONE,
                secret: F::from_canonical_u64(i)
            };

            distribution.push(pair);
        }

        let commitment_tree = setup_commitment(distribution.clone());

        let amount = distribution.get(0).unwrap().amount;
        let secret = distribution.get(0).unwrap().secret;
        let index = 0;

        let claim = Claim {
            pair: AmountSecretPairing {
                amount: GoldilocksField::from_canonical_u64(amount.to_canonical_u64()),
                secret: GoldilocksField::from_canonical_u64(secret.to_canonical_u64()),
            },
            commitment: commitment_tree.get_root(),
            commitment_merkle_proof: commitment_tree.get_siblings(index),
            index,
        };
    
        // Execute claim to get the PIs
        let claim_proving_inputs = get_claim_proving_inputs(claim);
    
        // Create claim circuit and set the PIs
        let mut builder = CircuitBuilder::<F, D>::new(STANDARD_CONFIG);
        let mut pw = PartialWitness::<GoldilocksField>::new();
    
        let claim_targets = generate_claim_circuit(&mut builder, commitment_tree.depth);
        set_claim_circuit(claim_targets, claim_proving_inputs, &mut pw);
    
        // Build circuit from data and prove
        builder.print_gate_counts(0);
        let mut timing = TimingTree::new("prove", Level::Debug);
        let data = builder.build::<C>();
        let CircuitData { prover_only, common, verifier_only } = &data;
        let pw = to_any(&pw);
        let pw = pw.downcast_ref::<PartialWitness<F>>().unwrap();
        let proof_res = prove(&prover_only, &common, pw.clone(), &mut timing);
    
        let proof = proof_res.expect("Proof failed");
        // Verify proof
        let proof_verification_res = data.verify(proof.clone());
    


        let bn_128_proof = bn128_proof::<GoldilocksField, Cbn128, C, D>(
            &proof, 
            &verifier_only, 
            &common, 
            &HIGH_RATE_CONFIG, 
            None, true
        );
    }
}

