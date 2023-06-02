
use serde::{Deserialize, Serialize};
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm
    },
    Multipart
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse{
    response_code: String,
    response_message: String,
    success: bool
}
impl ApiResponse{
    pub fn success() -> ApiResponse{
        ApiResponse { response_code: "00".to_string(), response_message: "Success".to_string(), success: true }
    }
    pub fn failure() -> ApiResponse{
        ApiResponse { response_code: "02".to_string(), response_message: "Failed".to_string(), success: false }

    }
}



#[derive(Debug, Deserialize, Clone)]
pub struct TransferRequest{
    source_account: String,
    destination_account: String,
    amount: f64,
    source_bank_code: String,
    destination_bank_code: String,
    narration: String,

}

#[derive(Debug, MultipartForm)]
pub struct UploadForm{
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>
}