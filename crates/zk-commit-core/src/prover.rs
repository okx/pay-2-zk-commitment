use std::fs::File;

use anyhow::anyhow;
use log::Level;
use plonky2::{
    iop::witness::PartialWitness,
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitData, proof::ProofWithPublicInputs,
        prover::prove,
    },
    util::timing::TimingTree,
};

use bincode;
use std::io::Write;

use crate::{
    circuit_config::{D, STANDARD_CONFIG},
    claim_circuit::{generate_claim_circuit, set_claim_circuit},
    claim_execution::{get_claim_proving_inputs, Claim},
    commitment_tree::CommitmentTree,
    types::{C, F},
    utils::AmountSecretPairing,
};


/// Given a distribution, builds the commitment tree and returns the commitment tree.
pub fn setup_commitment(distribution: Vec<AmountSecretPairing>) -> CommitmentTree {
    let commitment_tree = CommitmentTree::new_from_distribution(&distribution);
    commitment_tree
}

/// Generates the proof of a given claim of a provided amount and secret at a specific index in a commitment tree.
/// Will panic if proof is not valid. Otherwise will return a success
pub fn generate_proof_of_claim(
    amount: F,
    secret: F,
    index: usize,
    commitment_tree: CommitmentTree,
    path: &str
) -> Result<(), anyhow::Error> {
    // Create claim from inputs
    let claim = Claim {
        pair: AmountSecretPairing { amount, secret },
        commitment: commitment_tree.get_root(),
        commitment_merkle_proof: commitment_tree.get_siblings(index),
        index,
    };

    // Execute claim to get the PIs
    let claim_proving_inputs = get_claim_proving_inputs(claim);

    // Create claim circuit and set the PIs
    let mut builder = CircuitBuilder::<F, D>::new(STANDARD_CONFIG);
    let mut pw: PartialWitness<F> = PartialWitness::<F>::new();

    let claim_targets = generate_claim_circuit(&mut builder, commitment_tree.depth);
    set_claim_circuit(claim_targets, claim_proving_inputs, &mut pw);

    // Build circuit from data and prove
    builder.print_gate_counts(0);
    let mut timing = TimingTree::new("prove", Level::Debug);
    let data = builder.build::<C>();
    let CircuitData { prover_only, common, verifier_only: _ } = &data;
    let proof_res = prove(&prover_only, &common, pw, &mut timing);

    // If proof failed then return error
    if proof_res.is_err() {
        return Err(anyhow!("Proof failed"));
    }

    let proof = proof_res.expect("Proof failed");

    // Verify proof
    let proof_verification_res = data.verify(proof.clone());

    // If proof verification failed then return error
    if proof_verification_res.is_err() {
        return Err(anyhow!("Proof verification failed"));
    }

    let write_res = write_to_file(path, proof);
    if write_res.is_err(){
        return Err(anyhow!("Unable to write to file"));
    }

    return Result::Ok(());
}

/// Writes the proof of a claim to a specified path as a binary file
pub fn write_to_file(path: &str, proof: ProofWithPublicInputs<F,C, D>)-> std::io::Result<()>{
     // Serialize the struct to a binary format
     let encoded: Vec<u8> = bincode::serialize(&proof).unwrap();

     // Write the binary data to a file
     let mut file = File::create(path).expect("File create error");
     file.write_all(&encoded).expect("Error writing to file");
 
     Ok(())
}


#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use crate::{circuit_config::D, types::{C, F}, utils::AmountSecretPairing};
    use plonky2::{field::types::Field, plonk::proof::ProofWithPublicInputs};

    use super::{generate_proof_of_claim, setup_commitment};

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
        
        let commitment_tree = setup_commitment(distribution.clone());

        let claim_proof = generate_proof_of_claim(distribution.get(0).unwrap().amount, distribution.get(0).unwrap().secret, 0, commitment_tree, "test.bin");

        assert!(claim_proof.is_ok());

        let mut file = File::open("test.bin").expect("Cannot read file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Cannot read file");
    
        // Deserialize the binary data to a struct
        let decoded: ProofWithPublicInputs<F,C,D> = bincode::deserialize(&buffer).unwrap();
    
        println!("{:?}", decoded);
    }
}
