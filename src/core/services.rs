
use actix_web::{get, post, put, delete,middleware, Error, web::{self, Json}, Responder, HttpResponse};
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm
    },
    Multipart
};


use futures_util::TryStreamExt as _;
//use uuid::Uuid;

use crate::{AppState};
use super::models::{TransferRequest, UploadForm, ApiResponse};


#[post("/bulktransfer")]
async fn do_bulk_transfer(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    let response = ApiResponse::success();

    Ok(web::Json(response))
    //Ok(HttpResponse::Ok())
}

#[post("createsubscription")]
async fn create_subscription(){
    
}

//create serviceConfig
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(do_bulk_transfer)
    ;
}