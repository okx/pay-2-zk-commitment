
use log::Level;
use anyhow::anyhow;
use plonky2::{iop::witness::PartialWitness, plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitData, proof::ProofWithPublicInputs, prover::prove}, util::timing::TimingTree};

use crate::{circuit_config::{D, STANDARD_CONFIG}, claim_circuit::{generate_claim_circuit, set_claim_circuit}, claim_execution::{execute_claim, Claim}, commitment_tree::CommitmentTree, types::{C, F}, utils::AmountSecretPairing};

pub fn setup_commitment(distribution: Vec<AmountSecretPairing>)->CommitmentTree{
    let commitment_tree = CommitmentTree::new_from_distribution(&distribution);
    commitment_tree
}


/// Generates the proof of a given claim of a provided amount and secret at a specific index in a commitment tree. 
/// Will panic if proof is not valid. Otherwise will return a success 
pub fn generate_proof_of_claim(amount: F, secret: F, index: usize, commitment_tree: CommitmentTree)->Result<(ProofWithPublicInputs<F,C,D>, F), anyhow::Error>{
    // Create claim from inputs
    let claim = Claim{
        pair: AmountSecretPairing{amount, secret},
        commitment: commitment_tree.get_root(),
        commitment_merkle_proof: commitment_tree.get_siblings(index),
        index
    };

    // Execute claim to get the PIs 
    let claim_proving_inputs = execute_claim(claim);

    // Create claim circuit and set the PIs
    let mut builder = CircuitBuilder::<F, D>::new(STANDARD_CONFIG);
    let mut pw: PartialWitness<F> = PartialWitness::<F>::new();

    let claim_targets = generate_claim_circuit(&mut builder, commitment_tree.depth);    
    set_claim_circuit(claim_targets, claim_proving_inputs, &mut pw); 

    // Vuild circuit from data and prove
    builder.print_gate_counts(0);
    let mut timing = TimingTree::new("prove", Level::Debug);
    let data = builder.build::<C>();
    let CircuitData {
        prover_only,
        common,
        verifier_only: _,
    } = &data;
    let proof_res = prove(&prover_only, &common, pw, &mut timing);

    // If proof failed then return error
    if proof_res.is_err(){
        return Err(anyhow!("Proof failed"));
    }

    let proof = proof_res.expect("Proof failed");

    // Verify proof
    let proof_verification_res = data.verify(proof.clone());

    // If proof verification failed then return error
    if proof_verification_res.is_err(){
        return Err(anyhow!("Proof verification failed"));
    }

    return Result::Ok((proof, amount));
}