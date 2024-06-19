use log::Level;
use plonky2::{iop::witness::PartialWitness, plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitData, prover::prove}, util::timing::TimingTree};

use crate::{circuit_config::{D, STANDARD_CONFIG}, claim_circuit::{generate_claim_circuit, set_claim_circuit}, claim_execution::{execute_claim, Claim}, commitment_tree::{self, CommitmentTree}, types::{C, F}, utils::AmountSecretPairing};

pub fn setup_commitment(distribution: Vec<AmountSecretPairing>)->CommitmentTree{
    let commitment_tree = CommitmentTree::new_from_distribution(&distribution);
    commitment_tree
}


/// Generates the proof of a given claim of a provided amount and secret at a specific index in a commitment tree. 
/// Will panic if proof is not valid
pub fn generate_proof_of_claim(amount: F, secret: F, index: usize, commitment_tree: CommitmentTree)-> Result<(), anyhow::Error>{
    let claim = Claim{
        pair: AmountSecretPairing{amount, secret},
        commitment: commitment_tree.get_root(),
        commitment_merkle_proof: commitment_tree.get_siblings(index),
        index
    };

    let claim_proving_inputs = execute_claim(claim);

    let mut builder = CircuitBuilder::<F, D>::new(STANDARD_CONFIG);
    let mut pw: PartialWitness<F> = PartialWitness::<F>::new();

    let claim_targets = generate_claim_circuit(&mut builder, commitment_tree.depth);
    // println!("{:?}", commitment_tree.depth);
    set_claim_circuit(claim_targets, claim_proving_inputs, &mut pw); 

    builder.print_gate_counts(0);
    let mut timing = TimingTree::new("prove", Level::Debug);
    let data = builder.build::<C>();
    let CircuitData {
        prover_only,
        common,
        verifier_only: _,
    } = &data;
    let proof = prove(&prover_only, &common, pw, &mut timing).expect("Prove fail");
    timing.print();
    data.verify(proof)
}