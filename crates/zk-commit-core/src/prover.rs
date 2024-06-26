#![feature(generic_const_exprs)]

use anyhow::{anyhow, Result};
use log::Level;
use plonky2::{
    field::{
        extension::Extendable,
        types::{Field, PrimeField64, Sample},
    },
    gates::noop::NoopGate,
    hash::hash_types::RichField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{
            CircuitConfig, CircuitData, CommonCircuitData, VerifierCircuitTarget,
            VerifierOnlyCircuitData,
        },
        config::{AlgebraicHasher, GenericConfig},
        proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget},
        prover::prove,
    },
    util::timing::TimingTree,
};
use plonky2_field::goldilocks_field::GoldilocksField;

use crate::{
    circuit_config::STANDARD_CONFIG,
    claim_circuit::{generate_claim_circuit, set_claim_circuit},
    claim_execution::{get_claim_proving_inputs, Claim},
    commitment_tree::CommitmentTree,
    utils::AmountSecretPairing,
};

/// Given a distribution, builds the commitment tree and returns the commitment tree.
pub fn setup_commitment(
    distribution: Vec<AmountSecretPairing>,
) -> CommitmentTree {
    let commitment_tree = CommitmentTree::new_from_distribution(&distribution);
    commitment_tree
}

fn to_any(pw: &PartialWitness::<GoldilocksField>) -> &dyn std::any::Any {
    pw
}

/// Generates the proof of a given claim of a provided amount and secret at a specific index in a commitment tree.
/// Will panic if proof is not valid. Otherwise will return a success
pub fn generate_proof_of_claim<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    InnerC: GenericConfig<D, F = F>,
    const D: usize,
>(
    amount: F,
    secret: F,
    index: usize,
    commitment_tree: CommitmentTree,
    _path: &str,
) -> Result<(ProofWithPublicInputs<F, C, D>, VerifierOnlyCircuitData<C, D>, CommonCircuitData<F, D>)>
{
    // Create claim from inputs
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
    let CircuitData { prover_only, common, verifier_only: _ } = &data;
    let pw = to_any(&pw);
    let pw = pw.downcast_ref::<PartialWitness::<F>>().unwrap();
    let proof_res = prove(&prover_only, &common, pw.clone(), &mut timing);

    // If proof failed then return error
    if proof_res.is_err() {
        return Err(anyhow!("Proof failed"));
    }

    let proof = proof_res.expect("Proof failed");
    Ok((proof, data.verifier_only, data.common))
    // // Verify proof
    // let proof_verification_res = data.verify(proof.clone());

    // // If proof verification failed then return error
    // if proof_verification_res.is_err() {
    //     return Err(anyhow!("Proof verification failed"));
    // }

    // let write_res = write_to_file(path, proof);
    // if write_res.is_err(){
    //     return Err(anyhow!("Unable to write to file"));
    // }

    // let write_res = write_to_file(path, proof);
    // if write_res.is_err() {
    //     return Err(anyhow!("Unable to write to file"));
    // }

    // return Result::Ok(());
}

pub fn recursive_single_proof<
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
    is_bn254: bool,
) -> Result<(ProofWithPublicInputs<F, C, D>, VerifierOnlyCircuitData<C, D>, CommonCircuitData<F, D>)>
where
    InnerC::Hasher: AlgebraicHasher<F>,
    // [(); C::Hasher::HASH_SIZE]:,
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
    println!("set circuit digest {:?}", inner_vd.circuit_digest);
    pw.set_hash_target(inner_data.circuit_digest, inner_vd.circuit_digest);

    // Register all public inputs
    println!(
        "register public inputs: {:?}, size: {:?}",
        pt.public_inputs.as_slice(),
        pt.public_inputs.as_slice().len()
    );
    builder.register_public_inputs(pt.public_inputs.as_slice());

    // we do not need to register circuit digest on last recursive layer; as it is hard code as a constant in the circom constant file

    println!("register circuit_digest: {:?}", inner_data.circuit_digest.elements.as_slice());
    builder.register_public_inputs(inner_data.circuit_digest.elements.as_slice());

    // if is_bn254 {
    //     let mut configs = vec![];
    //     for i in 0..builder.config.fri_config.num_cap_elements() {
    //         // let sigma_hash = builder.hash_n_to_hash_no_pad(inner_data.constants_sigmas_cap.0[i].elements);
    //         configs.append(&mut inner_data.constants_sigmas_cap.0[i].elements.to_owned().to_vec());
    //     }
    //     let hash = builder.hash_n_to_hash_no_pad::<PoseidonBN128Hash>(configs);
    //     println!("register fri hash: {:?}", hash.elements);
    //     builder.register_public_inputs(&hash.elements);
    // }

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
    println!("number of gates: {:?}", builder.num_gates());
    let data = builder.build::<C>();

    if is_bn254 {
        println!("######################### bn254 recursive prove");
    }

    let mut timing = TimingTree::new(
        if is_bn254 { "recursive_single_proof_bn254" } else { "recursive_single_proof_gl64" },
        Level::Debug,
    );
    let start = std::time::Instant::now();
    let proof = prove(&data.prover_only, &data.common, pw, &mut timing)?;
    println!("proof used: {:?}", start.elapsed());
    println!("######################### recursive verify #########################");
    data.verify(proof.clone())?;
    println!("recursive signle proof time: {:?}", now.elapsed());
    Ok((proof, data.verifier_only, data.common))
}

/// Writes the proof of a claim to a specified path as a binary file
// pub fn write_to_file(path: &str, proof: ProofWithPublicInputs<F, C, D>) -> std::io::Result<()> {
//     // Serialize the struct to a binary format
//     let encoded: Vec<u8> = bincode::serialize(&proof).unwrap();

//     // Write the binary data to a file
//     let mut file = File::create(path).expect("File create error");
//     file.write_all(&encoded).expect("Error writing to file");

//     Ok(())
// }

#[cfg(test)]
mod test {

    use crate::{circuit_config::D, types::F, utils::AmountSecretPairing};
    use plonky2::field::types::Field;

    use super::setup_commitment;

    #[test]
    fn test_generate_proof_of_claim() {
        let distribution = vec![
            AmountSecretPairing { amount: F::ONE, secret: F::ZERO },
            AmountSecretPairing { amount: F::ONE, secret: F::ONE },
            AmountSecretPairing { amount: F::ONE, secret: F::TWO },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(3) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(4) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(5) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(6) },
            AmountSecretPairing { amount: F::ONE, secret: F::from_canonical_u64(7) },
        ];

        let _commitment_tree = setup_commitment(distribution.clone());

        // let claim_proof = generate_proof_of_claim(
        //     distribution.get(0).unwrap().amount,
        //     distribution.get(0).unwrap().secret,
        //     0,
        //     commitment_tree,
        //     "test.bin",
        // );

        // assert!(claim_proof.is_ok());

        // let mut file = File::open("test.bin").expect("Cannot read file");
        // let mut buffer = Vec::new();
        // file.read_to_end(&mut buffer).expect("Cannot read file");

        // // Deserialize the binary data to a struct
        // let decoded: ProofWithPublicInputs<F, C, D> = bincode::deserialize(&buffer).unwrap();

        // println!("{:?}", decoded);
    }
}
