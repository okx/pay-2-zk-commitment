use std::{fs::File, io::Read};

use crate::{
    integration::rapidsnark::{Groth16ProofWithPublicData, RapidSnark},
    types::{C, D, F},
    AppState,
};
use actix_multipart::form::MultipartForm;
use actix_web::web::{self, Data};
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::controllers::wrap::UploadForm;

use crate::error::ServiceError;

pub(crate) fn wrap_groth16(
    file: UploadForm,
    _app_state: Data<AppState>,
) -> Result<Groth16ProofWithPublicData, ServiceError> {
    println!("receive  wrap_groth16 request with file");

    let path = format!("./tmp/{}", file.file.file_name.unwrap());
    log::info!("saving to {path}");
    file.file.file.persist(path.clone()).unwrap();

    let mut file = File::open(path).expect("Cannot read file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Cannot read file");

    // Deserialize the binary data to a struct
    let proof_with_pis: ProofWithPublicInputs<F, C, D> = bincode::deserialize(&buffer).unwrap();

    let proof_with_pis_json_string = serde_json::to_string(&proof_with_pis).unwrap();

    let rapidsnark = RapidSnark::new("http://localhost:3000");
    let rapidsnark_proof = rapidsnark.request_proof_sync(proof_with_pis_json_string);
    let pwi: Groth16ProofWithPublicData = rapidsnark_proof.into();

    Ok(pwi)
}
