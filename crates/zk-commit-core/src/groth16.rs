use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Groth16Proof {
    pub pi_a: [String; 3],
    pub pi_b: [[String; 2]; 3],
    pub pi_c: [String; 3],
    pub protocol: String,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct Groth16ProofWithPublicData {
    pub proof: Groth16Proof,
    pub public_data: Vec<String>,
}
