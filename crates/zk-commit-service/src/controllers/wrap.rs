use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, Responder};

use crate::{error::ServiceError, service::biz, AppState};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

/// Wraps a plonky2 proof in a groth 16 proof to reduce the size of the proof.
#[post("/groth16")]
pub async fn wrap_groth16(
    MultipartForm(file): MultipartForm<UploadForm>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    println!("receive wrap request");
    let ret = biz::wrap_groth16(file, app_state);
    match ret {
        Ok(r) => Ok(web::Json(r)),
        Err(e) => Err(e),
    }
}
