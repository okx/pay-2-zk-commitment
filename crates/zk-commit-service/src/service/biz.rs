use std::{fs::File, io::Read};

use crate::{
    integration::rapidsnark::RapidSnark, types::{C, D, F}, AppState
};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use plonky2::plonk::{circuit_data::{CommonCircuitData, VerifierOnlyCircuitData}, proof::ProofWithPublicInputs};
use serde::{Deserialize, Serialize};
use zk_commit_core::groth16::Groth16ProofWithPublicData;



use crate::error::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileProofData{
    pub proof_with_pis: ProofWithPublicInputs<F, C, D>,
    pub verifier_only_data: VerifierOnlyCircuitData<C, D>,
    pub common_circuit_data: CommonCircuitData<F, D>
}

pub(crate) fn wrap_groth16(
    proof_file: TempFile,
    _app_state: Data<AppState>,
) -> Result<Groth16ProofWithPublicData, ServiceError> {
    println!("receive  wrap_groth16 request with file");

    let path = format!("./tmp/{}", proof_file.file_name.unwrap());
    log::info!("saving to {path}");
    proof_file.file.persist(path.clone()).unwrap();

    let mut file = File::open(path).expect("Cannot read file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Cannot read file");

    // Deserialize the binary data to a struct
    let mobile_proof_with_pis: MobileProofData = bincode::deserialize(&buffer).unwrap();

    // Wrap the mobile proof in a bn128 plonky proof


    let proof_with_pis_json_string = serde_json::to_string(&mobile_proof_with_pis).unwrap();

    let rapidsnark = RapidSnark::new("http://localhost:3000");
    let rapidsnark_proof = rapidsnark.request_proof_sync(proof_with_pis_json_string);
    let pwi: Groth16ProofWithPublicData = rapidsnark_proof.into();

    Ok(pwi)
}
