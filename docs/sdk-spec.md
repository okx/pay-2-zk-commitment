# sdk spec
1. setup
```rust
/*
- F in this case is represented as a u64, its maximum value is 2^64-2^32.

- Hashout<F> is a hash of 256 bits represented as a [u64; 4].
*/

struct AmountSecretPair {
    amount: F,
    secret: F
}

pub struct CommitmentTree {
    depth: u64,
    commitment_root: HashOut<F>,
    commitment_tree: Vec<HashOut<F>>,
}

pub struct ClaimProofResponse {
    proof_result: Result<ProofWithPublicInputs<F, C, D>>,
    amount_claimed: F
}


/// for depositer to setup a commitment to the distribution; return the commitment tree
fn set_up(distrubutions: Vec<AmountSecretPair>) -> CommitmentTree; 

/// deposit to contract; rust sdk will compose tx and return to mobile end; mobile end will do rpc [NEXT ITERATION]
/// How do we verify the amount ios equal to the sum of the distributions? @Cliff 
fn deposit_usdc(commitment: [u8;32], amount: u64, private_key: [u8;32]) 

/// Generate the proof of the claim and return the proof along with the amount (public input) that was claimed
fn gen_plonky2_claim_proof(secret: F, amount: F, index: u64, commitment_tree: CommitmentTree) -> ClaimProofResponse; 

/// Features done in next iteration
fn request_groth16_proof(plonky2_proof: String) -> String// return groth16 proof in json string; Http client implemented by mobile; server provided by ZKP team
fn claim(groth16_proof: String, private_key: [u8;32]);// for receiver to claim, rust sdk will compose tx and return to mobile end; mobile end will do rpc