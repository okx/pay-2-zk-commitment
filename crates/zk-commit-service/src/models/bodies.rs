use serde::{Deserialize, Serialize};

use crate::integration::rapidsnark::Groth16ProofWithPublicData;

#[derive(Deserialize, Serialize, Debug)]
pub struct WrapGroth16ReqBody {
    pub plonky2_proof: String,
}

#[derive(Deserialize, Serialize)]
pub struct WrapGroth16ResBody {
    pub groth16_proof: Groth16ProofWithPublicData,
}
