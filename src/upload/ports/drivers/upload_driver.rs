use std::error::Error;

use actix_web::{web, HttpResponse, Responder, Result};
use async_trait::async_trait;

use crate::share::models::Torrent;

#[async_trait]
pub trait UploadDriver: Send + Sync {
   async fn upload(&self, torrent:Torrent) -> Result<HttpResponse, Box<dyn Error>>;
}
