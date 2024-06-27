use anyhow::Result;
use plonky2::{field::extension::Extendable, hash::hash_types::RichField, plonk::{circuit_data::{CircuitConfig, CommonCircuitData, VerifierOnlyCircuitData}, config::{AlgebraicHasher, GenericConfig}, proof::ProofWithPublicInputs}};


pub fn recursive_last_single_proof<
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

    // verify vd proofs
    let vd_proof_verification_targets = verify_vd_proof(&mut builder, vd_proof);

    // set vd targets
    pw.set_hash_target(vd_proof_verification_targets.vd_root_target, vd_proof.root);
    pw.set_target(vd_proof_verification_targets.vd_index_target, vd_proof.index);

    for (vd_target, vd_element) in vd_proof_verification_targets.vd_digest_target.iter().zip(
        [
            inner_vd.constants_sigmas_cap.flatten().as_slice(),
            inner_vd.circuit_digest.to_vec().as_slice(),
        ]
        .concat(),
    ) {
        pw.set_target(vd_target.clone(), vd_element);
    }
    for i in 0..vd_proof.merkle_proof.siblings.len() {
        pw.set_hash_target(
            vd_proof_verification_targets.vd_proof_target.siblings[i],
            vd_proof.merkle_proof.siblings[i],
        );
    }

    // Register all public inputs
    builder.register_public_inputs(pt.public_inputs.as_slice());
    // register vd root as public input
    builder
        .register_public_inputs(vd_proof_verification_targets.vd_root_target.elements.as_slice());
    builder.register_public_inputs(inner_data.circuit_digest.elements.as_slice());
    for i in 0..builder.config.fri_config.num_cap_elements() {
        builder.register_public_inputs(&inner_data.constants_sigmas_cap.0[i].elements);
    }
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
