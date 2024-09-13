mod share;
mod upload;

use std::error::Error;
use std::sync::Arc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use share::models::Torrent;
use upload::{adpaters::{controllers::UploadController, repositories::rd_repository::{self, RdRepository}}, ports::drivers::UploadDriver};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let u = Arc::new(UploadController::new(RdRepository::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(u.clone()))
        .service(
            web::resource("/upload")
                .route(web::post().to(UploadController::upload)), // Define el manejador
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn upload() -> Result<HttpResponse, Box<dyn Error>> {
    return Ok((HttpResponse::Ok().body("test")));
}
