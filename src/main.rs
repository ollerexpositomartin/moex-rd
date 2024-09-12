mod share;
mod upload;

use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use upload::{adpaters::{controllers::UploadController, repositories::rd_repository::RdRepository}, ports::drivers::UploadDriver};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uploadController = Arc::new(UploadController::new(RdRepository::new()));

    HttpServer::new(|| {
        App::new()
        .service(web::resource("/upload").route(web::post().to(move |json| uploadController.upload(json))))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
