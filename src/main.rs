use std::io::Write;
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm
    },
    Multipart
};

mod core;
use crate::core::services;


use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
//use futures_util::TryStreamExt as _;
//use uuid::Uuid;

struct AppState{

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("creating temporary upload directory");

    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .app_data(TempFileConfig::default().directory("./tmp"))
        .configure(services::config)
    })
    .bind(("127.0.0.1", 8081))?
    .workers(4)
    .run()
    .await

}
