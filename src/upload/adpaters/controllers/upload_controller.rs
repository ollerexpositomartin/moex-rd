use std::{error::Error, sync::Arc};

use crate::share::models::Torrent;
use crate::upload::ports::drivens::UploadDriven;
use crate::upload::ports::drivers::UploadDriver;
use actix_web::{get, http::StatusCode, web, HttpResponse};
use async_trait::async_trait;


pub struct UploadController {
    service: Arc<Box<dyn UploadDriven>>,
}

#[async_trait]
impl UploadDriver for UploadController {
     async fn upload(&self, torrent: web::Json<Torrent>) -> Result<HttpResponse, Box<dyn Error>>{
        let future = async {
            // Simula una operación asíncrona
            42
        };
        future.await;
        //self.service.upload(torrent);
        return Ok(HttpResponse::Ok().body("Hello world!"));
    }
}

impl UploadController {
    pub fn new(service: Box<dyn UploadDriven>) -> Box<dyn UploadDriver>{
       Box::new(Self{service: Arc::new(service)})
    }
}
