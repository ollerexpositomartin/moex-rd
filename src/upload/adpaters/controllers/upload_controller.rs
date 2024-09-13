use std::{error::Error, sync::Arc};

use crate::share::models::Torrent;
use crate::upload::ports::drivens::UploadDriven;
use crate::upload::ports::drivers::UploadDriver;
use async_trait::async_trait;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};


pub struct UploadController {
    service: Box<dyn UploadDriven>
}

#[async_trait]
impl UploadDriver for UploadController {
    async fn upload(&self) -> Result<HttpResponse, Box<dyn Error>> {
        println!("HOLA");
        //self.service.upload(torrent);
        return Ok(HttpResponse::Ok().body("Ok"));
    }
}

impl UploadController {
    pub fn new(service: Box<dyn UploadDriven>) -> Box<dyn UploadDriver> {
       Box::new(Self{service: service})
    }

    pub async fn upload(controller: web::Data<Arc<UploadController>>) -> Result<HttpResponse, Box<dyn Error>> {
        controller.upload();
        return Ok(HttpResponse::Ok().body("Ok"));
    }
}
